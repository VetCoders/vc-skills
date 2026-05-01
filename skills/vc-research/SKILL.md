---
name: vc-research
version: 1.3.0
description: >
  Standalone triple-agent research skill. Co-define the problem with the user,
  write a research plan, then spawn claude + codex + gemini simultaneously on the
  same questions. Three independent reports come back. Synthesize into one
  gap-free research document ready for implementation. Use whenever the team
  needs ground truth before coding: unknown APIs, architecture decisions, library
  assessment, protocol research, best-practice survey, competitive analysis,
  or any situation where one agent's perspective is not enough. Trigger phrases:
  "research this", "zbadaj to", "triple research", "research swarm", "3 agenty
  research", "gap-free research", "zbadaj przed implementacją", "co mówi
  dokumentacja", "state of the art", "SoTA research", "porównaj podejścia",
  "analyze options", "research plan", "plan researchu".
compatibility:
  tools:
    - Bash
    - Read
    - Write
    - Agent
---

# vc-research — Triple-Agent Research Swarm

## Operator Entry

Operator enters the framework session through:

```bash
vibecrafted start
# or
vc-start
# same default board as: vc-start operator
```

Then launch this workflow through the command deck, not raw `skills/.../*.sh` paths:

```bash
vibecrafted <workflow> \
  --<options> <values> \
  --<parameters> <values> \
  --file '/path/to/plan.md'
```

```bash
vc-<workflow> \
  --<options> <values> \
  --<parameters> <values> \
  --prompt '<prompt>'
```

If `vc-<workflow>` is invoked outside Zellij, the framework will attach
or create the operator session and run that workflow in a new tab. Replace
`<workflow>` with this skill's name. Prefer `--file` for an existing plan or
artifact and `--prompt` for inline intent.

### Concrete dispatch examples

```bash
vibecrafted research --prompt 'Compare auth libraries for Tauri desktop'
vc-research --prompt 'State of the art for MCP streaming transports'
vibecrafted research --file /path/to/research-plan.md
```

<details>
<summary>Foundation Dependencies (Loaded with framework)</summary>

- [vc-loctree](../foundations/vc-loctree/SKILL.md) — primary map and structural awareness.
- [vc-aicx](../foundations/vc-aicx/SKILL.md) — primary intentions and steerability index.
</details>

> One perspective is an opinion. Three perspectives are evidence.

## Purpose

Research a problem from three independent angles before writing a single line of
code. The orchestrating agent (you) co-defines the problem with the user, writes
a plan, spawns claude + codex + gemini on the same questions, then synthesizes
their findings into one gap-free research document.

This is the Research phase from vc-workflow, extracted as a standalone
skill and upgraded with triple-agent triangulation.

## When To Use

- Unknown API, protocol, or library
- Architecture decision with multiple valid approaches
- "What is the current best practice for X?"
- Library assessment (A vs B vs C)
- Integration research (how does X talk to Y?)
- Any moment where guessing would be cheaper than being wrong

Do NOT use for:

- Questions answerable by reading one file in the repo
- Problems where loctree slice + grep gives the answer in 30 seconds
- Pure implementation tasks (use vc-workflow, usually through vc-agents; use vc-delegate only for small model-agnostic
  work)

## Research Safety

Research mode is read-only for the source repository.

- **Closure marker = filesystem artifacts**, not git. The run directory under
  `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/research/<run_id>/`
  with report.md + meta.json + transcript.log is the deterministic anchor.
  Operator verifies completion via `ls`, `cat meta.json | jq .status`, etc.
  No git commits needed.
- **No source mutation.** Do not edit repo source files, config, `.gitignore`,
  generated files, or cleanup stray files unless the operator plan explicitly
  asks for source modifications.
- **No git writes.** Do not stage, commit, amend, tag, branch, merge, rebase,
  push, stash, clean, reset, checkout, switch. Working tree must be unchanged
  at the end of the run. Empty commits, `--allow-empty`, chore stamps —
  forbidden across the board.
- If research discovers an obvious fix, write the proposed fix, exact file
  references, and implementation notes to the report artifact instead of
  applying it.
- Reports and plans go under the run-scoped research directory:
  `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/research/<run_id>/`.
- Codex workers must write the full markdown report to the given report path
  through the filesystem before exiting. The `codex exec --output-last-message`
  final message is only a completion note, not the durable research report.

## The 6-Step Research Flow

### Step 1 — Co-define the problem

Talk with the user. Do not write a plan yet. Establish:

- **What we need to know** — the actual question, not the symptom
- **Why we need to know it** — what decision depends on this answer
- **What we already know** — priors, assumptions, prior art in the repo
- **Boundaries** — what is out of scope for this research

Output: a short problem statement (3-5 sentences) agreed with the user.

### Step 2 — Write the research plan

Create one plan file. The plan is what every agent receives. It contains:

```markdown
---
run_id: <generated-unique-id>
agent: <claude|codex|gemini>
skill: vc-research
project: <repo-name>
status: in-progress
---

# Research Plan: <title>

## Problem

<the co-defined problem statement from Step 1>

## Questions

1. <specific, answerable question>
2. <specific, answerable question>
3. ...

## Mandatory tools

- loctree MCP (repo-view, slice, find, impact) — for any codebase-related questions
- Brave Search or WebSearch — for external ground truth

## Encouraged tools (agent's choice)

- Context7 (resolve-library-id → query-docs) — for library documentation
- WebFetch — for specific URLs found via search
- Codebase grep — for internal patterns (only after loctree mapping)

## Report format

Write your findings to the report file as markdown with this structure:

### Q1: <question>

**Sources**: <URLs, docs, file refs>
**Finding**: <concise answer>
**Confidence**: high / medium / low
**Evidence**: <code snippet, quote, or data>

### Q2: ...

### Synthesis

- Recommended approach: <your recommendation>
- Alternatives considered: <with tradeoffs>
- Open questions: <what you could not answer>
- Implementation notes: <concrete guidance>

## Constraints

- Append current year to search queries for freshness
- Prefer primary sources (official docs, RFCs, source code) over blog posts
- If two sources disagree, note the disagreement explicitly
- Do not hallucinate API signatures — verify them
```

Input plans may live anywhere, but `vc-research` records the effective prompt
or plan under
`$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/research/<run_id>/plans/<ts>_<slug>_research-plan.md`.

Plans can be split if the problem has clearly separable domains. Each agent
gets ALL plans — they are independent researchers, not specialists.

### Step 3 — Spawn triple research swarm

Canonical operator-facing launch path goes through the command deck:

```bash
PLAN="$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/plans/<ts>_<slug>_research-plan.md"

vc-research --file "$PLAN"
```

The repo-owned spawn scripts remain the internal engine behind that surface. Do
not document raw `bash skills/...spawn.sh` paths as the operator entrypoint.

The launcher opens one shared Zellij research tab using `research.kdl`,
keeps a common `run_id`, and starts claude + codex + gemini against the same
plan. This is intentional — divergence between reports reveals blind spots.

Research observability is mandatory.
`vc-research` is not "running" just because three panes appeared.
Immediately after spawn, the operator should get a launch card with the shared
`run_id`, run directory, reports directory, summary path, and the exact await
command.

That launch card is the default surface.
`observe --last` is a drilldown tool, not the primary source of truth.

### Step 4 — Collect reports

Reports land in:

```
$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/research/<run_id>/reports/claude.md
$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/research/<run_id>/reports/codex.md
$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/research/<run_id>/reports/gemini.md
```

The readable launch card lives at `research/<run_id>/summary.md`. Metadata,
transcripts, raw streams, runtime prompts, launchers, and the Zellij layout stay
inside `research/<run_id>/logs/` and `research/<run_id>/tmp/` so the date-level
artifact root remains readable.

Wait for all three through the dedicated runtime helper, not by hand-rolled
snippets.
The standard operator move is:

```bash
vc-research-await --run-id <run_id>
```

If you just launched the latest research swarm and want the newest one, this is
also valid:

```bash
vc-research-await --last
```

If you need transcript-level inspection while the swarm is still running, use
the observer helpers:

```bash
vibecrafted claude observe --last
vibecrafted codex observe --last
vibecrafted gemini observe --last
```

Do not treat manual `observe --last` calls as sufficient observability for the
workflow itself. The workflow should expose its state through launch metadata,
the await helper, and durable report paths by default.

### Step 5 — Synthesize (operator's expertise with line-precise citations)

**Before you cite a single line of any source report, you MUST have read each
report in full via layered slicing.** This is non-negotiable.

Most research reports run 30-100KB. Tools have output caps that hit at
~25KB and dump the rest to a file with a "see path: ..." warning. Skipping
that file because it's "long" or working only from the warning text is
exactly the failure mode this skill exists to prevent. A synthesis built
from truncation warnings is a hallucination wearing the costume of expertise.

Procedure for each of the three source reports:

1. Read the file in full using offset/limit slicing in spans of ~1500-2000
   lines (or ~80,000 chars), until you have covered the entire file.
2. Note in your synthesis document, in section "0. Coverage statement",
   exactly how many lines / bytes you read per source report. Example:
   ```
   ## 0. Coverage
   - claude_<run_id>.md: 1842 lines, 71KB, 100% read in 2 spans
   - codex_<run_id>.md: 2106 lines, 84KB, 100% read in 2 spans
   - gemini_<run_id>.md: 1297 lines, 49KB, 100% read in 1 span
   ```
3. If a source report is too large to read in the available time/budget,
   HALT and report the boundary clearly. Do NOT proceed to synthesis. Do
   NOT cite line ranges you have not actually read. The operator will
   re-dispatch with a narrower scope or grant more time.

Only after coverage is complete do you build the synthesis below.

**Synthesis = operator's expert opinion built ON the three reports, NOT a
copy of them.**

Anti-pattern (what we explicitly leave behind):

- ❌ "patchwork meta-artifact" — verbatim concatenation of 3 reports glued
  together. Becomes 30-50KB monolith with duplicate frontmatter, hard to
  read, hard to ingest, no real synthesis. The reports already exist as
  individual artifacts; copying them adds noise, not value.
- ❌ "compressed view" — operator reads 3 reports, paraphrases to one short
  synthesis, publishes only that. Individual findings get crushed; reader
  loses the option to verify a specific claim against its source.

Mandatory pattern (operator decision 2026-05-01, after rozmowa Maciej+Monika):

- ✓ Synthesis is **a separate, concise document** that interprets the three
  reports and points to **exact lines** in them for every non-trivial claim.
- ✓ Reports stay as **individual artifacts** (`claude_<run_id>.md`,
  `codex_<run_id>.md`, `gemini_<run_id>.md`). They are immutable expert
  testimony — full content, full frontmatter, original style.
- ✓ Synthesis cites them with file:line refs (e.g.
  `claude_<run_id>.md:L42-58`). Reader who wants to verify clicks the ref
  and reads the full source paragraph. Operator does not paraphrase the
  expert; operator points at the expert.
- ✓ When reports disagree, synthesis notes the dissent **with file:line
  refs to each side** and gives the operator's reasoned judgment. The
  reader can see exactly which paragraph in which report supports each
  position.

**Final synthesis form (operator+Monika decision 2026-05-01, rev 4)** has two
distinct sections: **A. Convergent findings (deduplicated)** and **B. Signals
(single-agent findings, potentially key insights)**. Voting / majority rules
are explicitly rejected.

### A. Convergent — deduplicate, do not repeat

Findings on which two or three reports overlap get reduced to **one statement**.
The reader does not need the same obvious point repeated three times. Cite the
agreeing reports with file:line refs. If one of the three did not address the
question, note that explicitly — silence is not disagreement.

The operator's added value here is small (the agreement does the work), but
the operator may add a one-line nuance if it sharpens the statement.

### B. Signals — single-agent findings are rare and often pivotal

A finding surfaced by **only one** of the three agents is **not less important
than convergent findings**. It is a **signal** — a rare indicator that one
expert saw something the others missed. In our experience these single-agent
findings are often the actual direction the work needed to take.

For every signal, the operator writes:

- **what the signal says** (cited by file:line)
- **why the other two missed it** (didn't address the question, gave a wrong
  answer, ran a weaker search strategy, etc.)
- **operator's signal verdict**:
  - **amplify** — operator agrees, this signal is the right direction; treat
    it as a real finding for downstream work
  - **flag** — operator unsure, signal worth follow-up research or
    runtime experiment before acting on it
  - **acknowledge & reject** — operator read carefully, finds the signal
    not credible (must explain why with reasoning, not handwave)

Signals never get hidden in "consensus" or "minority" framing. They get a
**dedicated section** in the synthesis where each one is named, cited, and
adjudicated by the operator individually.

### Step 6 — Produce the synthesis document

Write the synthesis document to
`$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/research/<run_id>/synthesis.md`
in the run directory. **The three source reports remain as individual files
in the same directory — DO NOT inline them.**

**Document structure (kategoryczny — operator dyscyplinowany):**

```markdown
---
run_id: <generated-unique-id>
skill: vc-research
project: <repo-name>
status: completed
operator_synthesis_by: <claude|codex|gemini|maciej|monika>
source_reports:
  - claude_<run_id>.md
  - codex_<run_id>.md
  - gemini_<run_id>.md
---

# Research Synthesis: <title>

> Operator's expert interpretation of the three source reports. Each non-trivial
> claim cites file:line in the source reports. The reports themselves remain
> as immutable expert testimony in separate files — read them directly when
> you want the full unfiltered evidence.

## 0. Coverage statement

Each source report below was read in full via layered slicing before synthesis
was written:

- `claude_<run_id>.md`: <N> lines / <K>KB, 100% read in <M> spans
- `codex_<run_id>.md`: <N> lines / <K>KB, 100% read in <M> spans
- `gemini_<run_id>.md`: <N> lines / <K>KB, 100% read in <M> spans

If any report could not be read in full, this synthesis MUST halt at section 0
with an explicit boundary statement. Citing line ranges from an unread
report is forbidden.

## 1. Problem

<from Step 1 — the actual question, not the symptom>

## 2A. Convergent findings (deduplicated — one statement per finding)

> Findings where two or three experts agreed. Reduced to a single statement
> per finding. Source refs cite the agreeing reports.

### F1: <finding statement, in operator's voice>

**Sources**: `claude_<run_id>.md:L42-58`, `codex_<run_id>.md:L101-115`
**Not addressed by**: Gemini (or "all three"; or omit if all addressed)
**Operator note**: <one-line nuance if it sharpens, otherwise omit>

### F2: ...

## 2B. Signals (single-agent findings — POTENTIAL key insights)

> Findings surfaced by only one of the three agents. NOT lower-priority than
> convergent — often these are the actual direction the work needed to take.
> Each signal gets named, cited, and adjudicated individually.

### S1: <signal statement, in the agent's voice>

**Source**: `gemini_<run_id>.md:L78-92`
**Why others missed it**: <claude did not address Q3; codex addressed it but
gave wrong answer because…; etc.>
**Operator's verdict**: **amplify** | **flag for follow-up** | **acknowledge & reject**
**Reasoning**: <if amplify: why the signal is right and convergent view incomplete.
If flag: what experiment / further research would resolve it.
If reject: what specifically in the signal's reasoning fails, with reference
to repo evidence or named external knowledge — never handwave.>

### S2: ...

## 4. Architecture Decision

- **Chosen approach**: <operator's decision>
- **Why**: <reasoning citing specific findings via file:line>
- **Alternatives rejected**:
  - <alternative> — rejected per `<file>:Lxx-yy` because <reasoning>

## 5. Implementation Notes

- <concrete guidance — cite source for each non-trivial item>
- <API signature: see `codex_<run_id>.md:L130-145` for verified syntax>

## 6. Remaining Gaps

- <questions none of the three could answer — cite where each agent gave up>
- <areas needing hands-on experimentation>

## 7. How to Read This

- This synthesis is **operator's expertise**. The three reports it cites
  remain as standalone artifacts in this directory — open them when you
  want the full unfiltered text from each agent.
- File:line refs are absolute to the report file (e.g. `claude_<run_id>.md:L42-58`
  means lines 42-58 inclusive in that file).
- If you disagree with operator's judgment, the source reports are right
  there — read them and form your own view. That's what they are for.
```

**Imperatyw operatora (kategoryczny):**

1. **Synthesis NIE zawiera verbatim treści raportów** — tylko cytaty file:line
   do nich. Synthesis to **opinia eksperta** (operatora), nie copy-paste.
2. **Reports zostają jako osobne pliki** w run directory. Są immutable expert
   testimony — pełna treść, pełen frontmatter, oryginalny styl.
3. **Każda nietrywialna teza w synthesis MUSI mieć file:line ref** do co
   najmniej jednego raportu. Brak refa = anti-pattern (operator zmyśla).
4. **Dissent jest cytowany z file:line do obu/wszystkich stron** + reasoned
   judgment operatora dlaczego jedna strona przeważa.
5. Synthesis jest krótki (zwykle 3-8KB). Wartość = jakość interpretacji +
   precyzja cytowania, nie objętość.

Present the synthesis document to the user. This is the input for
vc-workflow Phase 3 (Implement) or standalone implementation.

The reader who wants the **full unfiltered evidence** opens the three source
report files directly. The reader who wants the **operator's reasoned judgment**
reads the synthesis. Both audiences served, neither is forced to wade through
the other's preferred view.

## Pipeline Integration

vc-research can be used:

- **Standalone** — when you need research without a full ERi pipeline
- **As workflow Phase 2** — vc-workflow can delegate here instead of
  doing single-agent research
- **Before vc-partner** — when partner mode needs ground truth before
  debug session
- **Before vc-agents/vc-delegate** — research feeds implementation plans

```
         ┌─── claude ──→ report ───┐
research │                         │
  plan ──├─── codex  ──→ report ───├──→ plans/<ts>_<slug>_RESEARCH.md
         │                         │
         └─── gemini ──→ report ───┘
```

## Anti-Patterns

- Passing `claude|codex|gemini` to `vc-research` (defeats the purpose — the launcher is the swarm)
- Giving each agent different questions (they must answer the SAME questions
  independently for triangulation to work)
- Skipping synthesis and just concatenating reports (the value is in the delta)
- Researching things you can verify by reading one file (use loctree slice)
- Writing the research plan without the user (Step 1 is collaborative)
- Trusting blog posts over official documentation
- Letting agents research without loctree context (they ask wrong questions)
- Jumping straight to raw `*_spawn.sh` invocations when `*-research` already
  exists in the real shell helper surface

---

_Created by M&K (c)2024-2026 VetCoders_
