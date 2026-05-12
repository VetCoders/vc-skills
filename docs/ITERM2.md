# iTerm2 Stack (GA since v1.8.0)

_Plan 10 (META_22) — promoted from `[experimental]` framing on 2026-05-12._

The Vibecrafted iTerm2 stack ships two layers on top of stock iTerm2: a
**Python OSC primitive library** (`vibecrafted_core.iterm2_osc`) for emitting
escape sequences from any program, and a **Dynamic Profiles generator**
(`vibecrafted_core.iterm2_profiles`) that installs a small set of operator
profiles into iTerm2's hot-reload directory.

> **Anti-aesthetic note.** This surface exists for the AI-agent + operator
> workflow — colored mesh-host profiles, badge-driven repo identity, OSC 8
> clickable hyperlinks in agent dashboards. It is **not** a customer-facing
> design system. Customers see Vibecrafted via the marketplace plugin and
> the install funnel, not via your terminal chrome.

## Status

- **Module status:** GA since v1.8.0 / 2026-05-12.
- **Wire contract:** stable. OSC primitive signatures, profile JSON shape,
  and GUID derivation (`uuid5(DNS, "vetcoders.<namespace>.<name>")`) are
  public API.
- **Predecessor:** v1.7 shipped this stack under the `[experimental]`
  prefix (kronika 2026-05-08). Operators with `vibecrafted-experimental.json`
  on disk should run `make iterm-plugin-migrate` once on upgrade — see
  [Migration](#migration-from-v17-experimental) below.

## What you get

### OSC primitives — `vibecrafted_core.iterm2_osc`

A pure-stdlib library that returns the literal byte strings iTerm2
understands. The functions never write to stdout themselves — callers
decide whether to `print()` them, embed them in shell commands, or push
them through an agent dashboard.

Coverage: OSC 1337 (badge, profile switch, user variables, colors, marks,
focus, scrollback, cursor shape, blocks, custom buttons), OSC 9
(notifications + tab progress bar), OSC 8 (hyperlinks), OSC 133 (FinalTerm
shell integration), OSC 4 (color reporting).

Example — clickable hyperlink in an agent log:

```python
from vibecrafted_core import iterm2_osc as osc

print(
    osc.hyperlink(
        "https://github.com/VetCoders/vibecrafted/blob/release/v1.7.1/docs/ITERM2.md",
        "open iTerm2 guide",
    )
)
```

Example — drive the tab progress bar from a long-running task:

```python
print(osc.progress(3))               # indeterminate spinner
# ... work ...
print(osc.progress(1, percent=100))  # success solid
print(osc.progress(0))               # clear
```

CLI surface (handy for shell scripts):

```bash
uv run --project vibecrafted-core python -m vibecrafted_core.iterm2_osc badge "🐉 dragon"
uv run --project vibecrafted-core python -m vibecrafted_core.iterm2_osc hyperlink \
    https://vibecrafted.io "open landing"
uv run --project vibecrafted-core python -m vibecrafted_core.iterm2_osc progress 1 75
```

Reference: <https://iterm2.com/documentation-escape-codes.html>.

### Dynamic Profiles — `vibecrafted_core.iterm2_profiles`

A small, opinionated set of iTerm2 Dynamic Profiles that ship alongside
your existing profiles. iTerm2 hot-reloads
`~/Library/Application Support/iTerm2/DynamicProfiles/vibecrafted.json`
the moment it appears, so no app restart is required.

Shipped profiles (8 total):

| Profile                   | Purpose                                       |
| ------------------------- | --------------------------------------------- |
| `VetCoders Repo`          | Parent profile — children inherit defaults    |
| `VetCoders / dragon`      | Mesh host: `ssh dragon`, red identity         |
| `VetCoders / sztudio`     | Mesh host: `ssh sztudio`, purple identity     |
| `VetCoders / silver`      | Mesh host: silver via sztudio jump, cyan      |
| `VetCoders / div0`        | Mesh host: local, green                       |
| `VetCoders / vibecrafted` | Repo profile: amber tab, badge + window title |
| `VetCoders / vista`       | Repo profile: emerald tab                     |
| `VetCoders / loctree`     | Repo profile: map-blue tab                    |

GUIDs are deterministic (uuid5 of `vetcoders.<namespace>.<name>`), so
re-running `make iterm-plugin` is idempotent and does not create duplicate
profile rows in iTerm2's Settings → Profiles.

## Operator commands

All commands are driven through the top-level Makefile:

```bash
make iterm-plugin           # install (idempotent, alongside existing profiles)
make iterm-plugin-refresh   # overwrite installed file (creates .bak first)
make iterm-plugin-show      # print generated JSON to stdout
make iterm-plugin-uninstall # remove the installed file
make iterm-plugin-migrate   # migrate v1.7 vibecrafted-experimental.json → GA
```

Under the hood these wrap `python -m vibecrafted_core.iterm2_profiles`,
which is documented via `--help`:

```bash
uv run --project vibecrafted-core python -m vibecrafted_core.iterm2_profiles --help
```

## Migration from v1.7 experimental

If you installed the iTerm2 stack on v1.7 — kronika 2026-05-08 ships it as
`[experimental]` — you have a file like:

```
~/Library/Application Support/iTerm2/DynamicProfiles/vibecrafted-experimental.json
```

with profile names like `[experimental] VetCoders / dragon`. On v1.8.0
upgrade, run **once**:

```bash
make iterm-plugin-migrate
```

This will:

1. Read `vibecrafted-experimental.json`.
2. Strip the `[experimental]` prefix from each profile `Name` and each
   `Dynamic Profile Parent Name` reference.
3. Preserve every `Guid` verbatim — **this matters**: iTerm2 keys profiles
   by GUID, so reusing the GUIDs means iTerm2 sees a _rename_, not a
   _new profile_. You will not see profile duplication in
   Settings → Profiles, and any profile assignments you made (default,
   per-tab, per-window) keep pointing at the same row.
4. Write the cleaned document to `vibecrafted.json`.
5. Back the legacy file up to `vibecrafted-experimental.json.bak`.
6. Remove the legacy file.

The migration is **idempotent**. Re-running it on an already-migrated tree
prints `already migrated: <path> present, nothing to do` and exits 0
without touching anything. Running it on a clean tree (no legacy file, no
GA file) prints `nothing to migrate` and exits 0.

After migration, future `make iterm-plugin-refresh` runs operate on
`vibecrafted.json` directly. The `.bak` is yours to delete whenever you're
satisfied.

### What if you customized the experimental profiles?

The migration helper preserves every JSON field verbatim except `Name` and
`Dynamic Profile Parent Name`. Custom triggers, key bindings, font tweaks,
or smart selection rules you added in iTerm2 Settings will carry over —
but only if they made it into the JSON file iTerm2 holds the dynamic
profiles in. If you edited the profile interactively in iTerm2's UI
_without_ iTerm2 writing those changes back to the dynamic profiles file,
your edits live in iTerm2's own profile DB and the migration leaves them
alone (they continue to apply to whatever Name iTerm2 has indexed against
the GUID).

If you forked the dynamic profiles JSON manually (e.g. added a profile by
hand), the migration will still process it — every `Name` gets the prefix
stripped, every `Dynamic Profile Parent Name` gets rewritten. Verify the
output before relying on it in production.

## Related surfaces

- **Plan 11 (next):** Hammerspoon URL handlers (`vc-*` clickable actions
  embedded in OSC 8 hyperlinks) — extends this stack with desktop-level
  routing for agent dashboards.
- **Plan 12 (Wave 4):** Zellij multi-agent layouts + mesh theme switching
  (`config/zellij/themes/vetcoders-mesh.kdl`). The iTerm2 profile colors
  intentionally rhyme with the zellij mesh themes — same identity, two
  presentation layers.
- **kronika 2026-05-08:** experimental ship landmark — documents the
  original 18 OSC primitives, 8 ProfileSpecs, and 58 tests that shipped
  as `[experimental]` before this GA promotion.

## Testing

- In-process pytest: `vibecrafted-core/tests/test_iterm2_osc.py` +
  `vibecrafted-core/tests/test_iterm2_profiles.py` (82 tests total, run
  via `make test` or the vibecrafted-core test suite directly).
- Bash smoke: `make test-iterm2-migrate` — sandboxed end-to-end check of
  the experimental→GA migration path.

## Vibecrafted with AI Agents (c)2024-2026 LibraxisAI
