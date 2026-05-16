# Zellij Multi-Agent Layouts

> Plan 12 (META_22) — Wave 4 agent-native runtime cut.

VibeCrafted ships a zellij configuration tuned for the way VetCoders actually
work: parallel agents, shared Living Tree, mesh of workstations, no babysitting.
The shipped surface gives every layout a live AICX session counter, a loctree
snapshot-age indicator, and host-aware identity colors so an operator instantly
knows which machine they are looking at.

This document covers what is shipped, how it auto-discovers itself, and how to
extend it.

## What ships

```
config/zellij/
├── config.kdl                       # base config + neutral theme
├── aicx-status.sh                   # live AICX session counter (1-line)
├── loctree-drift.sh                 # loctree snapshot age (1-line, color)
├── auto-theme.sh                    # host detection -> theme name
├── themes/
│   └── vetcoders-mesh.kdl           # 4 mesh themes (dragon/sztudio/silver/div0)
└── layouts/
    ├── operator.kdl                 # entrypoint  -- `vibecrafted start`
    ├── dashboard.kdl                # mission control 2x2 grid
    ├── marbles.kdl                  # convergence workspace
    ├── research.kdl                 # triple-agent research swarm
    └── workflow.kdl                 # ERi implementation workspace
```

Once installed (`vibecrafted install` or `make install`), the framework symlinks
this directory under `~/.config/vetcoders/frontier/zellij/` and the layouts
become reachable through the `vibecrafted dashboard <layout>` family of CLIs.

## Status row — what the operator sees

Each layout's `default_tab_template` carries a thin one-line pane above the
compact status-bar plugin, split 50/50:

```
[ aicx: 2/5 (claude+codex)   ][ loctree: drift vibecrafted (7m) ]
```

The two halves run independent shell loops; either failing is non-fatal — the
helper prints a single-line warning and the rest of the layout keeps working.

### `aicx-status.sh`

Reads `~/.aicx/store/<org>/<project>/<YYYY_MMDD>/conversations/<agent>/*.{md,jsonl}`
for the _current_ day. Counts **total** segments touched today and **active**
segments modified within the last `AICX_STATUS_WINDOW` seconds (default 600).
Output:

| state                  | example                                  | color |
| ---------------------- | ---------------------------------------- | ----- |
| active sessions exist  | `aicx: 3/12 (claude+codex+gemini)`       | green |
| idle but day populated | `aicx: 0/12`                             | amber |
| empty / no store       | `aicx: 0/0 idle` / `aicx: store offline` | dim   |

Knobs:

- `AICX_STORE` — override the store root (default `~/.aicx/store`)
- `AICX_STATUS_WINDOW` — "active" window in seconds (default 600)
- `AICX_STATUS_REFRESH` — refresh interval in seconds (default 5)
- `AICX_STATUS_ONESHOT` — emit one line and exit (used by smoke tests)

### `loctree-drift.sh`

Walks the operator's repo neighbourhood (PWD + `~/vc-workspace` + `~/Libraxis`
by default) for `*/.loctree/snapshot.json`. Reports the **oldest** snapshot age:

| age band     | output                                | color |
| ------------ | ------------------------------------- | ----- |
| < 5 min      | `loctree: fresh (N snapshots)`        | green |
| < 1 hour     | `loctree: drift <repo> (12m)`         | amber |
| ≥ 1 hour     | `loctree: stale <repo> (3h)` / `(5d)` | red   |
| no snapshots | `loctree: no snapshots`               | dim   |

Knobs:

- `LOCTREE_DRIFT_ROOTS` — colon-separated roots to scan
- `LOCTREE_DRIFT_DEPTH` — `find -maxdepth` limit (default 4)
- `LOCTREE_DRIFT_REFRESH` — refresh interval (default 30s)
- `LOCTREE_DRIFT_ONESHOT` — emit one line and exit

## Mesh-aware host theming

Kronika 2026-05-05 fixed the VetCoders mesh topology and assigned a default
accent color to each workstation so an operator can instantly tell which
machine they are looking at through screen-share or browser-mirrored zellij:

| host    | theme               | accent | role                             |
| ------- | ------------------- | ------ | -------------------------------- |
| dragon  | `vetcoders-dragon`  | red    | LibraxisAI server, central hub   |
| sztudio | `vetcoders-sztudio` | purple | Monika's desktop                 |
| silver  | `vetcoders-silver`  | cyan   | Monika's laptop                  |
| div0    | `vetcoders-div0`    | green  | Maciej's laptop, primary dev     |
| \*      | `vibecrafted`       | amber  | neutral default (fleet baseline) |

The themes live in `config/zellij/themes/vetcoders-mesh.kdl`. Zellij auto-loads
nested theme blocks from the same config dir, so no extra wiring is needed at
the framework level.

### Resolving the theme at runtime

`config/zellij/auto-theme.sh` emits the theme name for the current workstation.
Detection order:

1. `VIBECRAFTED_HOST_NAME` (operator override — useful for tests/staging)
2. `scutil --get LocalHostName` (macOS default local name)
3. `scutil --get ComputerName` (macOS user-friendly name)
4. `hostname -s` / `hostname` (Linux fallback)

The result is normalized (lowercase + strip `.local`/`.lan`) before matching,
and `mgbook16` is wired as an alias for `div0` because that is what the
LocalHostName actually returns on Maciej's laptop.

The `VIBECRAFTED_THEME` env var bypasses host detection outright, so an
operator can pin a fleet baseline theme even when running on a mesh host.

### Activating the host theme

The shipped `config.kdl` defaults to the neutral `vibecrafted` theme so a fresh
install looks the same on every machine. To activate the host accent, wire one
of the following in your shell init or in a host-local `config/zellij/local.kdl`
overlay:

```bash
# Shell init — print the matching theme name for diagnostics.
~/.config/vetcoders/frontier/zellij/auto-theme.sh
```

or pin via env:

```bash
export VIBECRAFTED_THEME="$(~/.config/vetcoders/frontier/zellij/auto-theme.sh)"
```

When the operator-facing launcher in a future plan rewrites the theme line on
session start, all five layouts will pick up the host accent automatically.

## Verification

```bash
make test-zellij
```

Runs `tests/zellij_layouts_smoke.sh`, which asserts:

- every shipped layout parses via `zellij --layout <name> setup --check`
- every mesh theme loads alongside `config.kdl` without parse errors
- `aicx-status.sh`, `loctree-drift.sh`, `auto-theme.sh` pass `bash -n` and
  shellcheck (when installed)
- the two status helpers emit recognizable status lines in oneshot mode
- `auto-theme.sh` maps `dragon|sztudio|silver|div0|mgbook16` to the right
  mesh theme and falls back to neutral for unknown hosts (case-insensitive,
  `.local` suffix tolerant)

Tolerant of missing `zellij` / `shellcheck` — those checks are deferred to CI
when the host doesn't have them.

## Living Tree etiquette

- Layout edits are **append-only**. Existing pane configurations were preserved
  byte-for-byte; only the new status row was inserted into `default_tab_template`.
- Helper scripts probe multiple roots
  (`$VIBECRAFTED_HOME/tools/vibecrafted-current/config/zellij`,
  `$VIBECRAFTED_ROOT/config/zellij`, `./config/zellij`) so they work whether
  invoked from the installed framework, a Living Tree worktree, or a CI runner.
- A missing helper does **not** kill the layout — the pane prints a single-line
  diagnostic and sleeps, so the workspace stays usable.

## Related

- Kronika 2026-05-05 — VetCoders mesh topology + per-host color assignments
- Kronika 2026-04-12 — first zellij landing
- `docs/plans/META_22_SCAFFOLD_TO_RELEASE.md` Plan 12 — full contract
- `skills/vc-agents/SKILL.md` — operator-facing dispatch surface

Vibecrafted with AI Agents (c)2024-2026 LibraxisAI
