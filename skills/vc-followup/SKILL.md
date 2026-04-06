---
name: vc-followup
version: 2.0.0
description: >
  [DEPRECATED / MERGED] Production follow-up audit skill. 
  This skill is redundant with the Review phase of vc-workflow and the standalone
  vc-review skill. The workflow has been consolidated.
  Trigger phrases: "follow-up check", "followup audit", "czy sa jeszcze luki",
  "readiness before hands-on", "audit this implementation", "po implementacji",
  "gaps after agents", "co zostało do zrobienia", "post-implementation review".
compatibility:
  tools: []
---

# vc-followup

> **DEPRECATED**: This skill caused logical drift by replicating the "Review & Marbles Escalation" loop already present in `vc-workflow`, `vc-justdo`, and `vc-review`.

To perform a followup audit:

1. If you are reviewing a branch or PR gap, use **`vc-review`** directly to generate artifacts.
2. If those finding artifacts contain `P0`/`P1` issues, immediately route to **`vc-marbles`** to loop through and fix them.

Do not use this file anymore. Follow the `vc-review` → `vc-marbles` pipeline instead.
