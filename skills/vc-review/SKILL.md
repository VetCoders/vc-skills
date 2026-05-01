---
name: vc-review
version: 1.0.0
description: >
  Bounded code review pipeline: review a PR, branch diff, commit range, or
  generated artifact pack, then produce findings-first output with concrete
  evidence. Use when the user asks to "review PR", "analyze branch", "run
  prview", "sprawdź PR", "zrób review", "audit PR", "daj findings", "zbadaj
  branch", "artifact pack", "PR quality check", "merge gate", "findings-max",
  "deep review", or needs structured diff artifacts with line-level analysis
  for AI review pipelines. Do not use this as a synonym for post-implementation
  direction audit; that is `vc-followup`.
---

# vc-review — Bounded Code Review (Generate + Audit)

Two-phase skill: **Phase 1** generates structured artifacts with `prview-rs`, **Phase 2** squeezes maximum findings from them. Output: P-leveled findings with evidence + before-merge TODO checklist.

Use `vc-followup` instead when the target is not a bounded diff/artifact and the real question is whether the implementation direction is healthy.

- Binary: `prview` (resolve via `command -v prview`; do not assume cargo path)
- Source: `https://github.com/VetCoders/prview-rs`
- Author: Monika (@m-szymanska) — VetCoders

## Operator Entry

Enter the framework via `vibecrafted start` (or `vc-start`). Then launch through the command deck — never raw `skills/.../*.sh` paths:

```bash
vibecrafted review claude --prompt 'Review PR #4'
vc-review codex --prompt 'Deep review of release/v1.2.1 branch against main'
vibecrafted review codex --prompt 'Review HEAD~10..HEAD'
vibecrafted review gemini --file /path/to/pr-artifacts-pack.md
```

`vc-review` must have a bounded target: a PR, branch diff, commit range, or generated artifact pack. Prefer `--pr` or other review-specific inputs.

---

## Phase 1 — Generate Artifacts

Most common dispatches:

```bash
prview --pr <NUMBER>                                    # local branch vs develop/main
prview -R --remote-only <branch> <base>                 # remote branch (no checkout)
prview --pr <NUMBER> --with-tests --with-lint           # GitHub PR by number
prview --deep                                           # all gates
```

Default for vc-review: **do not use `--quick`**. Use `--quick` only for explicit fast triage, artifact refresh under time pressure, or when heavy gates are impossible.

Add `--gh-repo owner/repo` if origin is ambiguous.

> See [references/artifact-pack-layout.md](references/artifact-pack-layout.md) for full flag reference, mode table, profile detection, policy system, and tooling-special-cases.

---

## Artifact Pack Quick Map

Output: `$VIBECRAFTED_ROOT/.prview/pr-artifacts/<branch>/<timestamp>/` (newest = canonical; symlink `latest`).

Top-level structure:

- `report.json` — **canonical structured report** (parse first)
- `dashboard.html` — interactive HTML
- `AI_INDEX.md` — artifact map + reading order
- `00_summary/` — MERGE_GATE, RUN, MANIFEST, SANITY, pr-metadata, file-status, commit-list
- `10_diff/` — `full.patch`, `per-commit-diffs/`, `per-file-diffs/`
- `20_quality/` — per-gate logs/results, `checks-errors.log`, `coverage-delta.txt`, `BREAKING_CHANGES.md`
- `30_context/` — `INLINE_FINDINGS.sarif`, `changed-tests.txt`, tooling output
- `artifacts.zip` — everything zipped

Empty/missing newest dir → finding **P0**.

---

## Phase 2 — Analyze Artifacts (Findings-Max)

### Philosophy

Tryb: **Findings-max**. Nie kończ na "kilku punktach". Jeśli widać 25 osobnych problemów, wypisz 25. Lepiej 20 celnych findingów niż 5 ogólników.

Każdy finding MUSI mieć:

- **Dowód**: artefakt + ścieżka (najlepiej 1–2 linie z patcha/logu)
- **Komentarz**: dlaczego to ważne (1 zdanie) + co grozi
- **Rekomendacja**: co zrobić / jak zweryfikować

Zasady:

- Jeden punkt = jeden problem (nie łącz tematów).
- Czego nie da się potwierdzić z artefaktów: oznacz **[VERIFY]**.
- Rozdzielaj **problem w kodzie** vs **problem narzędzia [TOOLING]**.

### P-Level Scale

| P-level | Definicja                                                          | Przykłady                                                              |
| ------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------- |
| **P0**  | Blocker merge / security / data loss / failing blocking check      | Failing tsc, leaked credentials, missing artifacts                     |
| **P1**  | Wysoki risk regresji w core flow, niekompatybilne zmiany kontraktu | Breaking API, duże zmiany bez testów, import cycles in critical module |
| **P2**  | Średni risk: edge-cases, a11y, telemetria, częściowy brak testów   | Missing i18n keys, hardcoded URLs, no error handling on external call  |
| **P3**  | Niskie ryzyko / higiena / drobne niespójności                      | Empty doc titles, test setup duplication, cosmetic naming              |

---

## Reading Order (Obowiązkowy)

1. **`AI_INDEX.md`** (if exists) — verify it points to real paths. Lying index → P3 [TOOLING].
2. **`report.json`** (canonical) — `meta`, `gate.allow_merge` + `policy_mode` + reasons, `checks[]` (status/log_path/command), `diff.stats` + `diff.files[]` (scale/churn/hotspots), `quality` (breaking/coverage/sarif/heuristics).
3. **`00_summary/MERGE_GATE.json` + `SANITY.json`** — cross-check with `report.json`. Inconsistency → P2 [TOOLING]. "All checks passed" with WARN/INLINE_FINDINGS = misleading.
4. **`00_summary/pr-metadata.txt` + `file-status.txt` + `commit-list.txt`** — scope, A/M/D categories, commit progression. Look for branch drift (infra files outside PR scope).
5. **`30_context/INLINE_FINDINGS.sarif`** — every SARIF result = ready-made finding. Transfer all to findings list.
6. **`20_quality/*`** — PASS gates: extract warnings from logs (cargo warns, tsc non-errors). WARN/ERROR/FAIL: root cause + recommendation. `checks-errors.log` for high-signal filtered errors. `BREAKING_CHANGES.md`: assess real weight (P?). `coverage-delta.txt`: flag critical "NO_TEST_CHANGE" entries.
7. **`30_context/changed-tests.txt`** — cross-reference with source changes. Untested source files → finding.
8. **Diffs (selective)** — `10_diff/per-file-diffs/00-INDEX.txt` for top churn. Per-file patches for hotspots. `10_diff/per-commit-diffs/00-SUMMARY.md` for highest-impact commits.

---

## Mandatory Pattern Scans

Scan per-file patches and `full.patch` for:

**Rust** — `.unwrap()`, `.expect(`, `panic!`, `todo!`, `unsafe`, `dbg!`, `println!`, `#[allow(`

**TypeScript / JavaScript** — `any`, `as unknown as` (double cast), `@ts-ignore`, `@ts-expect-error`, `eslint-disable`, `// TODO|FIXME|HACK`, empty `catch {}` without log/rethrow, non-null assertion `!` on uncertain values, `console.log|warn|error` (should use secureLogger in Vista)

**Security / PII** — logging tokens/emails/passwords/personal IDs, new telemetry without privacy review, new endpoints/command handlers without auth checks, hardcoded URLs/keys/secrets

**Data / Performance** — query in loop (N+1), missing batching for bulk ops, large payloads without pagination, unnecessary I/O in hot paths

Each "hit" in the diff = potential finding with evidence.

---

## Minimum Coverage Requirements

To prevent laconic reports:

- **All** entries in `INLINE_FINDINGS.sarif`
- **Top 10** files by churn — read per-file patch
- **All** files in core risk categories: auth, payments, database, session, security, encryption, middleware
- **All** critical "NO_TEST_CHANGE" entries from `coverage-delta.txt`
- **All** entries in `BREAKING_CHANGES.md` with assessed P-level
- **Per-commit progression** for PRs with >5 commits — identify phases, risky transitions

---

## Output Format (Obowiązkowy)

Two mandatory sections, in this order:

### 1) Findings (P0/P1/P2/P3)

```
- **[P?] <Title>** (optionally: [VERIFY] or [TOOLING])
  - **Evidence:** `<artifact-path>` + `<file:line>` + short fragment (1-2 lines)
  - **Comment:** 1 sentence on risk/impact
  - **Recommendation:** concrete "what to do" / "how to verify"
  - **Owner:** `author` / `reviewer` / `infra` (optional)
```

Number for cross-referencing: `P1-01`, `P1-02`, `P2-01`, etc.

### 2) Before-Merge TODO (Markdown Checkboxes)

```markdown
- [ ] **(P0)** ... (ref: P0-01)
- [ ] **(P1)** ... (ref: P1-01, P1-02)
- [ ] **(P2)** ... (ref: P2-01)
- [ ] **(P3)** ... (ref: P3-01)
```

Each TODO references finding IDs. Include verification commands in code fences where applicable.

### 3) Optional Sections

Add when they provide value:

- **Executive Summary** (max 8 bullets): gate verdict, top 3 risks, test signal, top hotspots, scope delta
- **Architecture Context**: diagram or description of affected subsystem
- **Scope / What Changed**: based on `diff.stats` + top directories + top files
- **Commit Progression**: phases of work for multi-commit PRs
- **Test Coverage Matrix**: source → test → new tests count
- **Security & Privacy Check**: PII in logs, data flows, event filtering
- **QA Plan**: 5-15 manual + automated test recommendations
- **Evidence Index**: links to key artifacts used

---

## 𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍. Pipeline Integration

As input to `vc-followup`:

```bash
prview --pr $PR_NUMBER --with-tests --with-lint
ARTIFACTS="$VIBECRAFTED_ROOT/.prview/pr-artifacts/<branch>/latest"
```

Subagent delegation context:

```
- prview artifacts at: $VIBECRAFTED_ROOT/.prview/pr-artifacts/<branch>/latest/
- Parse report.json first (canonical)
- Read 00_summary/MERGE_GATE.json for quick verdict
- Read 20_quality/checks-errors.log for error details
- Read 10_diff/per-file-diffs/ for hotspot patches
```

JSON pipeline:

```bash
prview --json --quiet | jq '.checks[] | select(.status == "Failed")'
```

---

## Anti-Patterns

### Tool usage

- Using `--quick` as default for PR review (drops test/lint/security signal)
- Running `--deep` on every PR when `--with-tests --with-lint` is enough (save `--deep` for merge gate / high-risk PRs)
- Reading `full.patch` entirely for large PRs (use `per-file-diffs/`)
- Ignoring `report.json` and `MERGE_GATE.json` (parse structured data first)
- Not using `--update` after amend/force-push (generates duplicate artifact sets)
- Running without `--no-fetch` on slow networks

### Analysis

- Stopping at 5 findings when 25 are visible (findings-max means exhaustive)
- Findings without evidence (every point needs artifact path + code fragment)
- Mixing separate problems into one finding (one point = one problem)
- Ignoring tooling issues (tool crash ≠ code issue, but still a finding)
- Skipping pattern scans (the `.unwrap()` / `any` / PII checklist is mandatory)
- Not cross-referencing coverage-delta with changed source files

---

_𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍. with AI Agents by VetCoders (c)2024-2026 LibraxisAI_
