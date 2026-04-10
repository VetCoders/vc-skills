from __future__ import annotations

from pathlib import Path

from scripts import installer_gui


def test_build_install_command_respects_shell_toggle(tmp_path: Path) -> None:
    installer = tmp_path / "scripts" / "vetcoders_install.py"
    installer.parent.mkdir(parents=True)
    installer.write_text("#!/usr/bin/env python3\n", encoding="utf-8")

    with_shell = installer_gui.build_install_command(str(tmp_path), with_shell=True)
    without_shell = installer_gui.build_install_command(str(tmp_path), with_shell=False)

    assert with_shell[-1] == "--with-shell"
    assert "--with-shell" not in without_shell
    assert with_shell[:5] == without_shell[:5]


def test_preflight_payload_summarizes_diagnostics(monkeypatch, tmp_path: Path) -> None:
    diagnostics = {
        "frameworks": {
            "workflows": {
                "label": "workflows",
                "found": True,
                "detail": "ready",
            }
        },
        "foundations": {
            "loctree-mcp": {
                "label": "loctree-mcp",
                "found": False,
                "detail": "missing",
            }
        },
        "toolchains": {},
        "agents": {},
        "additional_tools": {},
    }

    monkeypatch.setattr(installer_gui, "read_framework_version", lambda _: "1.2.1")
    monkeypatch.setattr(installer_gui, "run_diagnostics", lambda: diagnostics)
    monkeypatch.setattr(
        installer_gui,
        "start_here_path",
        lambda: tmp_path / "guide" / "START_HERE.md",
    )
    monkeypatch.setattr(
        installer_gui,
        "helper_layer_path",
        lambda: tmp_path / ".config" / "vetcoders" / "vc-skills.sh",
    )

    controller = installer_gui.InstallController(str(tmp_path))
    payload = controller.preflight_payload()

    assert payload["version"] == "1.2.1"
    assert payload["found_count"] == 1
    assert payload["missing_count"] == 1
    assert payload["needs_install"] == {"foundations": ["loctree-mcp"]}
    assert payload["status"]["completed"] is False
