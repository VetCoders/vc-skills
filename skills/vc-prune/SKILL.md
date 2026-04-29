---
name: vc-prune
version: 3.3.0
description: >
  Repository curation, not clear-cutting. Map what truly participates in runtime 
  truth versus what is silently parked — then decide revive, archive, or delete. 
  Includes the silencer strip: rip every `#[allow(...)]`, `// nosemgrep`, 
  `eslint-disable`, `@ts-ignore`, `# noqa`, `# type: ignore`, panic-vs-skip pattern, 
  and any other annotation that mutes a quality gate. Run the gates. Listen.
  Triage with care — `#[allow(dead_code)]` (and equivalents) is often the most 
  valuable smell in a repo: parked work the team forgot about. Surface those as 
  forgotten gems for the operator to decide.
  This skill is a gem hunter, not a clear-cutter.
  Trigger phrases: "prune", "strip dead code", "wyczyść mądrze", "strip the silencers",
  "zdejmij wszystkie ignore", "zobacz co realne", "forgotten gems",
  "co tam zapomnieliśmy".
---

# vc-prune — Curation, Not Clear-Cutting

## Operator Entry

Launch through the command deck (see `vc-init` for the full operator-entry contract):

```bash
vibecrafted prune <agent> --file /path/to/prune-plan.md
vc-prune codex --prompt 'Strip silencers and listen'
```

> Don't burn the house down. Strip it to the load-bearing walls and report what you found behind the wallpaper.

A vibe-coded repo accumulates two layers of debris: **dead surface** (abandoned auth experiments, duplicate Stripe handlers, dead serverless functions) and **silenced surface** (warnings muted in a hurry, tests that always skip, panics that always fire). `vc-prune` separates both layers from runtime truth — and from each other.

## Axioms

1. **Aggressive pruning, with belief in the VCS archive.** Dead code is not bad code. It is a graveyard of valuable ideas. Its place is in Git history, not the runtime. Cut without sentiment — but only after axiom 4.
2. **Move on over backward compatibility.** Rotten abstractions blocking stabilization get cut cleanly. The dependency graph is part of runtime truth.
3. **The code knows. Strip the silence and listen.** Every silencer is a deferred conversation. Most were added in a hurry. The only honest test of which ones still earn their keep is to rip them all out and let the toolchain speak.
4. **`#[allow(dead_code)]` (and its cousins) is often the most valuable signal in a repo.** It usually marks parked work — a 90%-complete login flow, an export pipeline for a churned customer, a debug visualizer no one mentioned to new hires. These are **forgotten gems**, not garbage. Surface them; never auto-delete.

## Core Contract

- For non-trivial prune, `vc-agents` external dispatch is the default first move.
- Assume 30% of a vibe-coded repo is dead scaffolding.
- Classify every candidate: `KEEP-RUNTIME`, `KEEP-BUILD`, `MOVE-ARCHIVE`, `DELETE-NOW`, `VERIFY-FIRST`, or `FORGOTTEN-GEM`.
- Prefer cutting whole dead vertical slices over trimming symbolic leaves.
- Tighten contracts after every wave: manifests, docs, CI, package bounds.
- Run gates after every wave. Require one real smoke or build proof.

## Delegation Doctrine

| Need                                             | Best model |
| ------------------------------------------------ | ---------- |
| Archaeology, hidden reachability, gem-hunting    | Claude     |
| Exact deletions, manifest tightening, mech. work | Codex      |
| Radical simplification, cutting whole subsystems | Gemini     |

## Workflow

### Phase 1 — Define the runtime cone

Capture: real entrypoints, mandatory user flows, build/release path. Do not start from "unused exports". Start from "does this serve live traffic?"

### Phase 2 — Map with `loct`

```bash
loct auto && loct manifests && loct hotspots && loct dead
loct routes      # web/API
loct commands    # desktop/Tauri
loct events
```

### Phase 3 — Prune in waves (safest → riskiest)

**Wave 1 — AI exhaust & prototype scaffolding.** `v1_backup.ts`, `old_auth_handler.js`, `stripe_test_claude.ts`, dead `.claude/` `.codex/` session folders, stale screenshots.

**Wave 2 — Whole dead vertical slices.** Frontends with no consumers, alternate login pages never mounted, webhook handlers replaced by SaaS. Cut the strand, let Git archive it.

**Wave 3 — Unreachable product surface.** Unmounted routes, duplicate engines (Prisma + raw SQL doing the same thing), dead feature flags retained after launch.

**Wave 4 — Contract tightening.** `package.json` deps, `Cargo.toml` features, `pyproject.toml` extras, `.env.example` stale secrets, CI workflows.

**Wave 5 — The Silencer Strip.** Separate wave because it is not about removing dead code; it is about un-muting the toolchain so live code can speak. See below.

### Phase 4 — Verify reality

Green static gates are necessary, not sufficient. Add one real proof path: boot the app, run the CLI, hit the main route.

---

## Wave 5 — The Silencer Strip (Strip and Listen)

### Inventory

```bash
# Rust
rg -n '#\[allow\(' src-tauri/src
rg -n 'nosemgrep' .

# TypeScript / JavaScript
rg -n 'eslint-disable' src
rg -n '@ts-(ignore|nocheck|expect-error)' src
rg -n 'biome-ignore' src

# Python
rg -n '# noqa' .
rg -n '# type: ignore' .
rg -n '# pylint: disable' .
rg -n '@pytest\.mark\.skip' .

# Go (parallel ecosystem)
rg -n '//nolint' .
rg -n 'testing\.Short\(\) \|\| t\.Skip' .

# Test theater across languages
rg -n 'panic!\("Test requires|throw new Error\("requires' .
rg -n 'it\.skip|test\.skip|describe\.skip' .
```

Capture **counts per category**. That is your before-baseline.

### Strip ALL of them in one pass

Bulk-delete the lines. Do not pre-curate "obvious keepers" — the bias that put them there is the same bias that would protect them. Let the toolchain decide.

### Run gates

Whatever the repo already has — do not invent new ones:

```bash
cargo clippy --all -- -D warnings && cargo test --all
pnpm lint:tsc && pnpm code:all && pnpm vitest run
ruff check . && mypy . && pytest          # Python
golangci-lint run && go test ./...        # Go
semgrep --config=auto .                   # if available
pre-commit run --all-files                # if installed
```

### Triage with care

| Finding                         | Resolution                                                                                                                                                             |
| ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Authentic code smell            | **Fix root cause.** Refactor, write proper types, add adapter, drop lock before await.                                                                                 |
| False positive                  | **Refactor stylistically** so the warning never fires. No re-add.                                                                                                      |
| Genuine technical constraint    | **Re-add silencer with incident-grade comment**: WHY (technical reason), WHEN (under what conditions), WHERE (specific code path). Not "intentional", not "by design". |
| **Forgotten gem (gentle path)** | **Do not delete. Report.** A `dead_code` warning on a 200-line module with thoughtful structure is parked work. Add to **Forgotten Gems Report**. Operator decides.    |
| Test theater unmasked           | **Stop. Bigger than the silencer.** A panic-or-skip pattern that always evaluates one way means the test was never real. Open a separate plan for real wiring.         |
| Truly dead code revealed        | The silencer was hiding `dead_code` on a one-liner stub or scratch file. Delete — but only after a fast forgotten-gem check.                                           |

The rule: **a silencer earns its keep only with a written technical reason that another engineer six months from now would accept as serious.** Vague "intentional" comments are not serious. Equally: **delete code only when it is unambiguously trash** — anything in between goes to the Forgotten Gems Report.

### Forgotten Gems Report

Output of Wave 5 is **not** a smaller repo. It is a written report. Save to `$VIBECRAFTED_HOME/artifacts/<org>/<repo>/<YYYY_MMDD>/reports/<timestamp>_forgotten-gems.md`.

```markdown
# Forgotten Gems — <repo> <date>

## Summary

Stripped: N silencers. Real bugs (X), false positives (Y), constraints (Z),
gems (G), test theater (T), truly dead (D). Operator decisions needed: G + T.

## Gems

### #1 src/archive/clinic_export_v2.rs (412 LOC, last touch 2025-09-04)

- What: 2nd-gen export pipeline, clean trait split, full SOAP→PDF, never wired
- Why valuable: better-structured than current export, tests included, no dep drift
- Why parked: PR #341 merged the trait shape; wiring step deferred and forgotten
- Recommendation: revive, retire current path. Operator decision (customer-facing).
- Alt: archive in docs/archive/ + delete from runtime if direction superseded.
```

### Test theater report (separate)

Test theater is debt, not gem. Save to `<timestamp>_test-theater.md`:

```markdown
## src-tauri/tests/e2e/rust/document_tests.rs:120

Was: `panic!("Test requires API credentials")`
Reality: never ran in any CI; required manual `LIBRAXIS_API_KEY` export
Real fix: `tests/common/credentials.rs` loading `src-tauri/.env` via dotenvy
before `has_vision_credentials()`
Owner: <to be assigned>
```

Test theater always gets a follow-up plan. Never a silencer reinstatement.

### Acceptance for the wave

- Remaining silencer count is a **small fraction** of the inventory (target ≤25%, often ≤10%).
- Every remaining silencer carries an incident-grade comment.
- Every gate runs green without `--no-verify`, `cargo clippy --allow-dirty`, `pnpm lint --fix --quiet`, or any other "make it green by hiding it" trick.

### Surprise findings are the prize

Watch specifically for: tests that always skip, tests that always panic, `dead_code` allow on functions whose only caller was deleted three releases ago, `@ts-ignore` on types that have been correct for a year, `eslint-disable jsx-a11y/...` on real a11y violations the framework allegedly forced (when the framework was upgraded in 2025), `nosemgrep: react-dangerouslysetinnerhtml` on HTML that is **not** sanitized, `# type: ignore[arg-type]` on a function whose signature was fixed two refactors ago.

Each is a real bug or a real lie the silencer was hiding. Strip-and-listen finds them. That is the point.

### Case studies

**Vista 0.67.3 (Rust + TypeScript), 2026-04-28.** A late-evening sweep stripped 12 `#[allow(...)]`, 7 `// nosemgrep`, 10 `eslint-disable`, and 24 `@ts-(ignore|nocheck|expect-error)` annotations. After the strip, `cargo test --all` failed on 13 e2e tests with `panic!("Test requires API credentials")` — but the panic was new noise only because adjacent tests in the same suite already silently skipped on the same precondition. **Two contradictory "missing credentials" behaviours co-existing.** Stripping did not introduce the inconsistency; it surfaced it. The bigger lesson: none of those 13 e2e tests had ever actually run without manual env injection. The 13 panicking tests were CI theater; the 5 quietly skipping tests were equally theater. Two flavours of the same lie. The follow-up — a real `dotenvy::from_path("src-tauri/.env")` loader — was the **prize** of running Wave 5.

**Hypothetical Python equivalent (vista-portal billing service).** A sweep of `# type: ignore` and `@pytest.mark.skipif(not stripe_keys_present(), reason="...")` reveals: 11 `# type: ignore[attr-defined]` on the `stripe.Customer` object — every one was added before the `stripe-python` 11.x upgrade landed proper types in 2025-Q1. None of them were still needed. Plus 3 `@pytest.mark.skipif` decorators on webhook idempotency tests that **always skipped in CI** because nobody had wired Stripe test keys into GitHub Actions secrets. Same pattern as Vista, different ecosystem: silencers outliving the bug they hid, plus tests that never ran. The forgotten gem in the same sweep: a 380-line `app/billing/archived_invoice_export.py` with `# noqa: F401` on every import — turns out it was a complete invoice CSV exporter someone built for a customer who churned, never wired into a CLI command, and tested coverage was 87%. Reported up to operator: revive as `vc-export-invoices` CLI, or archive in `docs/archive/billing-archive.md` and delete.

The pattern is universal. Languages and toolchains differ; the discipline is identical.

## Anti-Patterns

- Deleting ten dead symbols while a whole abandoned subsystem still stands.
- Trusting "unused" reports without checking dynamic loading via framework router.
- Preserving a chaotic 2000-line file because "we might need it" — that is what Git history is for.
- Cleaning code but leaving stale dependencies in the lockfile.
- Stripping silencers selectively. The whole point of Wave 5 is to bypass that bias.
- Mass-restoring silencers because there were "too many warnings". That is burying the message again.
- Adding new silencers to silence newly-uncovered warnings. Fix the warning or refactor it away.
- Treating `panic!("Test requires X")` as a real gate, or `it.skip` / `@pytest.mark.skipif` as harmless. Tests that always skip do not exist; they just cost reviewer attention.
- Auto-deleting code that a `dead_code` allow was hiding without a forgotten-gem check first.
- Treating Wave 5 as adversarial. Past engineers added silencers for plausible reasons. Wave 5 is the reread, not a verdict.

## The Pruning Principle

Do not ask the repo to explain every scar. Ask it to justify every surviving surface.

If a surface does not run in production, build the release, or test integrity — the move is **not** automatically delete. The move is **decide with intent**: revive, archive, or delete. The skill exists to surface decisions, not to make them on autopilot.

**The toolchain is not an enemy to be muted. It is a witness to be interrogated.** Strip the silence. Run the gates. Listen. Then decide — case by case, with a written reason — what genuinely deserves to stay quiet, what needs a real fix, and what was a forgotten gem hiding behind the silencer all along.

A repo that has been through `vc-prune` is not necessarily smaller. It is **legible**. Every surviving surface, every surviving silencer, every surviving test has a written reason to be there. That is the win.

---

_Vibecrafted with AI Agents by VetCoders (c)2024-2026 LibraxisAI_
