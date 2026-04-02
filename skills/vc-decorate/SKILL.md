---
name: vc-decorate
version: 2.1.0
description: >
  Late-stage visual finishing and experience coherence skill. Detects the user's
  existing design language, audits system consistency, distinguishes identity
  from drift, upgrades weak patterns, and proposes tasteful polish that works
  WITHIN the user's system. Never imposes the agent's taste. Never decorates
  chaos. First make the system coherent. Then make it feel premium.
  Trigger phrases: "decorate", "make it look good", "add polish", "smaczki",
  "micro-interactions", "udekoruj", "dopracuj wizualnie", "curb appeal",
  "premium pass", "finish the experience", "make it feel intentional",
  "coherence audit", "design system cleanup", "interactive demo", "animate",
  "add hover effects", "make it feel nice", "visual polish".
---

# vc-decorate — Coherence First. Premium Second.

> "Do not decorate chaos. First make the system coherent. Then make it feel premium."

Decorate is **not** a "make it pretty" skill.
Decorate is a **late-stage product finishing skill**.
Its job is to take a working product and turn it into a **coherent, intentional, premium experience**.

That means:

- detecting the user's real design language (colors, fonts, theme, spacing, interaction rhythm)
- separating identity from drift
- preserving what is distinctive
- upgrading what is weak, dated, or inconsistent
- verifying the end-to-end feel of the product
- and **only then** adding tasteful visual polish and micro-interactions

Decorate does **not** impose the agent's taste.
Decorate does **not** overwrite the user's brand.
Decorate does **not** add random blur, glow, parallax, or "AI prettiness."

Its job is to make the existing system feel:

- more deliberate
- more modern
- more stable
- more precise
- more complete

**Premium is not ornament. Premium is coherence.**

---

## Core Rule: Detect, Don't Dictate

Before decorating anything, run style detection and system audit.

```text
1. SCAN existing CSS variables, theme files, brand colors, fonts, spacing, and component patterns
2. IDENTIFY the user's palette, font stack, theme mode, surface logic, and interaction rhythm
3. AUDIT for visual drift, weak patterns, inconsistent states, and prototype-feel leftovers
4. SEPARATE identity from drift:
   - preserve what is distinctive
   - improve what is weak, stale, or incoherent
5. PROPOSE improvements using THEIR tokens, THEIR language, THEIR stack
6. ASK which changes should be applied
7. IMPLEMENT only approved changes
8. VERIFY the experience end-to-end
```

If no existing style is detected, offer to scaffold a minimal design system first —
but present options, don't assume taste, and don't force a visual identity.

---

## CLI Is Also an Interface

A terminal is not a dumping ground.
CLI output is a user interface. It deserves the same coherence, rhythm,
and intentionality as any web page or app screen. Nasty, raw, unformatted
terminal output is not "developer-friendly." It is offensive to the operator.

Decorate applies to CLI surfaces too:

- installer output (alignment, color, progress signals)
- agent spawn banners (branded, compact, informative)
- doctor/health check output (clear pass/fail, not wall of text)
- `make help` (structured, branded, scannable)
- error messages (human-readable, not stacktrace-first)

If the product has a terminal interface, that interface is part of the
product surface. Decorate it.

## ScreenScribe Input

If `screenscribe` is available as a foundation tool, vc-decorate can consume
a narrated UI screencast to detect visual drift, awkward transitions, and
coherence breaks across a real user flow. Use this when static screenshots are
too thin to explain how the experience actually feels in motion.

### Unicode Toolkit for CLI

𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍. ships a Unicode database (2601 characters, 13 categories) and a
unicode-puzzles-mcp server. Use them for CLI decoration instead of
guessing code points or hardcoding ANSI escape sequences.

**Pipeline: plain text → unicode transform → decorate_text**

Always follow this order:

1. Write the plain text content first
2. Transform labels/titles via `rewrite_using_unicode` (choose style)
3. Wrap the final layout with `decorate_text` for box art (if needed)

Never hand-pick individual code points by memory. Use the MCP tools — they
return verified, consistent characters from the same Unicode block.

**Available styles** (via `rewrite_using_unicode`):

| Style          | Look     | Best for                        |
| -------------- | -------- | ------------------------------- |
| `squared`      | 🄵🅁🄰🄼🄴    | Branding stamps, footer badges  |
| `vaporwave`    | Ｖｉｂｅ | Spaced-out headers              |
| `monospace`    | 𝚟𝚒𝚋𝚎     | CLI subheaders, version strings |
| `smallCaps`    | Vɪʙᴇ     | Inline emphasis                 |
| `fraktur`      | 𝔙𝔦𝔟𝔢     | Decorative section titles       |
| `doubleStruck` | 𝕍𝕚𝕓𝕖     | Mathematical / formal labels    |
| `bubble`       | Ⓥⓘⓑⓔ     | Status badges, tags             |

**CLI decoration elements** (from the Unicode DB):

| Need       | Characters            | Source                           |
| ---------- | --------------------- | -------------------------------- |
| Box frames | `╭─╮│╰─╯`             | Box Drawing block                |
| Separators | `·` `─` `━` `┄`       | Box Drawing, General Punctuation |
| Checkmarks | `✓` `✗` `⚠`           | Dingbats                         |
| Bullets    | `▸` `▪` `◆` `›`       | Geometric Shapes                 |
| Progress   | `⣿⣶⣤⣀` `█▓▒░`         | Braille Patterns, Block Elements |
| Sparklines | `⣀⣤⣶⣿` (8px per cell) | Braille Patterns (256 combos)    |
| Arrows     | `→` `←` `↑` `↓` `⟶`   | Arrows block                     |
| Status     | `⚒` `⚡` `⚙` `⟳`      | Misc Symbols                     |
| Brands     | `🄵·🅁·🄰·🄼·🄴·🅆·🄾·🅁·🄺`   | Enclosed Alphanumerics           |

**Braille sparklines** deserve special attention. A single Braille character
encodes 8 dots in a 2×4 grid (256 combinations). This means a line of 40
Braille characters can display a **320-point convergence curve** in the
terminal — no graphics library needed. Use them for:

- token usage over time
- P0/P1/P2 findings across marbles loops
- agent activity timelines
- any trend data in CLI output

**Rules:**

- Zero ANSI escape codes for text styling — pure unicode renders everywhere
- ANSI colors (`\033[32m` etc.) are acceptable for status coloring only
- Never mix Unicode blocks within one label (squared F next to negative
  squared R looks like a bug, not a choice — unless deliberately designed
  as a signature mark like `🅵·🅁·🄰·🄼·🄴·🅆·🄾·🅁·🄺`)
- Test rendering on at least two terminals (macOS Terminal + Linux default)
- Use `search_unicode` when looking for a specific symbol — don't guess

---

## What Decorate Is For

Use Decorate when:

- the product works but still feels flat, prototype-ish, or unfinished
- the user asks for visual polish, smaczki, curb appeal, or premium feel
- the UI is functionally correct but lacks coherence across surfaces
- the product has good ingredients but weak system feel
- there are inconsistent cards, buttons, spacing rules, focus states, or animation timings
- a showcase page, demo, landing page, or app needs a finishing pass
- the team wants the product to feel more intentional, not just more decorated
- **the CLI output is functional but ugly, unbranded, or hard to scan**

---

## Pipeline Position

```text
scaffold → init → workflow → followup → marbles → dou → [DECORATE] → hydrate → release
                                                        ^^^^^^^^^^^
```

Decorate sits after `dou`, ensuring the now-complete product surface is visually coherent before final packaging (
`hydrate`) and shipping (`release`).

---

## Identity vs Drift

One of Decorate's most important jobs is to tell the difference between:

### Identity

The user's actual visual language:

- chosen palette, typography, spacing rhythm, component forms, and interaction style.

### Drift

Things that merely accumulated:

- inconsistent border radii, mismatched spacing, conflicting button styles, random hover behaviors, or prototype
  artifacts.

Decorate should preserve **identity** and reduce **drift**.

---

## Implementation Pattern

```text
Step 1: Detect
  - scan tokens, stylesheets, framework config, component patterns
Step 2: Audit
  - identify identity vs drift and weak patterns
Step 3: Propose
  - present coherence fixes, premium upgrades, and smaczki
Step 4: Implement
  - apply approved changes using user's tokens and structure
Step 5: Verify
  - review before/after for experience integrity
```

---

## Anti-Patterns

- decorating a broken structure
- keeping bad patterns because "the user already had them"
- replacing their style with ours
- adding motion without interaction purpose
- adding blur/glow because "premium"

---

_Phase 3 — Ship (dou → decorate → hydrate → release)_
_Vibecrafted with AI Agents by VetCoders (c)2026 VetCoders_
