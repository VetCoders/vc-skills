---
name: vc-marbles
version: 3.1.0
description: >
  Counterexample-guided convergence — the loop that makes code healthier.
  Runs adaptive loops that ask one question: "what is still wrong?"
  Each fix eliminates a counterexample to health — and reveals the next one.
  The system cannot get worse, only better. Monotonic entropy reduction.
  No target needed. No plan needed. Just repeated pressure against wrongness.
  Stop when nothing is wrong. The circle is full.
  Trigger phrases: "marbles", "loop until done", "fill the gaps", "kulki",
  "iteruj aż będzie gotowe", "convergence loop", "counterexample",
  "what is still wrong", "adaptive loops", "keep going until clean",
  "wypełnij okrąg", "entropy reduction", "konwergencja".
---

# vc-marbles — Convergence Through Counterexample

> Not "is it correct?" — that cannot be proven.
> Only "what is still wrong?" — and eliminate it.
> Each loop removes entropy. Stop when the circle is full.

## The Mechanism

Traditional quality asks: _is this correct?_ and tries to prove yes.
That question has no finite answer for a living codebase.

Marbles asks a different question: **what is still wrong?**

Each loop inspects the current state and finds **counterexamples** —
concrete things that contradict health. A dead export in `utils.ts:42`.
A circular import between `auth/` and `api/`. A twin export `Button`
living in two files. These are not abstract noise. They are specific,
named, located violations of health.

This is counterexample-guided convergence — CEGIS applied to code:

```
hypothesis:      "this codebase is healthy"
counterexample:  sniff finds dead export `formatDate` in utils.ts:42
correction:      remove dead export
new landscape:   utils.ts is now empty → new counterexample revealed
correction:      remove empty file
new landscape:   import in api.ts pointed to utils.ts → broken import revealed
correction:      fix import
new landscape:   cycle between api.ts and auth.ts disappeared → health score jumps

No single loop understood the whole.
Each loop only answered: "what is still wrong?"
The convergence was emergent.
```

Fixing one issue **changes the landscape**, exposing issues hidden beneath
worse ones. This is the cascade effect — the primary convergence driver.
Entropy drops monotonically. You cannot go backwards.

## Agent Blindness

The agent in each loop does not know it is in a loop.

It receives the original plan (or prompt) and sees the current state of
the living tree. That is all. No loop metadata, no previous reports, no
awareness that other agents ran before it. It just does the job: read the
plan, look at the code, find what is wrong, fix it, run gates.

Convergence happens because each agent independently finds less wrong than
the one before — the previous agent already fixed its share. No coordination
needed. No loop awareness needed. The shrinking problem space IS the
convergence signal.

Marbles is typically the result of `/vc-workflow` or `/vc-followup` identifying
issues that need iterative pressure (roughly half the time). The other half,
it runs from a raw prompt or a plan file. Either way, each agent gets the
same starting brief against an evolving codebase.

## When To Use

- After first implementation pass leaves known gaps
- When followup reveals findings that need iterative fixing
- When the team says "keep going until it's clean"
- Anytime the answer to "is it done?" is "almost"
- When you need adaptive iteration count (not fixed 2 loops)

## Convergence Protocol

### Each Loop Iteration

```
┌─────────────────────────────────────────────────────────┐
│  LOOP N                                                  │
│                                                          │
│  1. ASK: "what is still wrong?"                          │
│     └─ Run loctree-mcp tools widely (multiple independent sources)  │
│     └─ List concrete counterexamples to health           │
│                                                          │
│  2. TARGET the most prominent counterexamples            │
│     └─ Max 3-5 items per loop (don't boil the ocean)     │
│     └─ Expect cascades: fixing these will reveal more    │
│                                                          │
│  3. ELIMINATE counterexamples                             │
│     └─ vc-agents (first choice) or vc-delegate (small)   │
│     └─ Each fix narrows the space of possible bugs       │
│                                                          │
│  4. OBSERVE the new landscape                            │
│     └─ Run gates on the changed codebase                 │
│     └─ NEW findings may appear (cascade) — expected      │
│                                                          │
│  5. SCORE                                                │
│     └─ Distinguish cascade from divergence               │
│     └─ Decide: continue or converged?                    │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Convergence Metrics

After each loop, track:

- **P0 / P1 / P2 counts** (must all be 0 to converge)
- **Cascade findings** (new issues revealed by fixes — expected and healthy)
- **Net counterexamples remaining**
- **Quality gates** (build, lint, tests, security)
- **Convergence score** (0-100: 0=deep issues, 100=circle full)

### Stopping Criteria

**STOP when:**

1. No counterexamples remain at any priority
2. Two consecutive loops with zero delta (plateaued)
3. User says stop

**DO NOT STOP when:**

- Counterexamples remain (unless user explicitly accepts)
- Quality gates failing
- Divergence detected (stop iterating, but investigate)

### Cascade vs Divergence

When loop N has MORE findings than loop N-1:

**Cascade (healthy):** Previous findings RESOLVED, new ones appeared
because fixes revealed hidden issues. Old problems gone, new ones shallower.
Continue — the cascade will settle.

**Divergence (unhealthy):** Previous findings STILL PRESENT, new ones
appeared on top. Fixes are introducing problems without solving old ones.
**STOP.** Re-examine the approach. Do not continue blind iteration.

## Integration with VibeCrafted Pipeline

```
scaffold → init → workflow → followup → [MARBLES] ↻ → dou → decorate → hydrate → release
                                         ^^^^^^^^^^^^^
```

Marbles is the gate between building and shipping.
It loops itself until the circle is full.

## Last Pass: Prune Before You Leave

Before declaring convergence, step back and look at the repo from a distance.

Implementation loops accumulate sediment: dead helpers, orphaned modules,
stale experiments, duplicated glue, files that served a fix three loops ago
and now serve nothing. Every loop that adds code without removing dead code
increases entropy — the opposite of what marbles promises.

Run `/vc-prune` as the final gate before the circle closes. Use `loctree-mcp`
and structural tools to find what the implementation loops left behind:

- Dead code that no runtime, build, or test path reaches
- Twin files and near-duplicates introduced across loops
- Stale scaffolding that was necessary mid-convergence but not after
- Orphaned registrations, imports, and manifest entries

This is not optional cleanup. This is the last counterexample class:
**the sediment itself.** The circle is not full until the debris from
filling it is gone too.

## Anti-Patterns

- Fixed loop count ("always run 4 loops") — defeats adaptive convergence
- Looping without asking "what is still wrong?" — blind iteration
- Leaking loop awareness to agents (loop count, previous reports, convergence status)
- Rigid P0→P1→P2 ordering as steering — cascades don't respect categories
- Continuing past convergence (overfit — introduces new problems)
- Looping without writing reports (no trajectory = no learning)
- Confusing cascade with divergence
- Single counterexample per loop (too slow — target 3-5)
- Entire codebase per loop (too broad — scope to affected area)
- Skipping prune pass before declaring convergence

---

_"Not 'is it correct?' — that cannot be proven._
_Only 'what is still wrong?' — and eliminate it._
_Stop when the circle is full."_

_Vibecrafted with AI Agents by VetCoders (c)2026 VetCoders_
