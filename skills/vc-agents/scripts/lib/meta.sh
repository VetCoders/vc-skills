#!/usr/bin/env bash

spawn_control_plane_script() {
  local candidate
  for candidate in \
    "${VIBECRAFTED_ROOT:-}/scripts/control_plane_state.py" \
    "${HOME}/.vibecrafted/tools/vibecrafted-current/scripts/control_plane_state.py" \
    "$(spawn_repo_root 2>/dev/null)/scripts/control_plane_state.py"
  do
    [[ -n "$candidate" && -f "$candidate" ]] || continue
    printf '%s\n' "$candidate"
    return 0
  done
  return 1
}

spawn_sync_control_plane() {
  local script_path
  script_path="$(spawn_control_plane_script 2>/dev/null || true)"
  [[ -n "$script_path" ]] || return 0
  python3 "$script_path" sync >/dev/null 2>&1 || true
}

spawn_find_meta_for_run_id() {
  local reports_dir="$1"
  local target_run_id="$2"

  python3 - "$reports_dir" "$target_run_id" <<'PY'
import json
import os
import sys

reports_dir, target_run_id = sys.argv[1:3]
if not os.path.isdir(reports_dir):
    raise SystemExit(0)

for fname in sorted(os.listdir(reports_dir), reverse=True):
    if not fname.endswith(".meta.json"):
        continue
    fpath = os.path.join(reports_dir, fname)
    try:
        with open(fpath, encoding="utf-8") as handle:
            payload = json.load(handle)
    except (OSError, json.JSONDecodeError):
        continue
    if payload.get("run_id") == target_run_id:
        print(fpath)
        raise SystemExit(0)
PY
}

spawn_read_meta_field() {
  local meta_path="$1"
  local field_name="$2"

  python3 - "$meta_path" "$field_name" <<'PY'
import json
import sys

try:
    with open(sys.argv[1], encoding="utf-8") as handle:
        payload = json.load(handle)
except (OSError, json.JSONDecodeError):
    raise SystemExit(0)

value = payload.get(sys.argv[2], "")
if value is None:
    value = ""
print(value, end="")
PY
}

spawn_write_meta() {
  local meta_path="$1"
  local status="$2"
  local agent="$3"
  local mode="$4"
  local root="$5"
  local input_ref="$6"
  local report="$7"
  local transcript="$8"
  local launcher="$9"
  local model="${10:-__NONE__}"
  local prompt_id="${SPAWN_PROMPT_ID:-}"
  local run_id="${SPAWN_RUN_ID:-}"
  local loop_nr="${SPAWN_LOOP_NR:-0}"
  local skill_code="${SPAWN_SKILL_CODE:-}"
  local framework_version
  framework_version="$(spawn_framework_version)"

  python3 - "$meta_path" "$status" "$agent" "$mode" "$root" "$input_ref" "$report" "$transcript" "$launcher" "$model" "$prompt_id" "$run_id" "$loop_nr" "$skill_code" "$framework_version" <<'PY'
import datetime as dt
import json
import sys

meta_path, status, agent, mode, root, input_ref, report, transcript, launcher, model, prompt_id, run_id, loop_nr, skill_code, framework_version = sys.argv[1:16]
try:
    loop_nr_value = int(loop_nr)
except ValueError:
    loop_nr_value = loop_nr
payload = {
    "updated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
    "status": status,
    "agent": agent,
    "mode": mode,
    "root": root,
    "input": input_ref,
    "report": report,
    "transcript": transcript,
    "launcher": launcher,
    "prompt_id": prompt_id,
    "run_id": run_id,
    "loop_nr": loop_nr_value,
    "skill_code": skill_code,
    "framework_version": framework_version,
    "exit_code": None,
    # launcher_pid is set by the launcher itself once it starts (see
    # spawn_update_meta_pid). Dead launcher_pid → ghost state, out.
    "launcher_pid": None,
    "liveness": "pid_pending",
}
if model != "__NONE__":
    payload["model"] = model
with open(meta_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2, ensure_ascii=False)
    fh.write("\n")
PY
  spawn_sync_control_plane
}

spawn_update_meta_pid() {
  # Called by the generated launcher as soon as it starts. Writes the
  # launcher's own PID into the meta.json so the watcher and the
  # spawn-time GC can validate liveness via `kill -0`. Dead PID = ghost.
  local meta_path="$1"
  local pid="$2"

  [[ -f "$meta_path" ]] || return 0
  [[ -n "$pid" ]] || return 0

  python3 - "$meta_path" "$pid" <<'PY'
import json
import sys

meta_path, pid = sys.argv[1:3]
try:
    with open(meta_path, "r", encoding="utf-8") as fh:
        payload = json.load(fh)
except (OSError, json.JSONDecodeError):
    sys.exit(0)

try:
    payload["launcher_pid"] = int(pid)
    payload["liveness"] = "pid_alive"
except (TypeError, ValueError):
    payload["launcher_pid"] = None
    payload["liveness"] = "unknown_legacy"

with open(meta_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2, ensure_ascii=False)
    fh.write("\n")
PY
}

spawn_pid_alive() {
  # Returns 0 if pid is alive, 1 if dead or invalid. Uses kill -0 semantics;
  # treats permission denied as alive (kernel says: exists, not yours).
  local pid="$1"
  [[ -n "$pid" && "$pid" =~ ^[0-9]+$ ]] || return 1
  kill -0 "$pid" 2>/dev/null
}

spawn_reap_dead_run() {
  # Given a meta.json whose launcher_pid is dead, flip status to "ghost",
  # release any lock file referenced in the meta, and sync control plane.
  # Idempotent — callable from spawn-time GC and from watcher heartbeat.
  local meta_path="$1"
  [[ -f "$meta_path" ]] || return 0

  python3 - "$meta_path" <<'PY'
import datetime as dt
import json
import os
import sys

meta_path = sys.argv[1]
try:
    with open(meta_path, "r", encoding="utf-8") as fh:
        payload = json.load(fh)
except (OSError, json.JSONDecodeError):
    sys.exit(0)

status = payload.get("status")
if status not in ("launching", "running", "in-progress"):
    sys.exit(0)

now_iso = dt.datetime.now(dt.timezone.utc).isoformat()
payload["status"] = "ghost"
payload["updated_at"] = now_iso
payload.setdefault("completed_at", now_iso)
payload.setdefault("exit_code", 137)  # canonical kill-killed code, for parity
payload["ghost_reason"] = "launcher_pid dead at reap"
payload["liveness"] = "pid_dead"

with open(meta_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2, ensure_ascii=False)
    fh.write("\n")

# Best-effort lock cleanup — meta may reference a lock path.
lock_path = payload.get("run_lock") or payload.get("lock")
if lock_path and os.path.isfile(lock_path):
    try:
        os.unlink(lock_path)
    except OSError:
        pass
PY
  spawn_sync_control_plane
}

spawn_mark_unknown_liveness() {
  # Legacy live meta without launcher_pid is not safe to reap. Mark it
  # explicitly so dashboards stop pretending it is verified-live.
  local meta_path="$1"
  [[ -f "$meta_path" ]] || return 0

  python3 - "$meta_path" <<'PY'
import datetime as dt
import json
import sys

meta_path = sys.argv[1]
try:
    with open(meta_path, "r", encoding="utf-8") as fh:
        payload = json.load(fh)
except (OSError, json.JSONDecodeError):
    sys.exit(0)

if payload.get("status") not in ("launching", "running", "in-progress"):
    sys.exit(0)

pid = payload.get("launcher_pid")
if pid not in (None, "", "None"):
    sys.exit(0)

payload["liveness"] = "unknown_legacy"
payload.setdefault("liveness_reason", "live status without launcher_pid")
payload["updated_at"] = dt.datetime.now(dt.timezone.utc).isoformat()

with open(meta_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2, ensure_ascii=False)
    fh.write("\n")
PY
  spawn_sync_control_plane
}

spawn_gc_dead_runs() {
  # Scan a reports directory for meta.json files whose status is live
  # (launching/running/in-progress) but whose launcher_pid is dead.
  # Flip those to ghost. Safe to call at spawn-time before taking locks.
  local reports_dir="$1"
  [[ -d "$reports_dir" ]] || return 0

  local meta_path pid_value
  while IFS= read -r -d '' meta_path; do
    pid_value="$(spawn_read_meta_field "$meta_path" "launcher_pid")"
    # Safe reap contract: only reap when we can VERIFY the PID is dead.
    # Missing launcher_pid = pre-GC-era meta or legacy launcher that never
    # wrote it — we cannot prove death, so we leave it alone. This avoids
    # false-positive reaping of still-running agents whose meta was written
    # by an older launcher template. TTL-based cleanup is a separate path.
    if [[ -z "$pid_value" || "$pid_value" == "None" ]]; then
      spawn_mark_unknown_liveness "$meta_path"
      continue
    fi
    if ! spawn_pid_alive "$pid_value"; then
      spawn_reap_dead_run "$meta_path"
    fi
  done < <(find "$reports_dir" -type f -name '*.meta.json' -print0 2>/dev/null)
}

spawn_finish_meta() {
  local meta_path="$1"
  local status="$2"
  local exit_code="${3:-0}"

  python3 - "$meta_path" "$status" "$exit_code" <<'PY'
import datetime as dt
import json
import re
import sys

meta_path, status, exit_code = sys.argv[1:4]
with open(meta_path, "r", encoding="utf-8") as fh:
    payload = json.load(fh)
completed_at = dt.datetime.now(dt.timezone.utc)
started_at = payload.get("updated_at")
duration_s = None
if isinstance(started_at, str):
    try:
        started_dt = dt.datetime.fromisoformat(started_at)
    except ValueError:
        started_dt = None
    if started_dt is not None:
        duration_s = round((completed_at - started_dt).total_seconds(), 3)
payload["updated_at"] = completed_at.isoformat()
payload["completed_at"] = completed_at.isoformat()
payload["duration_s"] = duration_s
payload["status"] = status
payload["exit_code"] = int(exit_code)
payload["liveness"] = "terminal"

# Parse session_id from transcript (strip ANSI, match "session: <uuid>")
transcript_path = payload.get("transcript", "")
if transcript_path:
    try:
        with open(transcript_path, "r", errors="replace") as tf:
            raw = tf.read(64 * 1024)  # first 64KB is enough
        clean = re.sub(r'\x1b\[[0-9;]*m', '', raw)
        m = re.search(r'session: ([a-f0-9-]{8,})', clean)
        if m:
            payload["session_id"] = m.group(1)
    except (OSError, IOError):
        pass  # transcript not readable — skip silently

with open(meta_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2, ensure_ascii=False)
    fh.write("\n")
PY
  spawn_sync_control_plane
}
