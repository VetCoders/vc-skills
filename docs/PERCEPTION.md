# Perception: How Agents See the Codebase

Language models cannot guess architecture topology.
And they should not pretend to.

In 𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍., agents do not "read code" — they **perceive** structure
through instruments. The instruments deliver objective truth.
The agent delivers interpretation and action.

## Loctree: Your Senses

Loctree MCP is the primary perception layer. Every tool is a different sense:

| Sense            | Tool        | What it reveals                                         |
| ---------------- | ----------- | ------------------------------------------------------- |
| **Overview**     | `repo-view` | Files, LOC, languages, health score, top hubs           |
| **Focus**        | `focus`     | Module internals, edges, external dependencies          |
| **X-Ray**        | `slice`     | File + all dependencies + all consumers in one view     |
| **Blast Radius** | `impact`    | What breaks if you touch this file                      |
| **Search**       | `find`      | Does this symbol already exist? (regex, multi-query)    |
| **Signals**      | `follow`    | Dead code, cycles, twins, hotspots — field-level detail |
| **Layout**       | `tree`      | Directory structure with LOC counts                     |

## The Discipline

Before editing, an agent maps. Before deleting, an agent measures blast
radius. Before creating, an agent searches for existing parts.

This is not overhead. This is the difference between a craftsman who
studies the material and one who cuts blind.

```
1. repo-view    → know the territory
2. focus        → narrow the scope
3. slice        → X-ray before cutting
4. impact       → measure what depends on your target
5. find         → check if the part already exists
6. follow       → pursue structural symptoms to root cause
7. grep/read    → now detail matters
8. validate     → run gates, confirm the patient is stable
```

## Dual-Source Truth

When multiple senses disagree, the disagreement is the signal:

- `sniff` says an export is dead → `dist` says it is in the bundle
- That disagreement reveals a dynamic import the static analysis missed
- The correction eliminates an entire class of false positives

This is convergence through counterexample applied to perception itself.

## Mylik

A **mylik** is a small, plausible misread that grows into documentation drift.

It happens when an agent copies a true shape from one actor, layer, or runtime
surface into another one where it no longer applies. The source was not fake;
the mapping was wrong.

Typical myliki:

- operator fallback paths get documented as application runtime paths
- one endpoint serving two functions gets treated as one policy
- an empty template value gets mistaken for a configurable topology
- source code suggests one story while deployed env truth says another

The antidote is not more prose. The antidote is actor/layer/runtime
separation:

1. name the actor using the surface
2. name the function of the path or endpoint
3. verify real deployed values before updating docs
4. mark fallback/admin paths as such when they are not product runtime truth

If a future agent says "the docs drifted, but the code looked convincing",
look for the mylik first.

## Living Tree

Agents working in a shared workspace use these senses to avoid collisions,
understand side-effects, and make safe decisions without demanding constant
human micromanagement.

When you install 𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍., this perception engine is available by default
in every AI session.

---

`//𝚟𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍.`
