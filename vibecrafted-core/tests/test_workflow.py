from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest

from vibecrafted_core import workflow


def _source_dir(tmp_path: Path) -> Path:
    root = tmp_path / "src"
    scripts = root / "scripts"
    scripts.mkdir(parents=True)
    launcher = scripts / "vibecrafted"
    launcher.write_text("#!/usr/bin/env bash\nexit 0\n", encoding="utf-8")
    launcher.chmod(0o755)
    return root


def test_normalize_launch_spec_requires_prompt_or_file(tmp_path: Path) -> None:
    with pytest.raises(ValueError, match="Launch requires"):
        workflow.normalize_launch_spec({"skill": "workflow"}, tmp_path)


def test_launch_workflow_returns_pid_and_logs_spawn(
    monkeypatch: pytest.MonkeyPatch, tmp_path: Path
) -> None:
    monkeypatch.setenv("VIBECRAFTED_HOME", str(tmp_path / ".vibecrafted"))
    source = _source_dir(tmp_path)
    spec = workflow.normalize_launch_spec(
        {"skill": "workflow", "agent": "claude", "prompt": "go"},
        source,
    )

    payload = workflow.launch_workflow(spec, source)

    assert payload["accepted"] is True
    assert isinstance(payload["pid"], int)
    log_lines = Path(payload["launch_log"]).read_text(encoding="utf-8").splitlines()
    assert any(json.loads(line).get("event") == "spawned" for line in log_lines)


def test_launch_workflow_reports_spawn_error(
    monkeypatch: pytest.MonkeyPatch, tmp_path: Path
) -> None:
    monkeypatch.setenv("VIBECRAFTED_HOME", str(tmp_path / ".vibecrafted"))
    spec = workflow.WorkflowLaunchSpec(
        agent="claude",
        mode="workflow",
        skill="workflow",
        prompt="go",
        file="",
        runtime="headless",
        root=str(tmp_path),
    )

    def _missing_command(*_args: Any, **_kwargs: Any) -> list[str]:
        return ["definitely-missing-vibecrafted-binary"]

    monkeypatch.setattr(workflow, "build_launch_command", _missing_command)
    payload = workflow.launch_workflow(spec, tmp_path)

    assert payload["accepted"] is False
    assert "FileNotFoundError" in payload["error"]
