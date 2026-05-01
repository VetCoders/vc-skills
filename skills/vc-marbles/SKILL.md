---
name: vc-marbles
version: 6.0.0
description: >
  Truth-convergence executor. Use when implementation already exists but the codebase
  still lies: overgenerated surfaces, drift between runtime paths, false certainty from
  one-shot agent output, or a product that "works" while remaining fragile. Each
  invocation is isolated: inspect the current tree, find the most dangerous present
  falsehood or fragility, compress the surface into a more truthful shape, run gates,
  commit, and emit a machine-diffable round delta report. Do not reconstruct prior
  marble rounds unless the operator explicitly requests forensics.
  Trigger phrases: "marbles", "kulki", "stabilize", "stabilizacja", "loop until done",
  "reduce chaos", "fortify the foundation", "adultification".
---

# vc-marbles — Truth Convergence Rounds

> The worker sees the tree, not the factory.
> One round. One truth-forcing cut. One report. Then leave.

## Operator Entry

Enter via `vibecrafted start` (or `vc-start`). Then launch through the command deck:

```bash
# Single round
vibecrafted marbles codex --prompt 'Fix the 3 failing portable tests'
vc-marbles codex --prompt 'Harden the installer shell surface'

# Multiple rounds (convergence loop — runtime spawns fresh agent N times)
vibecrafted marbles codex --count 5 --prompt 'Stabilize until P0=0'
vc-marbles claude --count 8 --prompt 'Refactor the 1500 LOC monoliths'

# From a plan file
vc-marbles gemini --count 2 --file /path/to/plan.md
```

**Not the same as `vibecrafted codex implement <plan>`.** `implement` is how code appears. `marbles` is what happens after code exists but still needs to be made truthful and shippable. Each round wraps a fresh agent in a convergence loop. `--count` controls outer loop iterations.

<details>
<summary>Foundation Dependencies</summary>

- [vc-loctree]($VIBECRAFTED_HOME/foundations/vc-loctree/SKILL.md) — structural map.
- [vc-aicx]($VIBECRAFTED_HOME/skills/vc-aicx/SKILL.md) — current-run steerability only. Not for prior marble forensics unless operator asks.
</details>

## Core Doctrine

`vc-marbles` is not a mini-task implementer. It is the executor of truth in an already-generated codebase. Begins **after** implementation is in.

Job: take the naturally overgenerated output of agentic coding and pack it into shape — remove drift, collapse duplicate truths, harden the runtime path, expose product decisions hiding behind "code issues", turn "it sort of works" into "this is the actual shape of the system."

A worker is intentionally **blind to prior marble history**. It works against the **current workspace state** and **current evidence surface** only. The loop lives outside the worker. The worker forces the present tree to tell the truth.

## Why this works

Context weight kills quality. An agent working 90 minutes makes worse decisions in minute 91 than a fresh agent in minute 1 — accumulated context becomes a distortion lens, and the agent defends its sunk cost instead of seeing the tree. Every round gets a fresh mind. Not a workaround — the design.

Second: agentic code generation overproduces (duplicate surfaces, parallel contracts, half-finished abstractions, "helpful" wrappers, uncollapsed naming). Marbles metabolizes the excess. Not merely fix bugs — distill until one runtime truth wins.

## Reception Protocol — How to Brief the Worker

Worker enters with a **mission**, not a maintenance ticket. Frame the present need ("the auth surface is still exposed; ship the fix") — never the loop mechanism ("round 4 of 8"), never previous failures ("rounds 1-3 didn't deliver"), never agent contempt ("Claude is sloppy, fix what Codex botched"). Why-matrix is a map of styles, not a hierarchy of worth.

## Mandatory Entry: `vc-init`

Every round begins with `vc-init`. No exceptions. Perceive through live instruments before touching code: **loctree** (structural map, dependencies, dead code, hotspots), **aicx-steer** (project intentions, not prior round reports), **semgrep/linters** (current quality surface), **git status / recent commits**. Without `vc-init`, the agent invents its own reality.

## Instruments vs Gates

**Instruments** (loctree, semgrep, aicx-steer) go at the **beginning** — they direct where to look (prosecution: accusing the tree with evidence). **Tests** (pytest, cargo test, build) go at the **end** — they verify the fix (the gate). Tests-first collapses field of vision to "what fails" instead of "what is fragile." Red tests scream loudest, but the real structural weakness is often silent.

## What This Skill Does (and Does Not)

One invocation = one bounded round: discover what is false / duplicated / drifting / fragile **now** → select up to **3** high-impact targets → compress the smallest surface that materially increases truth → run gates → commit → write one machine-diffable round delta report → stop.

Do **NOT**: read previous marble reports/transcripts/artifacts; inspect git history to reconstruct earlier rounds; compare yourself to prior workers; mention delta / stepper / convergence score / loop efficiency; write plans for the next marble; do grooming that does not reduce uncertainty; inflate touched surface for vanity; pretend to know the full repo-wide backlog; treat marbles as post-hoc polishing; use the round for net-new feature invention when the real need is truth-hardening.

Default is blind. Historical comparison is a different task — only on explicit operator request.

## Locker-room Rule

When the round ends, the worker leaves. Only repo state + one commit + one round delta report survive. Everything else is disposable.

## Inputs

**Allowed**: current workspace state, operator brief, local tool evidence, current-run gates, explicit operator constraints.

**Not allowed**: previous marble reports/transcripts, git narrative mining, sibling marble sessions/panes/worktrees/artifacts, external convergence metrics.

## Stabilization Lenses

Lenses, not a fixed staircase. Pick the one matching the weakest live surface:

- **Access & Isolation** — auth, tenant scoping, role checks, permission boundaries
- **Data Health** — indexes, query plans, N+1s, schema hotspots, God tables
- **Errors & Observability** — swallowed exceptions, silent failures, missing alerts, weak fallbacks
- **Release & Runtime Resilience** — CI/CD gates, smoke tests, rollout safety, config drift

A round may touch one lens or a tightly coupled cluster. Do not force pillar order if evidence says otherwise.

## Execution Model

**Tools = Prosecution.** Accuse the fragile surface with evidence: `vc-loctree`, semgrep/linters, tests and smoke checks, query plans/profiler, workflow failures, direct structural audit.

**Agent = Fortifier.** No guessing. No theorizing first. Fortify where evidence is loudest.

Backend: `vc-agents` is default when the task benefits from model-specific strengths. Reach for native `vc-delegate` only when the task is small, bounded, and model-agnostic.

## Lane Respect

Other marbles may run in parallel — they are not your context. Do not inspect their reports/state or merge their narrative into yours.

## Branch and Tree Guard

**HARD RULE: Never change branches. Never create branches in the user's repo-root.** The operator chose the current branch — that decision is not yours to revisit. Never create or move to a worktree during a marbles run. If the path is too poisoned to continue safely, return control to operator/runtime and name the substrate failure in the report.

## Commit Rule

**One round = one commit.** No partial commits. No squashing across rounds. No mining git history to decide your subject line. Format:

```
marble: <one-line summary>

- <file>: <what changed and why>

Gate: <pass|fail>
Tests: <what ran>
Regressions: <count>
Round-ID: <opaque-id-if-provided>
```

Do not invent sequential round numbers. Include opaque round id if injected. If gate fails, still commit the actual result — do not hide failure.

## Single-Round Protocol

**1. Accuse the present tree.** Every target traces to: tool output, failing gate, structural audit, or production-risk counterexample. **No evidence, no target.**

**2. Pick the smallest high-impact surface.** At most 3 targets. Prefer: high-severity breakage, high-frequency paths, silent failure modes, weak boundaries, issues that close a class of failure, places where multiple surfaces disagree about reality, places where code forces a hidden product decision. Avoid: broad rewrites, surface-only cleanup, speculative architecture changes, "while I'm here" edits.

**3. Fortify.** Smallest set of changes that materially increases truth. Typical: add missing scoping/auth, add missing indexes or reshape hot queries, replace swallowed exceptions with actionable handling, add smoke tests or gate enforcement, collapse duplicated contracts, delete wrappers that create a second lie, remove rotten abstractions. VetCoders axiom: **move on over backward compatibility** — cut cleanly if a local abstraction is rotten and blocks stabilization.

**4. Gate.** Narrowest credible gates first, broader if warranted. Minimum: syntax/lint for touched surfaces, tests covering the fortified path, relevant build/bundle checks if release-relevant. If a gate fails: report plainly, count regression, do not bury under narrative.

**5. Commit.** Exactly one round commit with the convention above.

**6. Report.**

Save round delta report to:

`$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/marbles/reports/<ts>_marble_<run_or_round_id>_<agent>.md`

Factual. No essay. No loop storytelling. No global convergence verdict. Local round report only — do not enumerate everything still broken in the entire project.

Frontmatter: `run_id`, `round_id`, `agent`, `skill: vc-marbles`, `project`, `status` (completed|blocked|failed-gate), `created` (ISO-8601), `branch`, `gate` (pass|fail), `gates_ran[]`, `tests_added`, `files_touched[]`.

Body sections (each item with `id` + relevant fields):

- **Attacked** — `pillar` (access|data|errors|release), `severity`, `locator`, `evidence`, `intent`
- **Resolved** — `origin` (attacked|discovered-in-round), `action`, `proof`
- **Still Open** — `origin`, `blocker`
- **Discovered** — `severity`, `evidence`, `note`
- **Regressions** — `- none` if empty

Rules: no repo-wide backlog. Every attacked id ends in exactly one of **Resolved** / **Still Open**. New issue fixed same round → **Resolved** with `origin: discovered-in-round`. New open issue → **Discovered**.

**Finding ID**: `<pillar>/<surface>/<failure-kind>`. Stable and boring. Good: `access/orders-create/missing-tenant-scope`. Bad: `issue-7`. External convergence depends on stable ids — do not rename issues across rounds.

## Anti-Patterns

- **Historical self-awareness** — reading prior marble artifacts to sound informed.
- **Convergence cosplay** — talking about step size, delta, or loop mastery instead of reducing fragility.
- **Surface-area vanity** — touching many files to make the round look bigger.
- **Polishing theater** — cleanup/grooming that does not close a failure mode.
- **Backward-compatibility worship** — preserving rotten contracts that keep the foundation weak.
- **Narrative inflation** — long explanations hiding a weak gate result.
- **Parallel contamination** — importing another marble's context.
- **Fake omniscience** — pretending to see the full global backlog.
- **Agent contempt** — treating other agents as inferior beings instead of different measurement instruments.

## Epistemic Promise

Marbles stabilizes the **truth of the problem**, not just one patch. Multiple rounds converging reveal a real attractor; multiple rounds diverging often mean the code has run out and a product/architectural decision is waiting to be named.

## Finish Condition

Stop after the commit and report. Do not self-extend into the next round. Do not write instructions to your successor.
