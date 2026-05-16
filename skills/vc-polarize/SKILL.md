---
name: vc-polarize
version: 1.0.0
description: >
  Post-marbles concept and product truth polarization skill.
  Use when a concept or product surface has high prism score: the implementation
  may work, but its truth is smeared across runtime paths, tests, docs,
  artifacts, public promises, or operator memory. Polarize chooses one axis,
  rejects competing truths, aligns surfaces, and emits DoU/release handoff.
  Trigger phrases: "polarize", "vc-polarize", "wyostrz", "one sharp truth",
  "code smear", "prism score", "after marbles", "choose one axis".
requires:
  - stable loctree CLI/MCP prism support
---

# vc-polarize — One Sharp Truth After Marbles

`vc-polarize` is a default workflow running on top of Loctree's `loct prism` code-smear and context-pack analysis.

Use Loctree to produce and compare context packs for one concept across task framings:

```bash
loct prism --task "auth" --task "auth core" --task "auth core portal"
```

## Purpose

`vc-polarize` runs after `vc-marbles` or after a `loct context` prism pack
shows high conceptual smear.

`vc-marbles` establishes **Code Truth** (P0=0, P1=0, P2=0) by asking:

> What is still technically false, fragile, or untested in the implementation?

`vc-polarize` establishes **Product Truth** by asking:

> Which one concept or product boundary should now become authoritative?

The job is to collapse ambiguity into an explicit contract:

- one boundary
- one owner
- one runtime proof path
- one artifact/report truth
- one public promise when the concept reaches users
- explicit rejected alternatives

## Core Vocabulary

### Code Smear

A runtime or product concept whose truth is spread across many files, layers,
docs, tests, artifacts, public surfaces, and operator memory.

Smear is not automatically bad. It becomes dangerous when the spread creates
competing truths.

### Prism Score

A diagnostic and context-worthiness score for how much a concept refracts across
different `loct context --task` framings.

High prism score means:

- one local slice will mislead future agents
- the concept deserves a corpus entry
- a polarization pass may be needed before release or further implementation

It does **not** mean:

- bad code
- CI failure
- shame KPI
- file-count penalty

### Polarization

The opposite motion from smear. Pick one axis/facet and make runtime, tests,
docs, artifacts, and public surfaces agree.

## Modes

### Concept Mode

Use when an architectural/runtime concept is smeared.

Examples:

- marbles lifecycle
- release surface
- installer public contract
- research swarm synthesis
- auth boundary
- memory/search context

Output: one concept contract and proof path.

### Product Mode

Use when a product/public surface is smeared.

Examples:

- too many audiences
- split CTA
- landing page says one thing while install path says another
- docs overpromise runtime
- release brief lacks one shippable thesis

Output: one product thesis and DoU/release handoff.

## Pipeline Position

```text
scaffold -> init -> workflow -> implement -> followup -> marbles -> [POLARIZE] -> dou -> decorate -> hydrate -> release
```

`vc-polarize` is allowed to read recent marbles reports. This is deliberate.
`vc-marbles` workers should stay present-tense and avoid prior loop history;
`vc-polarize` is a synthesis step after convergence evidence exists.

## Operator Entry

### Living Tree / Worktree Rule

This workflow runs in the operator's current checkout and current branch. Do not create, switch to, or move execution into a git worktree unless the operator explicitly asks for a worktree in this prompt. Generic words like "isolate", "parallel", or "clean branch" are not enough. Re-read files before editing, adapt to concurrent changes, and report a substrate failure if the current tree is too poisoned to continue safely.

See [Living Tree Rule](../LIVING_TREE_RULE.md).

## Canonical Orientation Gate

Before this workflow performs repo-specific analysis, planning, implementation, review, release, or delegation, it MUST run or consume the `vc-init` procedure for the assigned repo. If fresh `vc-init` evidence is absent, perform the init pass first and treat workflow-specific work as blocked until repo truth exists.

`Loctree:loctree` is the default structural perception skill for that pass. Use Loctree before grep or docs-driven claims to produce or refresh the Code-Derived Application Map: repo-view, focus, slice, impact, find, and follow as relevant. Search for existing symbols and contracts before creating new ones; run impact before delete or major refactor; run slice before editing.

The point is to find the hooks: load-bearing hubs, twins, dead code, drift, runtime entrypoints, and blast-radius traps. If the task is explicitly non-repo or no-code, state the no-repo exception in the report. Otherwise, missing `vc-init`/Loctree evidence is a process failure.

Launch through the command deck:

```bash
vibecrafted polarize codex --task 'marbles versus polarize skills: polarize them'
vc-polarize codex --task 'installer public contract'
vc-polarize claude --file /path/to/prism-pack.md
vibecrafted polarize gemini --prompt 'Choose one launch thesis after marbles'
```

When `--task` is present, the runner must execute a fresh prism preflight before
choosing the execution path. The runner reads Loctree's `band_action` from the
prism JSON, falls back to `total_score` for older payloads, and only dispatches
an agent for `pass` and `doctrine` bands:

```bash
loct prism --with-aicx --task '<operator task>' --task '<operator task> code truth' --task '<operator task> product truth' --json
```

`--with-aicx` is the default prism mode. Operators may pass `--no-aicx` only
when they explicitly need a repo-only prism pack.

Operators may pass `--no-context-corpus` to skip optional retention pack
emission in environments without AICX extraction.

No `--count`. No convergence loop. This is not another marbles engine.

## Inputs

Allowed:

- current workspace truth
- fresh `vc-init` / loctree / AICX evidence
- prism packs generated by `loct context --with-aicx --task ...`
- recent marbles reports
- DoU/release/hydrate/decorate reports when relevant
- README, quickstart, installer, marketplace, command help, release surfaces
- operator constraints: buyer, launch channel, forbidden claims, desired boldness

Not allowed:

- unsupported claims
- averaging competing directions
- treating prism score as a shame metric
- using stale context packs as current code truth
- scoring file count instead of authority drift

## Prism Score

Use the stable Loctree scorer to evaluate prism score.

Score each axis from `0` to `3`, total `0..15`:

| Axis                | Meaning                                                                                                                       |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Spread              | How many surfaces carry the concept: code, shell env, tests, docs, artifacts, AICX, public copy.                              |
| Runtime Centrality  | Whether the concept sits on entrypoint, installer, marbles lifecycle, research lifecycle, release, or operator control plane. |
| Authority Diversity | Whether evidence comes from RepoVerified code, LoctreeDerived structure, tests, AICX/operator memory, public docs.            |
| Drift Risk          | Likelihood that surfaces disagree or stale artifacts create false truth.                                                      |
| Closure Evidence    | Whether gates, tests, reports, state files, launch cards, or artifacts make closure machine-checkable.                        |

Bands:

- `0..4`: `abort` — stop before agent dispatch and show the prism JSON path.
- `5..8`: `memo` — emit a local memo and thin context-corpus example only.
- `9..12`: `pass` — run the full polarize pass with the prism payload injected.
- `13..15`: `doctrine` — run the full doctrine pass with a regression-contract expectation.

## Outputs

Recommended artifacts:

```text
$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/polarize/
  thesis.md
  concept-contract.md
  surface-map.json
  decision-ledger.md
  dou-handoff.md
  release-brief.md
```

If runtime has no dedicated `polarize/` artifact directory yet, write only the
normal injected report and include these sections inside it.

## Report Sections

- **Polarized Thesis** — one sentence, no hedge words.
- **Mode** — `concept` or `product`.
- **Prism Evidence** — task framings, context pack paths, score axes.
- **Primary Boundary / Audience** — what wins now.
- **Rejected Alternatives** — what loses now, and why.
- **Runtime Proof** — concrete repo/runtime evidence.
- **Surface Alignment** — current claim/path -> problem -> chosen replacement.
- **Edits Made** — if implementation was in scope.
- **Gates Run** — commands, exit status, and what they prove.
- **DoU Handoff** — what DoU should audit next.
- **Release Handoff** — what release can honestly ship and what remains blocked.

## Lifecycle

1. **Perceive**
   Run repo truth checks. Confirm branch, dirty tree, current head, and whether
   prism packs are stale against live code.

2. **Collect**
   Read prism packs, latest relevant marbles reports, public/install/release
   surfaces, and command help.

3. **Score**
   Use the stable Loctree scorer to evaluate prism score.

4. **Choose**
   Pick one axis to make authoritative. Reject competing axes explicitly.

5. **Align**
   Edit only surfaces supported by runtime proof. In concept mode, prefer
   contracts, tests, docs, and artifact schemas. In product mode, align public
   copy, CTA, onboarding, and release thesis.

6. **Gate**
   Run narrow gates for touched surfaces and broader gates if release-facing.

7. **Emit**
   Write polarization artifacts and handoffs.

8. **Stop**
   Do not continue into DoU, hydrate, decorate, or release unless the operator
   asked for that chain.

## Context Corpus Contract

The runner is the producer for prism context-corpus packs. For `pass` and
`doctrine`, it captures the dispatched agent's `session: <uuid>` stdout line and
wraps the default extractor:

```bash
aicx extract --agent <agent> --session <uuid> --output <raw-path>
```

Do not regenerate content from scratch, rewrite raw Markdown packs, or mix them
into ordinary AICX conversation chunks. For `memo`, the runner writes only a
thin local memo and sidecar. For `abort`, it writes no context-corpus pack
because no useful truth was produced.

Suggested retention path:

```text
$HOME/.aicx/context-corpus/<org>/<repo>/<YYYY_MMDD>/loct-context-pack/<batch>/
  raw/
  sidecars/
  index.jsonl
```

Sidecars must mark stale-vs-live truth:

```json
{
  "schema_version": "context_corpus.v1",
  "artifact_family": "loct-context-pack",
  "truth_status": {
    "role": "example",
    "runtime_authoritative": false,
    "stale_against_current_head": false,
    "current_head_when_ingested": "ded1e0b"
  },
  "learning_use": {
    "allowed": ["format_examples", "section_order", "keyword_index"],
    "forbidden": ["current_code_truth", "implementation_claims", "gate_status"]
  },
  "keywords": ["installer", "contract"],
  "band": "pass",
  "total_score": 11
}
```

Memo-band sidecars restrict `learning_use.allowed` to `["format_examples"]`.

Runtime truth must always come from fresh `loct context`, fresh repo reads, and
relevant gates.

## Minimum Gates

Read-only / doctrine-only:

```bash
git status --short --branch
```

Docs or skill changes:

```bash
git diff --check
```

Command deck/help changes:

```bash
pytest tests/tui/test_vibecrafted_launcher.py -q
```

Installer/release-facing changes:

```bash
make check
make semgrep
```

Corpus sidecars:

```bash
python3 -m json.tool <sidecar>.json
```

## Failure Modes

- **No prism evidence**: ask for a `loct prism --task ...` pack.
- **Prism band too low**: abort before agent dispatch and print `vc-polarize aborted: prism score <n>/15 is below threshold. Inspect <path-to-prism.json>`.
- **Runtime cannot support desired promise**: send back to `vc-workflow` or `vc-marbles`.
- **Two viable axes remain**: emit decision memo; do not average them.
- **Public surfaces contradict release truth**: block release handoff.
- **Context pack is stale**: use it as corpus/example evidence only, then regenerate fresh context.

## Anti-Patterns

- "Everyone can use it."
- One more wrapper instead of one contract.
- Changing copy without runtime proof.
- Scoring file count instead of authority drift.
- Treating prism score as CI failure.
- Treating old context packs as live code truth.
- Hiding product choice behind technical cleanup.
