---
name: vc-agents
version: 3.1.0
description: >
  Spawn external specialized AI agents from the user's fleet (Codex, Claude, Gemini).
  Use this when you need parallel execution, deep isolation, or task-specific cognitive 
  strengths that surpass generic in-thread delegation.
  Trigger: "vc-agents", "/vc-agents", "delegate to agents", "spawn".
---

# vc-agents — The External Execution Fleet

## Operator Entry

### Living Tree / Worktree Rule

This workflow runs in the operator's current checkout and current branch. Do not create, switch to, or move execution into a git worktree unless the operator explicitly asks for a worktree in this prompt. Generic words like "isolate", "parallel", or "clean branch" are not enough. Re-read files before editing, adapt to concurrent changes, and report a substrate failure if the current tree is too poisoned to continue safely.

See [Living Tree Rule](../LIVING_TREE_RULE.md).

Operator enters the framework session through:

```bash
vibecrafted start
# or
vc-start
# same default board as: vc-start operator
```

`vc-agents` is the delegation contract behind active workflows, not the primary
operator command a founder types first. The operator-facing entrypoint stays:

```bash
vibecrafted <workflow> <agent> \
  --<options> <values> \
  --<parameters> <values> \
  --file '/path/to/plan.md'
```

```bash
vc-<workflow> <agent> \
  --<options> <values> \
  --<parameters> <values> \
  --prompt '<prompt>'
```

If `vc-<workflow> <agent>` is invoked outside Zellij, the framework will attach
or create the operator session and run that workflow in a new tab. `vc-agents`
defines how that workflow fans out into external workers.

### Concrete dispatch examples

```bash
vibecrafted codex implement /path/to/plan.md
vibecrafted claude implement /path/to/plan.md
vibecrafted gemini implement /path/to/plan.md
```

> We do not outsource thought. We deploy equally capable minds on parallel execution paths to protect the main context buffer.

A single agent session carries immense context. Attempting to execute every small rewrite, forensic deep-dive, or radical structural shift in-thread causes prompt bloat and dilutes your focus.

`vc-agents` is the external delegation layer. You identify the structural gap,
pick the right mind for the job from the **`vc-why-matrix`**, spawn the
autonomous external worker, and return to your main orchestration.

This skill is only for external workers. Native in-process delegation belongs to
`vc-delegate`, not here.

## The `vc-why-matrix`

You do not spawn agents blindly. You pick the cognitive profile required for the cut.

```mermaid
  graph TD
    subgraph Codex
        CodexDesc[Precision & Surgery]
        CodexBest[Best for:\n\n– Critical implementations\n– Exact refactors\n– Contract-gated execution]
        Codex --> CodexDesc
        Codex --> CodexBest
    end

    subgraph Claude
        ClaudeDesc[Forensics & Research]
        ClaudeBest[Best for:\n\n– Bug hunts across deep layers\n– Architecture audits\n– Assessing unknown paths]
        Claude --> ClaudeDesc
        Claude --> ClaudeBest
    end

    subgraph Gemini
        GeminiDesc[Radical Reframing]
        GeminiBest[Best for:\n\n– Architecture leaps\n– Fearless simplification\n– Stripping dead scaffolding]
        Gemini --> GeminiDesc
        Gemini --> GeminiBest
    end
```

## Delegation Doctrine

- **Delegate, do not micromanage:** Do not produce 15-point bureaucratic checklists for the spawned agent. Write a high-level plan with `Goal`, `Scope`, and `Acceptance Criteria`. Let them figure out the _how_.
- **The Living Tree:** Agents must know they operate in a live system. Ensure your spawn plan states: _"You are working on a living tree. Concurrent changes are expected. Adapt proactively."_
- **Full Replacement over Scar Tissue:** Tell your agents they are empowered to rewrite broken abstractions. Sometimes a full replacement is cleaner than patching over bad prototype code.

## Escalation Authority

`vc-agents` is an operator-level orchestration layer.

The decision to use `vc-agents` already encodes `vc-why-matrix` intent:
the operator selected a specific model family and cognitive profile for the
mission.

Because of that:

- spawned fleet agents must not call `vc-agents` again on their own
- spawned fleet agents must not re-open model selection or launch a second external fleet
- spawned fleet agents must not reinterpret the `vc-why-matrix`
- escalation into `vc-agents` belongs exclusively to the operator agent

If a spawned worker discovers that the mission surface is wider, more parallel,
or less bounded than expected, it should not self-escalate outward.

Instead it must:

- complete the assigned mission as far as honestly possible
- record the boundary it encountered
- name the unresolved surface clearly in its report
- leave any orchestration change to the operator

A fleet worker may reveal orchestration pressure.
It may not act on it.

## Plan template

```markdown
---
run_id: <generated-unique-id>
agent: <claude|codex|gemini>
skill: vc-agents
project: <repo-name>
status: <pending|in-progress|completed|failed>
loops_completed: <number>
---

# Task: <short title>

Goal:

- <1-3 bullets>

Scope:

- In scope: <files/areas> as high-level suggestions
- Out of scope: <explicit>

Constraints:

- No --no-verify
- Follow repo conventions

Acceptance:

- [ ] <objective outcome>
- [ ] <objective outcome>

Test gate:

- <command(s)>

Context:

- <very short summary>

Living tree note:

- You work on a living tree with 𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚜𝚖𝚊𝚗𝚜𝚑𝚒𝚙 methodology, so concurrent changes are expected.
- Adapt proactively and continue, but this is never permission to skip quality, security, or test gates.
- Run required checks. If something is blocked, report the exact blocker and run the closest safe equivalent.
- Coordination mode: <solo on this stage / parallel with other agents on this stage>
- You do not need to inspect other agents' plans unless this plan explicitly tells you to.
- If this plan explicitly calls for a stabilization checkpoint, commit your own changes locally without push and continue on the current branch.
- You are an execution unit, not orchestration authority: do not invoke `vc-agents`, do not reopen frontier selection, and do not reinterpret the `vc-why-matrix`.
- If the mission reveals a wider unresolved surface, report that boundary clearly and leave orchestration changes to the operator.
```

## Spawn commands

The operator-facing launch path for out-of-process delegation goes through the
`vibecrafted` command deck or the `vc-<workflow>` helper. The repo-owned spawn
scripts remain the internal engine behind that path.

### Codex

```bash
PLAN="$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/plans/<plan-slug>.md"
vibecrafted codex implement "$PLAN"
```

### Claude

```bash
PLAN="$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/plans/<plan>.md"
vibecrafted claude implement "$PLAN"
```

### Gemini

```bash
PLAN="$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/plans/<plan>.md"
vibecrafted gemini implement "$PLAN"
```

If these tools are unavailable, stop pretending spawn is correctly configured and say so explicitly.

## Output convention

- Plans: `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/plans/<timestamp>_<slug>.md` or another stable per-task
  filename
- Reports: `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/reports/<timestamp>_<slug>_<agent>.md`
- Transcripts: `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/reports/<timestamp>_<slug>_<agent>.transcript.log`
- Metadata: `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/reports/<timestamp>_<slug>_<agent>.meta.json`

Every spawn should surface a launch card immediately after dispatch.
That card should expose at least:

- `run_id`
- chosen agent / model family
- plan path
- report path
- transcript path
- metadata path

If the operator cannot see those paths, observability is incomplete even if the
agent is technically running.

## Observation

Observe progress through durable artifacts in `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/reports/`.

The default check is metadata-first, not pane-first.
Use the dedicated runtime helper to wait on metadata completion and print the
final summary:

```bash
vibecrafted codex await --run-id <run_id>
```

For the most recent run of a given agent:

```bash
vibecrafted codex await --last
```

For multiple spawned workers, pass their launcher or metadata paths directly to
the helper and let it wait on all of them together.

If your environment exposes the observer helper, use it for transcript-level
inspection or debugging:

```bash
vibecrafted codex observe --last
```

Use the equivalent agent observer when needed, but do not rely on `observe` as
the only status surface. `vc-agents` should remain operable from durable
artifacts even when the operator is not staring at the live panes.

## Quality gate expectations

Keep the standard 𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍. quality bar:

- loctree-mcp as first-choice exploration and search tool with fail-fast if inaccessible
- semgrep as first-choice security guard when available
- Rust repos: `cargo clippy -- -D warnings`
- Non-Rust repos: choose the closest equivalent lint/type/test gate
- Tests: run if reviewing; write if implementing new behavior; prefer real e2e coverage for the actual pipeline
- If a gate is blocked, report the exact blocker and run the closest safe equivalent

## Safety rules

- Do not log secrets or commit `.env` files.
- Never use `--no-verify` for `commit` or `push`.
- Do not rewrite git history unless the user explicitly asks.
- Treat concurrent edits as normal, but still verify before overwriting.
- If a repo has a strict command such as `make check`, run it or explain why not.

## Final principle

Fleet is not for outsourcing thought.
Fleet is for deploying equally capable front-line agents through a strict, canonical launch path.
Use them to implement, not merely to comment on implementation.

## Automated model-parity enforcement (added 2026-05-12, Plan 06)

The kronika 2026-04-10 axiom — **every native delegation must pass the
parent's model tier; no Sonnet/Haiku fallback from Opus parent** — is now
an automated gate, not a prompt-review convention.

Two parallel implementations enforce it:

- Bash: [`scripts/lib/spawn.sh`](../../scripts/lib/spawn.sh) — source it
  from any bash spawn primitive to gain `spawn_detect_parent_model`,
  `spawn_check_parity`, and `spawn_require_parity`.
- Python: [`vibecrafted-core/vibecrafted_core/agent_dispatch.py`](../../vibecrafted-core/vibecrafted_core/agent_dispatch.py)
  — for vibecrafted-mcp and any Python dispatch primitive. Exposes
  `detect_parent_model`, `check_parity`, `require_parity`, and
  `ParityError`.

Both layers share the same tier model:

| Family    | Tiers (high -> low)                                            |
| --------- | -------------------------------------------------------------- |
| Anthropic | `opus` > `sonnet` > `haiku`                                    |
| Codex     | `gpt-5.3` > `gpt-5` > `spark` > `gpt-4`                        |
| Gemini    | `gemini-3-pro` > `gemini-3-auto` > `gemini-3-flash` > `gemini` |

### Rules encoded in the gate

- **Same-family, child rank >= parent rank** -> allowed.
- **Same-family, child rank < parent rank** -> rejected with diagnostic
  citing the kronika axiom and the override env var.
- **Cross-family** (e.g. Opus parent -> Codex child) -> allowed; the
  operator made an explicit `vc-why-matrix` selection, that is a
  different cognitive profile rather than a downgrade.
- **Unrecognized inputs** -> rejected; operator must classify the new
  model explicitly.

### Operator override (documented exceptions only)

For the exceptions named in `vc-delegate` (Codex Spark for extreme speed,
Gemini `auto-3` fallback during peak demand), set the env var:

```bash
VIBECRAFTED_SPAWN_ALLOW_DOWNGRADE=1 <spawn command>
```

The override path emits a stderr/log warning so the downgrade stays
auditable. The gate stays strict by default — Spark/Auto-3 are not
silently allowed, the operator must opt in per invocation.

### Parent-model detection

Bash and Python both probe the same env-var ladder in order:

1. `VIBECRAFTED_PARENT_MODEL` — operator-explicit override.
2. `CLAUDE_MODEL`
3. `CODEX_MODEL`
4. `GEMINI_MODEL`

If none is set, callers should safe-reject (parity unknown) rather than
assume parity is fine.

### Verification

```bash
make test-parity
```

Runs the bash suite (`tests/spawn_parity_test.sh`) and the pytest mirror
(`tests/agent_dispatch_test.py`) together.
