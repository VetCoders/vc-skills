# Contributing to VibeCraft Skills

## License

This project is licensed under the [Business Source License 1.1](LICENSE).
By contributing, you agree that your contributions will be licensed under the same terms.

## How to Contribute

### Reporting Issues

Open an issue on GitHub. Include:

- Which skill is affected
- What you expected vs what happened
- Your environment (OS, agent CLI versions)

### Proposing Changes

1. Fork the repo.
2. Create a branch from `main`.
3. Make your changes — one skill per PR when possible.
4. Run `bash scripts/check-portable.sh` to verify nothing breaks.
5. Open a PR with a clear description of what changed and why.

### Skill Conventions

Every skill lives in its own `vc-<name>/` directory with a `SKILL.md` file.

SKILL.md requirements:

- YAML frontmatter with `name`, `version` (semver), and `description`
- `description` must include trigger phrases in English and Polish
- No hardcoded absolute paths — use `$ROOT` or relative paths
- No secrets, no `.env` files, no machine-specific state

### Quality Bar

- `bash scripts/check-portable.sh` must pass
- No `.DS_Store`, editor junk, or local clutter
- Skills must work without optional dependencies (graceful fallback)
- If your change touches spawn scripts, test on both macOS and Linux

### Commit Style

Short imperative subject line. Body explains why, not what.
No `--no-verify`. No force push to main.

## Code of Conduct

Be direct, be honest, be constructive. VetCoders was built by veterinarians
learning to code — we value clarity over cleverness and respect over ceremony.

---

*VibeCrafted with AI Agents (c)2026 VetCoders*
