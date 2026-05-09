//! Integration tests for `wait_for_interactive_launch`.
//!
//! Drives the readiness loop through a fake `zellij`-shaped shell script so
//! the operator-visible behavior (success / "session exited before probe" /
//! "session never appeared" / probe error preservation) is exercised end to
//! end instead of just verified at command-shape level. Closes vc-review
//! P2-03.

#![cfg(unix)]

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
fn get_message(e: &vibecrafted_operator::LaunchRunError) -> String {
    if let vibecrafted_operator::LaunchRunError::Exec { message, .. } = e {
        message.clone()
    } else {
        panic!()
    }
}
fn get_probe_error(e: &vibecrafted_operator::LaunchRunError) -> Option<String> {
    if let vibecrafted_operator::LaunchRunError::Exec { probe_error, .. } = e {
        probe_error.clone()
    } else {
        panic!()
    }
}
fn get_probe_error_at_deadline(e: &vibecrafted_operator::LaunchRunError) -> Option<String> {
    if let vibecrafted_operator::LaunchRunError::Exec {
        probe_error_at_deadline,
        ..
    } = e
    {
        probe_error_at_deadline.clone()
    } else {
        panic!()
    }
}

use std::time::{Duration, Instant};

use tempfile::TempDir;
use vibecrafted_operator::launch::LaunchCommand;
use vibecrafted_operator::{READINESS_DEADLINE, wait_for_interactive_launch};

const FAKE_SCRIPT: &str = r#"#!/bin/sh
# Skip the optional top-level `--config-dir <dir>` flag so the same script
# can stand in for both the launch invocation and the readiness probe.
if [ "${1:-}" = "--config-dir" ]; then
  shift 2
fi
case "${1:-}" in
  list-sessions)
    if [ -n "${FAKE_VISIBLE_FILE:-}" ] && [ -f "${FAKE_VISIBLE_FILE}" ]; then
      cat "${FAKE_VISIBLE_FILE}"
    fi
    case "${FAKE_PROBE_BEHAVIOR:-ok}" in
      err) echo "probe config not found" >&2; exit 2 ;;
      *) exit 0 ;;
    esac
    ;;
  --session)
    NAME="$2"
    case "${FAKE_INTERACTIVE_BEHAVIOR:-hang}" in
      quick-success) exit 0 ;;
      quick-failure) echo "interactive boom" >&2; exit 7 ;;
      slow-visibility)
        sleep 0.25
        if [ -n "${FAKE_VISIBLE_FILE:-}" ]; then
          echo "$NAME" > "${FAKE_VISIBLE_FILE}"
        fi
        sleep 0.30
        exit 0
        ;;
      *) sleep 30 ;;
    esac
    ;;
  *) exit 0 ;;
esac
"#;

struct FakeZellij {
    _tmp: TempDir,
    program: PathBuf,
    visible_file: PathBuf,
}

fn fake_zellij() -> FakeZellij {
    let tmp = tempfile::tempdir().expect("tempdir");
    let program = tmp.path().join("zellij.sh");
    let visible_file = tmp.path().join("visible.txt");
    fs::write(&program, FAKE_SCRIPT).expect("write fake zellij");
    let mut perms = fs::metadata(&program).expect("metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&program, perms).expect("chmod +x");
    FakeZellij {
        _tmp: tmp,
        program,
        visible_file,
    }
}

fn build_command(
    program: &Path,
    session: &str,
    visible_file: &Path,
    interactive: &str,
    probe: &str,
) -> LaunchCommand {
    let mut env: BTreeMap<String, OsString> = BTreeMap::new();
    env.insert(
        "FAKE_VISIBLE_FILE".to_string(),
        visible_file.as_os_str().to_owned(),
    );
    env.insert("FAKE_INTERACTIVE_BEHAVIOR".to_string(), interactive.into());
    env.insert("FAKE_PROBE_BEHAVIOR".to_string(), probe.into());
    LaunchCommand {
        program: program.to_path_buf(),
        args: vec![
            "--session".into(),
            session.into(),
            "options".into(),
            "--layout-string".into(),
            "noop".into(),
        ],
        env,
    }
}

#[test]
fn quick_child_exit_before_visibility_reports_session_exited() {
    let fake = fake_zellij();
    let session = "vc-op-fake-quickexit";
    let command = build_command(
        &fake.program,
        session,
        &fake.visible_file,
        "quick-success",
        "ok",
    );
    let child = command
        .spawn_interactive_with_stderr()
        .expect("spawn fake zellij");
    let result = wait_for_interactive_launch(&command, child);
    let error = result.expect_err("quick-exit should fail readiness check");
    assert!(
        get_message(&error).contains("exited before the readiness probe saw it"),
        "unexpected message: {}",
        get_message(&error)
    );
    assert!(
        get_message(&error).contains(session),
        "session name must appear in the error: {}",
        get_message(&error)
    );
}

#[test]
fn slow_visibility_then_child_exits_returns_success() {
    let fake = fake_zellij();
    let session = "vc-op-fake-slow";
    let command = build_command(
        &fake.program,
        session,
        &fake.visible_file,
        "slow-visibility",
        "ok",
    );
    let child = command
        .spawn_interactive_with_stderr()
        .expect("spawn fake zellij");
    let started = Instant::now();
    let result = wait_for_interactive_launch(&command, child);
    let elapsed = started.elapsed();
    let output = result.expect("slow-visibility should converge to success");
    assert!(output.status.success(), "fake child should exit zero");
    assert!(
        elapsed < READINESS_DEADLINE + Duration::from_secs(2),
        "slow-visibility test took too long: {elapsed:?}"
    );
}

#[test]
fn deadline_kills_child_when_session_never_visible() {
    let fake = fake_zellij();
    let session = "vc-op-fake-hang";
    let command = build_command(&fake.program, session, &fake.visible_file, "hang", "ok");
    let child = command
        .spawn_interactive_with_stderr()
        .expect("spawn fake zellij");
    let started = Instant::now();
    let result = wait_for_interactive_launch(&command, child);
    let elapsed = started.elapsed();
    let error = result.expect_err("hanging child past deadline must be a failure");
    assert!(
        get_message(&error).contains("did not appear within"),
        "unexpected message: {}",
        get_message(&error)
    );
    assert!(
        get_message(&error).contains(session),
        "session name must appear in the error: {}",
        get_message(&error)
    );
    // Deadline is 2s; killing must release us soon after. Allow 5s slack
    // for slow CI runners.
    assert!(
        elapsed < READINESS_DEADLINE + Duration::from_secs(5),
        "deadline test should not hang for the full 30s sleep: {elapsed:?}"
    );
}

#[test]
fn probe_failure_surfaces_in_launch_error() {
    let fake = fake_zellij();
    let session = "vc-op-fake-probe-err";
    let command = build_command(&fake.program, session, &fake.visible_file, "hang", "err");
    let child = command
        .spawn_interactive_with_stderr()
        .expect("spawn fake zellij");
    let result = wait_for_interactive_launch(&command, child);
    let error = result.expect_err("probe error + hang must produce a failure");
    let probe_error = get_probe_error(&error)
        .clone()
        .expect("probe error must be preserved when probe exits non-zero with stderr");
    assert!(
        probe_error.contains("probe config not found"),
        "probe stderr should be surfaced verbatim: {probe_error}"
    );
    let deadline_probe = get_probe_error_at_deadline(&error)
        .clone()
        .expect("deadline kill must preserve the last probe diagnostic");
    assert!(
        deadline_probe.contains("killed after 2000ms")
            && deadline_probe.contains("last probe error:")
            && deadline_probe.contains("probe config not found"),
        "deadline diagnostic should include kill timing and last probe error: {deadline_probe}"
    );
    // Detail lines render the probe diagnostic in the operator overlay.
    let detail = error.detail_lines("zellij ...".to_string());
    assert!(
        detail
            .iter()
            .any(|line| line.starts_with("readiness probe:")
                && line.contains("probe config not found")),
        "probe error must show in the overlay detail block: {detail:?}"
    );
    assert!(
        detail
            .iter()
            .any(|line| line.starts_with("readiness timeout probe:")
                && line.contains("probe config not found")),
        "deadline probe error must show in the overlay detail block: {detail:?}"
    );
}
