<img width="1536" height="677" alt="image" src="https://github.com/user-attachments/assets/4c238bf2-3087-472a-a420-1f68f717f5ad" />

# VibeCrafted Framework

The definitive toolkit for AI-guided engineering.

VibeCrafted is not just a collection of prompts; it is a structured, opinionated framework for orchestrating AI agents (Codex, Claude, Gemini) to build, refactor, and ship software at veterinary speed.

Quick links:

- [Website](https://vetcoders.github.io/vibecrafted/)
- [Quick Start](docs/QUICK_START.md)
- [Answered FAQ](docs/FAQ.md)
- [Marketplace Listing Draft](docs/MARKETPLACE_LISTING.md)

## The Paradigm

We follow the **Living Tree** methodology. Agents work directly in your repository. We do not use isolated worktrees for active implementation unless testing destructive operations. We believe in _Product truth beating local elegance_.

Read more in our core documents:

- [VIBECRAFTED.md](docs/VIBECRAFTED.md) - The core philosophy.
- [PERCEPTION.md](docs/PERCEPTION.md) - How our agents see your code using loctree.

## Installation

We strictly adhere to a **"No 'why?' questions" rule** for installation.
Our installer is 100% transparent, interactive, and non-destructive. It explains everything it does and only adds a single `source` line to your shell configuration. It never overwrites your global configs.

To install the VibeCrafted Framework from the public bootstrap path:

```bash
curl -fsSLO https://raw.githubusercontent.com/VetCoders/vibecrafted/main/install.sh
bash install.sh
```

This stages a local control-plane copy inside `~/.vibecrafted/tools/` and then runs our safe, interactive orchestrator (`setup_vibecraft.py`) from that local snapshot.

To verify that staged install later:

```bash
make -C ~/.vibecrafted/tools/vibecrafted-current doctor
```

If you already have a local checkout and want to run the orchestrator directly:

```bash
make vibecrafted
```

## Directory Structure

- `skills/` - The core AI skills (e.g., `vc-justdo`, `vc-partner`, `vc-workflow`). These are the brains of the operations.
- `docs/` - Core architectural documentation.
- `scripts/` - Installation and migration scripts.
- `config/` - The VibeCrafted frontier configs (starship, atuin, zellij) loaded dynamically as sidecars.

## Getting Started

Once installed, simply run your preferred VibeCrafted command in the terminal. For example:

- `vc-justdo`: Build and ship a feature from idea to completion.
- `vc-dou`: Run a "Definition of Undone" audit.
- `vc-workflow`: Run the full Examine -> Research -> Implement pipeline.

For a full list of commands, just type `vc-` and hit tab.
