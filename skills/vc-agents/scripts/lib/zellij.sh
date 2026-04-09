#!/usr/bin/env bash

# shellcheck disable=SC2120
spawn_workflow_label() {
  local skill_name="${1:-${SPAWN_SKILL_NAME:-${VIBECRAFTED_SKILL_NAME:-}}}"
  if [[ -n "$skill_name" ]]; then
    printf 'vc-%s\n' "$skill_name"
  else
    printf 'vc-workflow\n'
  fi
}

spawn_write_startup_monitor_script() {
  local script_path="$1"
  local common_path="$2"
  local meta_path="$3"
  local transcript_path="$4"
  local report_path="$5"
  local session_name="$6"
  local workflow_name="$7"
  local landing_kind="$8"
  local landing_name="$9"

  local q_common q_meta q_transcript q_report q_session q_workflow q_kind q_name
  q_common="$(spawn_shell_quote "$common_path")"
  q_meta="$(spawn_shell_quote "$meta_path")"
  q_transcript="$(spawn_shell_quote "$transcript_path")"
  q_report="$(spawn_shell_quote "$report_path")"
  q_session="$(spawn_shell_quote "$session_name")"
  q_workflow="$(spawn_shell_quote "$workflow_name")"
  q_kind="$(spawn_shell_quote "$landing_kind")"
  q_name="$(spawn_shell_quote "$landing_name")"

  cat > "$script_path" <<EOF_MONITOR
#!/usr/bin/env bash
set -euo pipefail
trap 'rm -f "\$0"' EXIT
source $q_common
session_name=$q_session
workflow_name=$q_workflow
landing_kind=$q_kind
landing_name=$q_name

printf 'Your vibecrafted session %s invoked the %s run that landed in %s %s.\\n' "\$session_name" "\$workflow_name" "\$landing_kind" "\$landing_name"
printf 'Watching startup for %ss...\\n\\n' "\${VIBECRAFTED_SPAWN_WATCH_SECONDS:-10}"
spawn_watch_startup $q_meta $q_transcript $q_report
EOF_MONITOR

  chmod +x "$script_path"
}

spawn_open_startup_monitor_pane() {
  local session_name="$1"
  local workflow_name="$2"
  local landing_kind="$3"
  local landing_name="$4"
  local root_dir="${5:-${SPAWN_ROOT:-$(pwd)}}"
  local common_path monitor_script cmd_script

  [[ -n "$session_name" ]] || return 1
  [[ -n "${SPAWN_META:-}" && -n "${SPAWN_TRANSCRIPT:-}" && -n "${SPAWN_REPORT:-}" ]] || return 1
  command -v zellij >/dev/null 2>&1 || return 1

  common_path="$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd .. && pwd)/common.sh"
  monitor_script="$(mktemp "${TMPDIR:-/tmp}/vc-startup-monitor.XXXXXX")"
  cmd_script="$(mkdir -p "${VIBECRAFTED_HOME:-$HOME/.vibecrafted}/tmp" && mktemp "${VIBECRAFTED_HOME:-$HOME/.vibecrafted}/tmp/vc-spawn-cmd.XXXXXX")"

  spawn_write_startup_monitor_script \
    "$monitor_script" \
    "$common_path" \
    "${SPAWN_META:-}" \
    "${SPAWN_TRANSCRIPT:-}" \
    "${SPAWN_REPORT:-}" \
    "$session_name" \
    "$workflow_name" \
    "$landing_kind" \
    "$landing_name"

  spawn_write_command_script "$cmd_script" "bash '$monitor_script'; exit" || return 1

  zellij --session "$session_name" action new-pane \
    --direction down \
    --height 30% \
    --name "startup-monitor" \
    --cwd "$root_dir" \
    -- "$cmd_script"
}

spawn_in_zellij_context() {
  # ZELLIJ=0 is a valid pane index inside zellij — do NOT treat as false.
  # Only absent ZELLIJ means we're outside.
  [[ -n "${ZELLIJ_PANE_ID:-}" ]] || [[ -n "${ZELLIJ+set}" ]]
}

spawn_current_zellij_session_name() {
  printf '%s\n' "${ZELLIJ_SESSION_NAME:-}"
}

spawn_effective_operator_session() {
  spawn_normalize_ambient_context
  local session_name="${VIBECRAFTED_OPERATOR_SESSION:-}"
  if [[ -n "$session_name" ]]; then
    printf '%s\n' "$session_name"
    return 0
  fi

  session_name="${ZELLIJ_SESSION_NAME:-}"
  if [[ -n "$session_name" ]]; then
    printf '%s\n' "$session_name"
    return 0
  fi

  command -v zellij >/dev/null 2>&1 || return 1

  session_name="$(
    zellij list-sessions 2>/dev/null \
      | sed 's/\x1b\[[0-9;]*m//g' \
      | awk '/\(current\)/ {print $1; exit}'
  )"
  [[ -n "$session_name" ]] || return 1
  printf '%s\n' "$session_name"
}

spawn_in_target_zellij_session() {
  local target_session=""
  target_session="$(spawn_effective_operator_session 2>/dev/null || true)"
  spawn_in_zellij_context || return 1
  [[ -n "$target_session" ]] || return 0
  [[ "$(spawn_current_zellij_session_name)" == "$target_session" ]]
}

spawn_pane_direction() {
  # Grid policy: 4 per row, 8 per tab, 9th opens new tab.
  # Uses SPAWN_LOOP_NR (marbles) or VIBECRAFTED_PANE_SEQ (manual).
  # Fresh top-level spawns default to a new tab so they never land in a stale
  # operator tab by accident.
  local seq=""
  local max_per_row=4
  local max_per_tab=8

  if [[ -n "${SPAWN_LOOP_NR:-}" && "${SPAWN_LOOP_NR:-0}" -gt 0 ]]; then
    seq="${SPAWN_LOOP_NR}"
  elif [[ -n "${VIBECRAFTED_PANE_SEQ:-}" && "${VIBECRAFTED_PANE_SEQ:-0}" -gt 0 ]]; then
    seq="${VIBECRAFTED_PANE_SEQ}"
  else
    printf 'new-tab\n'
    return 0
  fi

  if (( seq >= max_per_tab )); then
    printf 'new-tab\n'
  elif (( seq > 0 && seq % max_per_row == 0 )); then
    printf 'down\n'
  else
    printf 'right\n'
  fi
}

spawn_in_zellij_pane() {
  local launcher="$1"
  local pane_name="${2:-agent}"
  local direction="${VIBECRAFTED_ZELLIJ_SPAWN_DIRECTION:-$(spawn_pane_direction)}"
  local launch_cmd="bash '$launcher'"
  local session_name=""
  local workflow_name=""
  local cmd_script

  if spawn_in_zellij_context && command -v zellij >/dev/null 2>&1; then
    # If the operator explicitly targets another zellij session, do not open a
    # pane in the current live session. Fall through to spawn_in_operator_session().
    if ! spawn_in_target_zellij_session; then
      return 1
    fi

    session_name="$(spawn_effective_operator_session 2>/dev/null || true)"
    workflow_name="$(spawn_workflow_label)"
    if [[ "$direction" == "new-tab" && -n "$session_name" ]]; then
      if spawn_open_startup_monitor_pane "$session_name" "$workflow_name" "tab" "$pane_name" "${SPAWN_ROOT:-$(pwd)}"; then
        launch_cmd="VIBECRAFTED_INLINE_STARTUP_WATCH=0 bash '$launcher'"
      fi
    fi

    cmd_script="$(mkdir -p "${VIBECRAFTED_HOME:-$HOME/.vibecrafted}/tmp" && mktemp "${VIBECRAFTED_HOME:-$HOME/.vibecrafted}/tmp/vc-spawn-cmd.XXXXXX")"
    spawn_write_command_script "$cmd_script" "$launch_cmd"

    if [[ "$direction" == "new-tab" ]]; then
      zellij action new-tab \
        --name "$pane_name" \
        --cwd "${SPAWN_ROOT:-$(pwd)}" \
        -- "$cmd_script"
    else
      zellij action new-pane \
        --direction "$direction" \
        --name "$pane_name" \
        --cwd "${SPAWN_ROOT:-$(pwd)}" \
        -- "$cmd_script"
    fi
    return 0
  fi
  return 1
}

spawn_in_operator_session() {
  local launcher="$1"
  local pane_name="${2:-agent}"
  local session_name=""
  local direction="${VIBECRAFTED_ZELLIJ_SPAWN_DIRECTION:-$(spawn_pane_direction)}"
  local effective_direction="$direction"
  local launch_cmd="bash '$launcher'"
  local workflow_name=""
  local cmd_script

  spawn_normalize_ambient_context

  session_name="$(spawn_effective_operator_session 2>/dev/null || true)"
  [[ -n "$session_name" ]] || return 1
  command -v zellij >/dev/null 2>&1 || return 1
  export VIBECRAFTED_OPERATOR_SESSION="$session_name"
  workflow_name="$(spawn_workflow_label)"

  # When routing into a session from outside its active pane context, always
  # open a fresh tab. Otherwise zellij targets whichever operator tab is
  # currently focused, which can be a stale marbles tab.
  if ! spawn_in_target_zellij_session; then
    effective_direction="new-tab"
  fi

  if [[ "$effective_direction" == "new-tab" ]]; then
    if spawn_open_startup_monitor_pane "$session_name" "$workflow_name" "tab" "$pane_name" "${SPAWN_ROOT:-$(pwd)}"; then
      launch_cmd="VIBECRAFTED_INLINE_STARTUP_WATCH=0 bash '$launcher'"
    fi
  fi

  cmd_script="$(mkdir -p "${VIBECRAFTED_HOME:-$HOME/.vibecrafted}/tmp" && mktemp "${VIBECRAFTED_HOME:-$HOME/.vibecrafted}/tmp/vc-spawn-cmd.XXXXXX")"
  spawn_write_command_script "$cmd_script" "$launch_cmd"

  # External spawn into existing operator session — route as pane or new tab per grid policy.
  if [[ "$effective_direction" == "new-tab" ]]; then
    zellij --session "$session_name" action new-tab \
      --name "$pane_name" \
      --cwd "${SPAWN_ROOT:-$(pwd)}" \
      -- "$cmd_script"
  else
    zellij --session "$session_name" action new-pane \
      --direction "$effective_direction" \
      --name "$pane_name" \
      --cwd "${SPAWN_ROOT:-$(pwd)}" \
      -- "$cmd_script"
  fi
}
