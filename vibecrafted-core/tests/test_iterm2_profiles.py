"""Tests for iTerm2 Dynamic Profile generator and runtime installer."""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from vibecrafted_core import iterm2_profiles as profiles


# --------------------------------------------------------------------- helpers


def test_hex_to_iterm2_six_digit() -> None:
    out = profiles.hex_to_iterm2("#ff6b6b")
    assert out["Color Space"] == "sRGB"
    assert out["Alpha Component"] == 1.0
    assert round(out["Red Component"], 3) == 1.0
    assert round(out["Green Component"], 3) == round(0x6B / 255, 3)


def test_hex_to_iterm2_three_digit() -> None:
    out = profiles.hex_to_iterm2("fff")
    assert out["Red Component"] == 1.0
    assert out["Green Component"] == 1.0
    assert out["Blue Component"] == 1.0


def test_hex_to_iterm2_alpha_override() -> None:
    out = profiles.hex_to_iterm2("#000000", alpha=0.5)
    assert out["Alpha Component"] == 0.5


def test_hex_to_iterm2_invalid_length() -> None:
    with pytest.raises(ValueError):
        profiles.hex_to_iterm2("#ff")


def test_stable_guid_is_deterministic() -> None:
    a = profiles.stable_guid("repo", "vibecrafted")
    b = profiles.stable_guid("repo", "vibecrafted")
    assert a == b


def test_stable_guid_distinguishes_inputs() -> None:
    a = profiles.stable_guid("repo", "vibecrafted")
    b = profiles.stable_guid("repo", "vista")
    c = profiles.stable_guid("mesh", "vibecrafted")
    assert a != b
    assert a != c
    assert b != c


# --------------------------------------------------------------------- ProfileSpec


def test_profilespec_minimal_fields() -> None:
    spec = profiles.ProfileSpec(name="Test", namespace="test", parent=None)
    out = spec.to_iterm2_profile()
    assert out["Name"] == "Test"
    assert "Guid" in out
    assert out["Tags"] == []
    assert "Dynamic Profile Parent Name" not in out


def test_profilespec_with_parent_inheritance() -> None:
    spec = profiles.ProfileSpec(name="Child", namespace="repo", parent="VetCoders Repo")
    out = spec.to_iterm2_profile()
    assert out["Dynamic Profile Parent Name"] == "VetCoders Repo"


def test_profilespec_tab_color_sets_use_flag() -> None:
    spec = profiles.ProfileSpec(
        name="Tabby", namespace="t", parent=None, tab_color="#ff0000"
    )
    out = spec.to_iterm2_profile()
    assert out["Use Tab Color"] is True
    assert "Tab Color" in out


def test_profilespec_custom_command_format() -> None:
    spec = profiles.ProfileSpec(
        name="Dragon",
        namespace="mesh",
        parent=None,
        custom_command="ssh dragon",
    )
    out = spec.to_iterm2_profile()
    assert out["Custom Command"] == "Yes"
    assert out["Command"] == "ssh dragon"


def test_profilespec_extras_merge() -> None:
    spec = profiles.ProfileSpec(
        name="Extras",
        namespace="e",
        parent=None,
        extras={"Custom Foo": True, "Triggers": [{"regex": "^x"}]},
    )
    out = spec.to_iterm2_profile()
    assert out["Custom Foo"] is True
    assert out["Triggers"] == [{"regex": "^x"}]


# --------------------------------------------------------------------- document


def test_build_profiles_document_shape() -> None:
    doc = profiles.build_profiles_document()
    assert "Profiles" in doc
    assert isinstance(doc["Profiles"], list)
    assert len(doc["Profiles"]) == len(profiles.PROFILE_SPECS)


def test_build_profiles_document_includes_parent_first() -> None:
    doc = profiles.build_profiles_document()
    first = doc["Profiles"][0]
    assert first["Name"] == "[experimental] VetCoders Repo"
    assert "Dynamic Profile Parent Name" not in first


def test_build_profiles_document_all_have_guid_and_name() -> None:
    doc = profiles.build_profiles_document()
    guids = set()
    for p in doc["Profiles"]:
        assert "Guid" in p
        assert "Name" in p
        guids.add(p["Guid"])
    # GUIDs must be unique
    assert len(guids) == len(doc["Profiles"])


def test_build_profiles_document_mesh_hosts_present() -> None:
    doc = profiles.build_profiles_document()
    names = {p["Name"] for p in doc["Profiles"]}
    assert "[experimental] VetCoders / dragon" in names
    assert "[experimental] VetCoders / sztudio" in names
    assert "[experimental] VetCoders / silver" in names
    assert "[experimental] VetCoders / div0" in names


def test_build_profiles_document_repo_profiles_present() -> None:
    doc = profiles.build_profiles_document()
    names = {p["Name"] for p in doc["Profiles"]}
    assert "[experimental] VetCoders / vibecrafted" in names
    assert "[experimental] VetCoders / vista" in names
    assert "[experimental] VetCoders / loctree" in names


def test_all_profile_names_carry_experimental_prefix() -> None:
    doc = profiles.build_profiles_document()
    for p in doc["Profiles"]:
        assert p["Name"].startswith("[experimental]"), (
            f"profile {p['Name']!r} missing [experimental] prefix"
        )


def test_serialize_is_valid_json_with_trailing_newline() -> None:
    doc = profiles.build_profiles_document()
    text = profiles.serialize(doc)
    assert text.endswith("\n")
    reparsed = json.loads(text)
    assert reparsed == doc


# --------------------------------------------------------------------- install


def test_install_writes_to_target(tmp_path: Path) -> None:
    target = profiles.install_profiles(target_dir=tmp_path, filename="test.json")
    assert target == tmp_path / "test.json"
    assert target.exists()
    payload = json.loads(target.read_text(encoding="utf-8"))
    assert "Profiles" in payload


def test_install_idempotent_no_overwrite(tmp_path: Path) -> None:
    first = profiles.install_profiles(target_dir=tmp_path, filename="test.json")
    second = profiles.install_profiles(target_dir=tmp_path, filename="test.json")
    assert first == second


def test_install_refuses_overwrite_without_force(tmp_path: Path) -> None:
    target = tmp_path / "test.json"
    target.write_text(
        '{"Profiles": [{"Name": "Other", "Guid": "x"}]}\n', encoding="utf-8"
    )
    with pytest.raises(FileExistsError):
        profiles.install_profiles(target_dir=tmp_path, filename="test.json")


def test_install_force_creates_backup(tmp_path: Path) -> None:
    target = tmp_path / "test.json"
    original = '{"Profiles": [{"Name": "Other", "Guid": "x"}]}\n'
    target.write_text(original, encoding="utf-8")
    profiles.install_profiles(target_dir=tmp_path, filename="test.json", force=True)
    backup = tmp_path / "test.json.bak"
    assert backup.exists()
    assert backup.read_text(encoding="utf-8") == original


def test_install_force_is_idempotent_without_backup_for_identical_payload(
    tmp_path: Path,
) -> None:
    target = profiles.install_profiles(target_dir=tmp_path, filename="test.json")
    profiles.install_profiles(target_dir=tmp_path, filename="test.json", force=True)
    assert target.exists()
    assert not (tmp_path / "test.json.bak").exists()


def test_install_force_skips_backup_when_disabled(tmp_path: Path) -> None:
    target = tmp_path / "test.json"
    target.write_text('{"Profiles": []}\n', encoding="utf-8")
    profiles.install_profiles(
        target_dir=tmp_path, filename="test.json", force=True, backup=False
    )
    assert not (tmp_path / "test.json.bak").exists()


def test_uninstall_removes_existing(tmp_path: Path) -> None:
    target = profiles.install_profiles(target_dir=tmp_path, filename="test.json")
    assert target.exists()
    removed = profiles.uninstall_profiles(target_dir=tmp_path, filename="test.json")
    assert removed
    assert not target.exists()


def test_uninstall_returns_false_when_missing(tmp_path: Path) -> None:
    removed = profiles.uninstall_profiles(target_dir=tmp_path, filename="test.json")
    assert not removed


def test_default_install_dir_in_application_support() -> None:
    target = profiles.default_install_dir()
    assert target.parts[-3:] == ("Application Support", "iTerm2", "DynamicProfiles")
    assert target.is_absolute()


# --------------------------------------------------------------------- CLI


def test_cli_show_emits_valid_json(capsys: pytest.CaptureFixture[str]) -> None:
    rc = profiles._cli(["show"])
    captured = capsys.readouterr()
    assert rc == 0
    parsed = json.loads(captured.out)
    assert "Profiles" in parsed


def test_cli_path_prints_default(capsys: pytest.CaptureFixture[str]) -> None:
    rc = profiles._cli(["path"])
    captured = capsys.readouterr()
    assert rc == 0
    assert "DynamicProfiles" in captured.out


def test_cli_help_includes_operations(capsys: pytest.CaptureFixture[str]) -> None:
    rc = profiles._cli(["--help"])
    captured = capsys.readouterr()
    assert rc == 0
    assert "install" in captured.out
    assert "uninstall" in captured.out
    assert "refresh" in captured.out


def test_cli_unknown_op_returns_2() -> None:
    assert profiles._cli(["nope"]) == 2
