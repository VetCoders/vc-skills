"""iTerm2 Dynamic Profiles generator + runtime installer (experimental).

Source-of-truth lives in ``PROFILE_SPECS`` below: the parent
``[experimental] VetCoders Repo`` profile plus per-host (mesh topology)
and per-repo children. ``build_profiles_document()`` materializes the
iTerm2-compatible JSON. ``install_profiles()`` writes that JSON to the
user's ``~/Library/Application Support/iTerm2/DynamicProfiles/``
directory, where iTerm2 hot-reloads it.

The default install filename is ``vibecrafted-experimental.json`` and
every profile name carries the ``[experimental]`` prefix so this layer
sits ALONGSIDE existing iTerm2 profiles, never replacing them. Once the
shape stabilizes the prefix can be dropped and the file renamed.

References:
- https://iterm2.com/documentation-dynamic-profiles.html
- Profile schema: ``Settings → Profiles → Other Actions → Save Profile as JSON``
"""

from __future__ import annotations

import json
import sys
import uuid
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Iterable, Mapping

# --------------------------------------------------------------------- helpers


def hex_to_iterm2(hex_color: str, alpha: float = 1.0) -> dict[str, Any]:
    """Convert a ``#rrggbb`` (or ``rrggbb``) hex string to iTerm2 color dict.

    iTerm2 stores colors as floats 0..1 with explicit color space.
    """
    s = hex_color.lstrip("#")
    if len(s) == 3:
        s = "".join(ch * 2 for ch in s)
    if len(s) != 6:
        raise ValueError(
            f"hex_to_iterm2: expected 3 or 6 hex digits, got {hex_color!r}"
        )
    r, g, b = (int(s[i : i + 2], 16) / 255 for i in (0, 2, 4))
    return {
        "Red Component": round(r, 6),
        "Green Component": round(g, 6),
        "Blue Component": round(b, 6),
        "Alpha Component": round(alpha, 6),
        "Color Space": "sRGB",
    }


def stable_guid(namespace: str, name: str) -> str:
    """Deterministic UUID derived from namespace+name.

    Same input always produces the same GUID across runs, which lets
    iTerm2 reuse existing profiles instead of duplicating them on each
    install. Uses uuid5 with the standard DNS namespace.
    """
    return str(uuid.uuid5(uuid.NAMESPACE_DNS, f"vetcoders.{namespace}.{name}"))


# --------------------------------------------------------------------- specs


@dataclass(frozen=True)
class ProfileSpec:
    """Source-of-truth entry for a generated iTerm2 profile.

    `parent` is the ``Name`` of another profile (built-in or generated);
    iTerm2 inherits unspecified attributes from it. Use ``None`` for the
    parent profile itself.

    `extras` is merged verbatim into the output JSON, so any iTerm2
    profile key may be overridden (Triggers, Smart Selection Rules,
    custom font sizes, etc.).
    """

    name: str
    namespace: str
    parent: str | None
    tags: tuple[str, ...] = ()
    badge: str | None = None
    foreground: str | None = None  # hex
    background: str | None = None  # hex
    cursor: str | None = None  # hex
    tab_color: str | None = None  # hex
    custom_window_title: str | None = None
    custom_command: str | None = None
    extras: Mapping[str, Any] = field(default_factory=dict)

    def to_iterm2_profile(self) -> dict[str, Any]:
        out: dict[str, Any] = {
            "Name": self.name,
            "Guid": stable_guid(self.namespace, self.name),
            "Tags": list(self.tags),
        }
        if self.parent is not None:
            out["Dynamic Profile Parent Name"] = self.parent
        if self.badge is not None:
            out["Badge Text"] = self.badge
        if self.foreground is not None:
            out["Foreground Color"] = hex_to_iterm2(self.foreground)
        if self.background is not None:
            out["Background Color"] = hex_to_iterm2(self.background)
        if self.cursor is not None:
            out["Cursor Color"] = hex_to_iterm2(self.cursor)
        if self.tab_color is not None:
            out["Tab Color"] = hex_to_iterm2(self.tab_color)
            out["Use Tab Color"] = True
        if self.custom_window_title is not None:
            out["Use Custom Window Title"] = True
            out["Custom Window Title"] = self.custom_window_title
        if self.custom_command is not None:
            out["Custom Command"] = "Yes"
            out["Command"] = self.custom_command
        out.update(self.extras)
        return out


# --------------------------------------------------------------------- mesh + repos


# Mesh topology — one profile per known VetCoders host. Colors carry
# operator-readable identity from the command-tab overview.
MESH_HOSTS: tuple[ProfileSpec, ...] = (
    ProfileSpec(
        name="[experimental] VetCoders / dragon",
        namespace="mesh",
        parent="[experimental] VetCoders Repo",
        tags=("vetcoders", "mesh", "ssh"),
        badge=r"🐉 dragon",
        background="#1a0e0e",
        foreground="#ffe5e0",
        cursor="#ff6b6b",
        tab_color="#ff6b6b",
        custom_command="ssh dragon",
    ),
    ProfileSpec(
        name="[experimental] VetCoders / sztudio",
        namespace="mesh",
        parent="[experimental] VetCoders Repo",
        tags=("vetcoders", "mesh", "ssh"),
        badge=r"🟣 sztudio",
        background="#0f0a14",
        foreground="#e9d5ff",
        cursor="#a78bfa",
        tab_color="#a78bfa",
        custom_command="ssh sztudio",
    ),
    ProfileSpec(
        name="[experimental] VetCoders / silver",
        namespace="mesh",
        parent="[experimental] VetCoders Repo",
        tags=("vetcoders", "mesh", "ssh"),
        badge=r"💿 silver (via sztudio)",
        background="#06141b",
        foreground="#cffafe",
        cursor="#67e8f9",
        tab_color="#67e8f9",
        custom_command="ssh sztudio 'ssh silver'",
    ),
    ProfileSpec(
        name="[experimental] VetCoders / div0",
        namespace="mesh",
        parent="[experimental] VetCoders Repo",
        tags=("vetcoders", "mesh", "local"),
        badge=r"🌱 div0 (local)",
        background="#0a1410",
        foreground="#dcfce7",
        cursor="#86efac",
        tab_color="#86efac",
    ),
)


# Per-repo profiles — open new tab in repo working directory, set badge
# from session variables. Parent inherits font/keys/global behavior.
REPO_PROFILES: tuple[ProfileSpec, ...] = (
    ProfileSpec(
        name="[experimental] VetCoders / vibecrafted",
        namespace="repo",
        parent="[experimental] VetCoders Repo",
        tags=("vetcoders", "repo", "framework"),
        badge=r"\(user.vetcoders.repo) — \(user.vetcoders.zellij_session)",
        tab_color="#fbbf24",  # vibecrafted brand amber
        custom_window_title=r"𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍 — \(session.path)",
    ),
    ProfileSpec(
        name="[experimental] VetCoders / vista",
        namespace="repo",
        parent="[experimental] VetCoders Repo",
        tags=("vetcoders", "repo", "vista"),
        badge=r"Vista — \(user.vetcoders.zellij_session)",
        tab_color="#10b981",  # vista emerald
        custom_window_title=r"Vista — \(session.path)",
    ),
    ProfileSpec(
        name="[experimental] VetCoders / loctree",
        namespace="repo",
        parent="[experimental] VetCoders Repo",
        tags=("vetcoders", "repo", "loctree"),
        badge=r"Loctree — \(user.vetcoders.zellij_session)",
        tab_color="#3b82f6",  # loctree map blue
        custom_window_title=r"Loctree — \(session.path)",
    ),
)


# Parent profile — the only spec without `parent`. Inherits from iTerm2
# default profile via implicit fallback. All children inherit unspecified
# attributes from this one.
PARENT_PROFILE = ProfileSpec(
    name="[experimental] VetCoders Repo",
    namespace="parent",
    parent=None,
    tags=("vetcoders", "parent"),
    badge=r"\(user.vetcoders.repo)",
    extras={
        # Modest defaults — children override colors/badge/tabs.
        "Working Directory": "Recycle",
        "Custom Directory": "Recycle",
        "Allow Title Setting": True,
        "Allow Title Reporting": True,
    },
)


PROFILE_SPECS: tuple[ProfileSpec, ...] = (PARENT_PROFILE,) + MESH_HOSTS + REPO_PROFILES


# --------------------------------------------------------------------- builders


def build_profiles_document(
    specs: Iterable[ProfileSpec] = PROFILE_SPECS,
) -> dict[str, Any]:
    """Materialize the iTerm2 DynamicProfile JSON document."""
    return {"Profiles": [spec.to_iterm2_profile() for spec in specs]}


def serialize(doc: Mapping[str, Any]) -> str:
    """Stable JSON serialization for diff-friendly output."""
    return json.dumps(doc, indent=2, sort_keys=False, ensure_ascii=False) + "\n"


# --------------------------------------------------------------------- install


def default_install_dir() -> Path:
    """iTerm2's monitored DynamicProfiles directory."""
    return (
        Path.home() / "Library" / "Application Support" / "iTerm2" / "DynamicProfiles"
    )


def install_profiles(
    *,
    target_dir: Path | None = None,
    filename: str = "vibecrafted-experimental.json",
    force: bool = False,
    specs: Iterable[ProfileSpec] = PROFILE_SPECS,
    backup: bool = True,
) -> Path:
    """Write the dynamic profile JSON to iTerm2's monitored directory.

    Returns the path written. Creates the parent directory if missing.

    If a file already exists at the target and `force` is False, raises
    ``FileExistsError``. Pass ``force=True`` to overwrite (the previous
    file is preserved as ``<filename>.bak`` when ``backup=True``).
    """
    target_dir = target_dir or default_install_dir()
    target_dir.mkdir(parents=True, exist_ok=True)
    target = target_dir / filename

    payload = serialize(build_profiles_document(specs))

    if target.exists():
        existing = target.read_text(encoding="utf-8")
        if existing == payload:
            return target  # idempotent no-op
        if force:
            if backup:
                backup_path = target.with_suffix(target.suffix + ".bak")
                backup_path.write_text(existing, encoding="utf-8")
        else:
            raise FileExistsError(
                f"{target} exists with different content; pass force=True to overwrite"
            )

    target.write_text(payload, encoding="utf-8")
    return target


def uninstall_profiles(
    *,
    target_dir: Path | None = None,
    filename: str = "vibecrafted-experimental.json",
) -> bool:
    """Remove a previously installed profiles file. Returns True if removed."""
    target_dir = target_dir or default_install_dir()
    target = target_dir / filename
    if not target.exists():
        return False
    target.unlink()
    return True


# --------------------------------------------------------------------- CLI


def _cli(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        print(
            "Usage: python -m vibecrafted_core.iterm2_profiles <op>\n"
            "\n"
            "Operations:\n"
            "  show              Print the JSON document to stdout\n"
            "  install           Write to iTerm2 DynamicProfiles dir (idempotent)\n"
            "  install --force   Overwrite existing file (creates .bak first)\n"
            "  refresh           Alias for `install --force`\n"
            "  uninstall         Remove the installed file\n"
            "  path              Print the install target path\n"
        )
        return 0

    op = argv[0]
    flags = argv[1:]
    force = "--force" in flags or "-f" in flags

    if op == "show":
        print(serialize(build_profiles_document()), end="")
        return 0
    if op == "path":
        print(default_install_dir() / "vibecrafted-experimental.json")
        return 0
    if op in ("install", "refresh"):
        try:
            target = install_profiles(force=force or op == "refresh")
        except FileExistsError as err:
            print(f"error: {err}", file=sys.stderr)
            print("hint: pass --force to overwrite (creates a .bak first)")
            return 3
        print(f"installed: {target}")
        return 0
    if op == "uninstall":
        removed = uninstall_profiles()
        print("removed" if removed else "nothing to remove")
        return 0

    print(f"unknown op: {op!r}", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(_cli(sys.argv[1:]))
