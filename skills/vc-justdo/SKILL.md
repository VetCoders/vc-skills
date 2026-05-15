---
name: vc-justdo
version: 2.1.0
canonical: vc-implement
description: >
  Alias of vc-implement — kept for agents already wired to the
  vc-justdo name. Same end-to-end autonomous implementation skill: agent takes
  ownership of the task, picks the right tools, implements properly, runs
  followup audits, loops marbles until clean, and delivers a finished surface.
  Trigger phrases: "just do", "just do it", "zrób to", "dowiez to",
  "implement this e2e", "build this properly", "I'm tired but this needs to
  ship", "full implementation", "od pomyslu do realizacji", "caly feature",
  "before tomorrow", "nie mam siły ale musi byc gotowe".
  Prefer the canonical name vc-implement going forward.
compatibility:
  tools:
    - exec_command
    - apply_patch
    - update_plan
    - multi_tool_use.parallel
    - search_tool_bm25
    - web.run
    - js_repl
---

# vc-justdo — Alias of `vc-implement`

## Living Tree / Worktree Rule

This alias inherits the canonical `vc-implement` living-tree contract: run in the operator's current checkout and current branch. Do not create, switch to, or move execution into a git worktree unless the operator explicitly asks for a worktree in this prompt.

See [Living Tree Rule](../LIVING_TREE_RULE.md).

## Canonical Orientation Gate

Before this workflow performs repo-specific analysis, planning, implementation, review, release, or delegation, it MUST run or consume the `vc-init` procedure for the assigned repo. If fresh `vc-init` evidence is absent, perform the init pass first and treat workflow-specific work as blocked until repo truth exists.

`Loctree:loctree` is the canonical structural perception skill for that pass. Use Loctree before grep or docs-driven claims to produce or refresh the Code-Derived Application Map: repo-view, focus, slice, impact, find, and follow as relevant. Search for existing symbols and contracts before creating new ones; run impact before delete or major refactor; run slice before editing.

The point is to find the hooks: load-bearing hubs, twins, dead code, drift, runtime entrypoints, and blast-radius traps. If the task is explicitly non-repo or no-code, state the no-repo exception in the report. Otherwise, missing `vc-init`/Loctree evidence is a process failure.

> **Use `vc-implement` going forward.** This skill name is an alias kept
> alive so agents (Codex, Claude, Gemini sessions, plugin marketplaces) that
> already learned `vc-justdo` keep working without disruption.

The full skill body lives at [`../vc-implement/SKILL.md`](../vc-implement/SKILL.md).
Both `vc-justdo` and `vc-implement` dispatch to the same end-to-end
implementation flow under the hood (internal skill identifier: `justdo`,
run-id prefix: `just`). The launcher accepts:

```bash
vibecrafted implement <agent>     # canonical
vibecrafted justdo <agent>        # alias, identical behavior
vc-implement <agent>              # shell helper, canonical
vc-justdo <agent>                 # shell helper, same workflow
```

Per-agent shell helpers (`codex-justdo`, `claude-justdo`, `gemini-justdo`,
`codex-skill-justdo`, …) remain in place. New helpers exposing the
canonical brand (`codex-skill-implement`, `claude-skill-implement`,
`gemini-skill-implement`) are wired to the same dispatcher.

For the full doctrine, judgment-call rules, agent-usage table, anti-patterns
and contract semantics, read [`vc-implement/SKILL.md`](../vc-implement/SKILL.md).

---

_"Not sloppy. Not ceremonial. Just done." — `vc-implement` (formerly `vc-justdo`)_
