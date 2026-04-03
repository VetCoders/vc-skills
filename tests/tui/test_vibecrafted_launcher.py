from __future__ import annotations

import os
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
LAUNCHER = REPO_ROOT / "scripts" / "vibecrafted"


def _write_fake_agent(bin_dir: Path, name: str, capture_file: Path) -> None:
    script = bin_dir / name
    script.write_text(
        "\n".join(
            [
                "#!/usr/bin/env bash",
                "set -euo pipefail",
                'printf "%s\\n" "$@" > "$CAPTURE_FILE"',
            ]
        )
        + "\n",
        encoding="utf-8",
    )
    script.chmod(0o755)


def _write_fake_command(bin_dir: Path, name: str, capture_file: Path) -> None:
    script = bin_dir / name
    script.write_text(
        "\n".join(
            [
                "#!/usr/bin/env bash",
                "set -euo pipefail",
                "{",
                '  printf "%s\\n" "$@"',
                '  printf "ZELLIJ_CONFIG_DIR=%s\\n" "${ZELLIJ_CONFIG_DIR:-}"',
                '} > "$CAPTURE_FILE"',
            ]
        )
        + "\n",
        encoding="utf-8",
    )
    script.chmod(0o755)


def test_init_prefers_repo_skill_path_when_repo_launcher_runs_with_portable_home(
    tmp_path: Path,
) -> None:
    home = tmp_path / "home"
    portable_home = home / ".portable-vc"
    skill_path = portable_home / "skills" / "vc-init" / "SKILL.md"
    skill_path.parent.mkdir(parents=True)
    skill_path.write_text("# vc-init\n", encoding="utf-8")

    fake_bin = tmp_path / "bin"
    fake_bin.mkdir()
    capture_file = tmp_path / "codex-args.txt"
    _write_fake_agent(fake_bin, "codex", capture_file)

    env = os.environ.copy()
    env["HOME"] = str(home)
    env["PATH"] = f"{fake_bin}:{env.get('PATH', '')}"
    env["VIBECRAFTED_HOME"] = "~/.portable-vc"
    env["CAPTURE_FILE"] = str(capture_file)

    subprocess.run(
        ["bash", str(LAUNCHER), "init", "codex"],
        check=True,
        cwd=REPO_ROOT,
        env=env,
    )

    args = capture_file.read_text(encoding="utf-8").splitlines()
    assert str(REPO_ROOT / "skills" / "vc-init" / "SKILL.md") in args[0]
    assert str(skill_path) not in args[0]


def test_init_falls_back_to_repo_skill_path_when_store_missing(tmp_path: Path) -> None:
    home = tmp_path / "home"
    fake_bin = tmp_path / "bin"
    fake_bin.mkdir()
    capture_file = tmp_path / "claude-args.txt"
    _write_fake_agent(fake_bin, "claude", capture_file)

    env = os.environ.copy()
    env["HOME"] = str(home)
    env["PATH"] = f"{fake_bin}:{env.get('PATH', '')}"
    env.pop("VIBECRAFTED_HOME", None)
    env["CAPTURE_FILE"] = str(capture_file)

    subprocess.run(
        ["bash", str(LAUNCHER), "init", "claude"],
        check=True,
        cwd=REPO_ROOT,
        env=env,
    )

    args = capture_file.read_text(encoding="utf-8").splitlines()
    assert str(REPO_ROOT / "skills" / "vc-init" / "SKILL.md") in args[0]


def test_vc_help_wrapper_symlink_renders_main_help(tmp_path: Path) -> None:
    wrapper = tmp_path / "vc-help"
    wrapper.symlink_to(LAUNCHER)

    result = subprocess.run(
        ["bash", str(wrapper)],
        check=True,
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )

    assert "𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍." in result.stdout
    assert "Front door:" in result.stdout
    assert "vibecrafted dashboard" in result.stdout


def test_repo_launcher_is_directly_executable() -> None:
    expected_version = (REPO_ROOT / "VERSION").read_text(encoding="utf-8").strip()
    result = subprocess.run(
        [str(LAUNCHER), "help"],
        check=True,
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )

    assert "𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍." in result.stdout
    assert expected_version in result.stdout
    assert "vibecrafted dashboard" in result.stdout


def test_skill_subcommand_help_is_human_readable_without_agent() -> None:
    result = subprocess.run(
        [str(LAUNCHER), "justdo", "--help"],
        check=True,
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )

    assert "justdo" in result.stdout
    assert "Autonomous end-to-end implementation" in result.stdout
    assert "vibecrafted justdo <claude|codex|gemini> [flags]" in result.stdout
    assert "vibecrafted implement <agent> [flags]" in result.stdout


def test_skill_wrapper_help_is_human_readable_without_agent(tmp_path: Path) -> None:
    wrapper = tmp_path / "vc-followup"
    wrapper.symlink_to(LAUNCHER)

    result = subprocess.run(
        [str(wrapper), "--help"],
        check=True,
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )

    assert "followup" in result.stdout
    assert "Post-implementation audit" in result.stdout
    assert "vc-followup <claude|codex|gemini> [flags]" in result.stdout


def test_agent_subcommand_help_lists_modes() -> None:
    result = subprocess.run(
        [str(LAUNCHER), "codex", "--help"],
        check=True,
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )

    assert "Plan-based helper modes for codex." in result.stdout
    assert "implement <plan.md>" in result.stdout
    assert "observe   --last" in result.stdout


def test_dashboard_subcommand_launches_repo_owned_zellij_layout(tmp_path: Path) -> None:
    home = tmp_path / "home"
    fake_bin = tmp_path / "bin"
    capture_file = tmp_path / "zellij-args.txt"

    home.mkdir()
    fake_bin.mkdir()
    _write_fake_command(fake_bin, "zellij", capture_file)

    env = os.environ.copy()
    env["HOME"] = str(home)
    env["PATH"] = f"{fake_bin}:{env.get('PATH', '')}"
    env["CAPTURE_FILE"] = str(capture_file)
    env.pop("ZELLIJ_CONFIG_DIR", None)
    env.pop("ZELLIJ", None)
    env.pop("ZELLIJ_PANE_ID", None)
    env.pop("ZELLIJ_SESSION_NAME", None)

    subprocess.run(
        ["bash", str(LAUNCHER), "dashboard"],
        check=True,
        cwd=REPO_ROOT,
        env=env,
    )

    payload = capture_file.read_text(encoding="utf-8").splitlines()
    assert "--session" in payload
    assert "vibecrafted-dashboard" in payload
    assert "--new-session-with-layout" in payload
    assert (
        str(REPO_ROOT / "config" / "zellij" / "layouts" / "vc-dashboard.kdl") in payload
    )
    assert f"ZELLIJ_CONFIG_DIR={REPO_ROOT / 'config' / 'zellij'}" in payload


def test_start_subcommand_launches_operator_entrypoint_layout(tmp_path: Path) -> None:
    home = tmp_path / "home"
    fake_bin = tmp_path / "bin"
    capture_file = tmp_path / "zellij-args.txt"

    home.mkdir()
    fake_bin.mkdir()
    _write_fake_command(fake_bin, "zellij", capture_file)

    env = os.environ.copy()
    env["HOME"] = str(home)
    env["PATH"] = f"{fake_bin}:{env.get('PATH', '')}"
    env["CAPTURE_FILE"] = str(capture_file)
    env.pop("ZELLIJ_CONFIG_DIR", None)
    env.pop("ZELLIJ", None)
    env.pop("ZELLIJ_PANE_ID", None)
    env.pop("ZELLIJ_SESSION_NAME", None)

    subprocess.run(
        ["bash", str(LAUNCHER), "start"],
        check=True,
        cwd=REPO_ROOT,
        env=env,
    )

    payload = capture_file.read_text(encoding="utf-8").splitlines()
    assert "--session" in payload
    assert "vibecrafted" in payload
    assert "--new-session-with-layout" in payload
    assert (
        str(REPO_ROOT / "config" / "zellij" / "layouts" / "vibecrafted.kdl") in payload
    )
    assert f"ZELLIJ_CONFIG_DIR={REPO_ROOT / 'config' / 'zellij'}" in payload
