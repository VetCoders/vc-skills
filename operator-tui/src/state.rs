use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ControlPlaneState {
    pub root: PathBuf,
    pub runs: Vec<RunSnapshot>,
    pub events: Vec<RunEvent>,
}

impl ControlPlaneState {
    pub fn load(root: impl AsRef<Path>) -> io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        let runs = load_runs(&root)?;
        let events = load_events(&root)?;
        Ok(Self { root, runs, events })
    }

    pub fn empty(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            runs: Vec::new(),
            events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunKind {
    Active,
    Recent,
    Completed,
    Failed,
    Stalled,
    Paused,
    Unknown,
}

impl RunKind {
    pub fn label(self) -> &'static str {
        match self {
            RunKind::Active => "active",
            RunKind::Recent => "recent",
            RunKind::Completed => "completed",
            RunKind::Failed => "failed",
            RunKind::Stalled => "stalled",
            RunKind::Paused => "paused",
            RunKind::Unknown => "unknown",
        }
    }

    pub fn sort_rank(self) -> u8 {
        match self {
            RunKind::Active => 0,
            RunKind::Stalled => 1,
            RunKind::Failed => 2,
            RunKind::Paused => 3,
            RunKind::Recent => 4,
            RunKind::Completed => 5,
            RunKind::Unknown => 6,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunSnapshot {
    #[serde(alias = "runId")]
    pub run_id: String,
    #[serde(default, alias = "session_id")]
    pub session_id: Option<String>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub skill: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default, alias = "status")]
    pub status: Option<String>,
    #[serde(default, alias = "startedAt")]
    pub started_at: Option<String>,
    #[serde(default, alias = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(default, alias = "lastHeartbeat")]
    pub last_heartbeat: Option<String>,
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default, alias = "operatorSession")]
    pub operator_session: Option<String>,
    #[serde(default, alias = "latestReport")]
    pub latest_report: Option<String>,
    #[serde(default, alias = "latestTranscript")]
    pub latest_transcript: Option<String>,
    #[serde(default, alias = "lastError")]
    pub last_error: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl RunSnapshot {
    pub fn display_state(&self) -> String {
        self.state
            .as_deref()
            .or(self.status.as_deref())
            .unwrap_or("unknown")
            .to_string()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunEvent {
    #[serde(alias = "timestamp")]
    pub ts: String,
    #[serde(alias = "runId")]
    pub run_id: Option<String>,
    pub kind: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub payload: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct RenderedRun {
    pub snapshot: RunSnapshot,
    pub kind: RunKind,
    pub age_label: String,
    pub recent_events: Vec<RunEvent>,
}

pub fn render_runs(state: &ControlPlaneState) -> Vec<RenderedRun> {
    let now = Utc::now();
    let mut runs: Vec<RenderedRun> = state
        .runs
        .iter()
        .cloned()
        .map(|snapshot| {
            let kind = classify_run(&snapshot, now);
            let recent_events = recent_events_for(&state.events, &snapshot.run_id);
            let age_label = age_label(&snapshot, now);
            RenderedRun {
                snapshot,
                kind,
                age_label,
                recent_events,
            }
        })
        .collect();

    runs.sort_by(|left, right| compare_runs(left, right, now));
    runs
}

pub fn classify_run(snapshot: &RunSnapshot, now: DateTime<Utc>) -> RunKind {
    let state = snapshot.display_state().to_lowercase();
    let heartbeat = snapshot
        .last_heartbeat
        .as_deref()
        .and_then(parse_timestamp)
        .or_else(|| snapshot.updated_at.as_deref().and_then(parse_timestamp));

    if snapshot.last_error.is_some() || state.contains("fail") || state.contains("error") {
        return RunKind::Failed;
    }
    if state.contains("stalled") {
        return RunKind::Stalled;
    }
    if state.contains("pause") {
        return RunKind::Paused;
    }
    if state.contains("done")
        || state.contains("complete")
        || state.contains("succeed")
        || state.contains("converged")
        || state.contains("stopped")
        || state.contains("gc")
    {
        return if is_recent(heartbeat, now) {
            RunKind::Recent
        } else {
            RunKind::Completed
        };
    }

    if is_active_like(&state) {
        if is_stale(heartbeat, now) {
            return RunKind::Stalled;
        }
        return RunKind::Active;
    }

    if is_recent(heartbeat, now) {
        return RunKind::Recent;
    }

    RunKind::Unknown
}

fn compare_runs(left: &RenderedRun, right: &RenderedRun, now: DateTime<Utc>) -> Ordering {
    left.kind
        .sort_rank()
        .cmp(&right.kind.sort_rank())
        .then_with(|| compare_timestamp(&right.snapshot.updated_at, &left.snapshot.updated_at))
        .then_with(|| compare_timestamp(&right.snapshot.started_at, &left.snapshot.started_at))
        .then_with(|| {
            compare_timestamp(
                &right.snapshot.last_heartbeat,
                &left.snapshot.last_heartbeat,
            )
        })
        .then_with(|| left.snapshot.run_id.cmp(&right.snapshot.run_id))
        .then_with(|| {
            let left_recent = is_recent(
                left.snapshot
                    .updated_at
                    .as_deref()
                    .and_then(parse_timestamp),
                now,
            );
            let right_recent = is_recent(
                right
                    .snapshot
                    .updated_at
                    .as_deref()
                    .and_then(parse_timestamp),
                now,
            );
            right_recent.cmp(&left_recent)
        })
}

fn compare_timestamp(left: &Option<String>, right: &Option<String>) -> Ordering {
    let left = left.as_deref().and_then(parse_timestamp);
    let right = right.as_deref().and_then(parse_timestamp);
    match (left, right) {
        (Some(left), Some(right)) => right.cmp(&left),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn recent_events_for(events: &[RunEvent], run_id: &str) -> Vec<RunEvent> {
    let mut recent: Vec<RunEvent> = events
        .iter()
        .filter(|event| event.run_id.as_deref() == Some(run_id))
        .cloned()
        .collect();
    recent.sort_by(|left, right| right.ts.cmp(&left.ts));
    recent.truncate(8);
    recent
}

fn age_label(snapshot: &RunSnapshot, now: DateTime<Utc>) -> String {
    let timestamp = snapshot
        .last_heartbeat
        .as_deref()
        .and_then(parse_timestamp)
        .or_else(|| snapshot.updated_at.as_deref().and_then(parse_timestamp))
        .or_else(|| snapshot.started_at.as_deref().and_then(parse_timestamp));
    timestamp
        .map(|ts| relative_age(ts, now))
        .unwrap_or_else(|| "age unknown".to_string())
}

fn load_runs(root: &Path) -> io::Result<Vec<RunSnapshot>> {
    let mut snapshots = HashMap::<String, RunSnapshot>::new();
    for path in run_snapshot_files(root)? {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let parsed: Result<RunSnapshot, _> = serde_json::from_str(&text);
        if let Ok(snapshot) = parsed {
            snapshots.insert(snapshot.run_id.clone(), snapshot);
        }
    }
    Ok(snapshots.into_values().collect())
}

fn load_events(root: &Path) -> io::Result<Vec<RunEvent>> {
    let mut events = Vec::new();
    for path in event_stream_files(root)? {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines().filter(|line| !line.trim().is_empty()) {
            if let Ok(event) = serde_json::from_str::<RunEvent>(line) {
                events.push(event);
            }
        }
    }
    Ok(events)
}

fn run_snapshot_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let runs_dir = root.join("runs");
    safe_child_files(&runs_dir, root, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("json")
    })
}

fn event_stream_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let events_path = root.join("events.jsonl");
    if !events_path.is_file() {
        return Ok(Vec::new());
    }
    let Some(path) = sanitize_control_plane_path(root, &events_path)? else {
        return Ok(Vec::new());
    };
    Ok(vec![path])
}

fn safe_child_files<F>(directory: &Path, root: &Path, predicate: F) -> io::Result<Vec<PathBuf>>
where
    F: Fn(&Path) -> bool,
{
    let mut files = Vec::new();
    if !directory.is_dir() {
        return Ok(files);
    }
    let mut seen = HashSet::new();
    for entry in fs::read_dir(directory)? {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if !predicate(&path) {
            continue;
        }
        let Some(path) = sanitize_control_plane_path(root, &path)? else {
            continue;
        };
        if seen.insert(path.clone()) {
            files.push(path);
        }
    }
    Ok(files)
}

fn sanitize_control_plane_path(root: &Path, candidate: &Path) -> io::Result<Option<PathBuf>> {
    let canonical_root = if root.exists() {
        fs::canonicalize(root)?
    } else {
        return Ok(None);
    };
    let canonical_candidate = if candidate.exists() {
        fs::canonicalize(candidate)?
    } else {
        return Ok(None);
    };
    if canonical_candidate.starts_with(&canonical_root) {
        Ok(Some(canonical_candidate))
    } else {
        Ok(None)
    }
}

fn parse_timestamp(raw: &str) -> Option<DateTime<Utc>> {
    if let Ok(parsed) = DateTime::parse_from_rfc3339(raw) {
        return Some(parsed.with_timezone(&Utc));
    }
    if let Ok(seconds) = raw.parse::<i64>() {
        return Utc.timestamp_opt(seconds, 0).single();
    }
    None
}

fn relative_age(timestamp: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let delta = now.signed_duration_since(timestamp);
    let minutes = delta.num_minutes().max(0);
    let hours = delta.num_hours().max(0);
    if hours >= 24 {
        let days = delta.num_days().max(0);
        return format!("{days}d ago");
    }
    if hours > 0 {
        return format!("{hours}h ago");
    }
    format!("{minutes}m ago")
}

fn is_recent(timestamp: Option<DateTime<Utc>>, now: DateTime<Utc>) -> bool {
    timestamp
        .map(|value| now.signed_duration_since(value).num_hours() < 24)
        .unwrap_or(false)
}

fn is_stale(timestamp: Option<DateTime<Utc>>, now: DateTime<Utc>) -> bool {
    timestamp
        .map(|value| now.signed_duration_since(value).num_minutes() > 15)
        .unwrap_or(false)
}

fn is_active_like(state: &str) -> bool {
    state.contains("active")
        || state.contains("run")
        || state.contains("watch")
        || state.contains("queued")
        || state.contains("pending")
        || state.contains("in-progress")
        || state.contains("progress")
        || state.contains("loop")
}
