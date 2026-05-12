.DEFAULT_GOAL := help

PYTHON   ?= python3
INSTALLER := scripts/vetcoders_install.py
GUI_INSTALLER := scripts/installer_gui.py
MANIFEST := install.toml
INSTALLER_DIR := scripts/installer
SHELL_INSTALLER := skills/vc-agents/scripts/install-shell.sh
SOURCE   := $(CURDIR)
BRANCH   ?= main
VERSION_FILE := VERSION

.PHONY: help vibecrafted gui-install wizard wizard-dev check test test-skills test-install test-parity install skills helpers setup-dev dry-run doctor list update uninstall restore migrate migrate-dry init-hooks bundle bundle-check foundations foundations-check semgrep version version-show version-bump bump-patch bump-minor bump-major iterm-plugin iterm-plugin-refresh iterm-plugin-show iterm-plugin-uninstall demo demo-full commit-safe test-race-protection skill-new

help:
	@printf "\n"
	@printf "  \033[1m\033[38;5;173m⚒  𝚅𝚒𝚋𝚎𝚌𝚛𝚊𝚏𝚝𝚎𝚍. Framework\033[0m\n"
	@printf "  ─────────────────────────────────────\n"
	@printf "\n"
	@printf "  \033[36m▸\033[0m  make vibecrafted   \033[2mTerminal-native installer wizard (default shell-first front door)\033[0m\n"
	@printf "  \033[36m▸\033[0m  make wizard        \033[2mBrowser-based guided installer (optional GUI surface)\033[0m\n"
	@printf "  \033[36m▸\033[0m  make gui-install   \033[2mAlias for the browser-based guided installer\033[0m\n"
	@printf "\n"
	@printf "  \033[33m◆\033[0m  make install       \033[2mNon-interactive install routed through the same runner with --yes\033[0m\n"
	@printf "  \033[33m◇\033[0m  make skills        \033[2mSkills only\033[0m\n"
	@printf "  \033[33m◇\033[0m  make helpers       \033[2mShell helpers only\033[0m\n"
	@printf "  \033[33m◇\033[0m  make foundations   \033[2mInstall loctree + aicx binaries\033[0m\n"
	@printf "\n"
	@printf "  \033[36m▸\033[0m  make setup-dev     \033[2mSelective interactive install\033[0m\n"
	@printf "  \033[36m▸\033[0m  make dry-run       \033[2mPreview install actions\033[0m\n"
	@printf "\n"
	@printf "  \033[32m✓\033[0m  make doctor        \033[2mVerify installation health\033[0m\n"
	@printf "  \033[32m↻\033[0m  make update        \033[2mPull latest + re-install\033[0m\n"
	@printf "  \033[32m◇\033[0m  make list          \033[2mShow bundle + runtime foundations\033[0m\n"
	@printf "  \033[32m◇\033[0m  make bundle        \033[2mRefresh marketplace plugin bundle\033[0m\n"
	@printf "  \033[32m◇\033[0m  make bundle-check  \033[2mFail if the committed marketplace bundle drifted from repo truth\033[0m\n"
	@printf "  \033[32m✓\033[0m  make test          \033[2mRun installer + marketplace pytest gates\033[0m\n"
	@printf "  \033[32m✓\033[0m  make test-skills   \033[2mRun skill-loader integration smoke (frontmatter + helpers + doctor)\033[0m\n"
	@printf "  \033[32m✓\033[0m  make test-install  \033[2mRun install.sh / install.ps1 cross-platform smoke (Plan 03)\033[0m\n"
	@printf "  \033[32m✓\033[0m  make test-race-protection \033[2mVerify Living Tree commit race detection helper\033[0m\n"
	@printf "  \033[32m✓\033[0m  make test-parity   \033[2mVerify AGENT MODEL PARITY enforcement (Plan 06)\033[0m\n"
	@printf "  \033[32m✓\033[0m  make check         \033[2mRun basic linters on shell scripts\033[0m\n"
	@printf "  \033[32m◆\033[0m  make commit-safe MSG=\"...\" FILES=\"...\" \033[2mRace-protected commit under concurrent agent activity\033[0m\n"
	@printf "  \033[32m◆\033[0m  make skill-new NAME=vc-<name> \033[2mScaffold a new vc-* skill from skills/_template/\033[0m\n"
	@printf "  \033[32m◇\033[0m  make version-show  \033[2mShow VERSION and release tag state\033[0m\n"
	@printf "  \033[32m↟\033[0m  make version-bump VERSION=X \033[2mBump VERSION; X={patch|minor|major|x.y.z}\033[0m\n"
	@printf "\n"
	@printf "  \033[33m◆\033[0m  make migrate       \033[2mMigrate .ai-agents/ to $$VIBECRAFTED_ROOT/.vibecrafted/artifacts/\033[0m\n"
	@printf "  \033[33m◇\033[0m  make migrate-dry   \033[2mPreview migration (dry run)\033[0m\n"
	@printf "\n"
	@printf "  \033[31m✕\033[0m  make uninstall     \033[2mRemove skills + helpers\033[0m\n"
	@printf "  \033[31m↺\033[0m  make restore       \033[2mUndo last install/uninstall\033[0m\n"
	@printf "\n"
	@printf "  \033[2m── experimental ────────────────────────\033[0m\n"
	@printf "  \033[33m◇\033[0m  make iterm-plugin           \033[2mInstall iTerm2 Dynamic Profiles (alongside, idempotent)\033[0m\n"
	@printf "  \033[33m◇\033[0m  make iterm-plugin-refresh   \033[2mOverwrite installed file (creates .bak)\033[0m\n"
	@printf "  \033[33m◇\033[0m  make iterm-plugin-show      \033[2mPrint generated JSON to stdout\033[0m\n"
	@printf "  \033[33m◇\033[0m  make iterm-plugin-uninstall \033[2mRemove the installed file\033[0m\n"
	@printf "  \033[33m✦\033[0m  make demo                   \033[2mLive terminal dashboard z klikalnymi akcjami (vc-* URL handlers)\033[0m\n"
	@printf "  \033[33m✦\033[0m  make demo-full              \033[2mDashboard + aicx HTML serve w tle (browser)\033[0m\n"
	@printf "\n"
	@printf "  ╭─────────────────────────────────────────╮\n"
	@printf "  │ Vibecrafted with AI Agents by VetCoders │\n"
	@printf "  ╰─────────────────────────────────────────╯\n"
	@printf "\n"

vibecrafted: init-hooks
	@if ! command -v uv >/dev/null 2>&1; then \
		echo "bootstrapping uv..."; \
		curl -LsSf https://astral.sh/uv/install.sh | sh; \
	fi; \
	export PATH="$$HOME/.local/bin:$$PATH"; \
	uv run --project $(INSTALLER_DIR) --quiet vetcoders-installer $(MANIFEST)

# BUNDLE_DIR accepts an external prebuilt Svelte site/dist tree
# (e.g. from the sibling vibecrafted-io repo). When empty, `make wizard`
# first tries to build and serve the sibling site checkout so the local
# control plane matches the branded install surface; otherwise it falls
# back to the built-in inline HTML.
BUNDLE_DIR ?=

wizard: init-hooks
	@if [ -n "$(BUNDLE_DIR)" ]; then \
		echo "[wizard] Launching wizard with explicit bundle $(BUNDLE_DIR)"; \
		$(PYTHON) $(GUI_INSTALLER) --source "$(SOURCE)" --bundle-dir "$(BUNDLE_DIR)"; \
		exit 0; \
	fi; \
	if [ -n "$$VIBECRAFTED_SITE_BUNDLE" ]; then \
		echo "[wizard] Using VIBECRAFTED_SITE_BUNDLE=$$VIBECRAFTED_SITE_BUNDLE"; \
		$(PYTHON) $(GUI_INSTALLER) --source "$(SOURCE)"; \
		exit 0; \
	fi; \
	site_repo=""; \
	for p in "$(CURDIR)/../vc-runtime/vibecrafted-io" "$(CURDIR)/../vibecrafted-io" "$$HOME/Libraxis/vc-runtime/vibecrafted-io"; do \
		if [ -d "$$p/site" ]; then site_repo="$$p"; break; fi; \
	done; \
	if [ -z "$$site_repo" ]; then \
		echo "[wizard] vibecrafted-io sibling not found — falling back to inline HTML"; \
		$(PYTHON) $(GUI_INSTALLER) --source "$(SOURCE)"; \
		exit 0; \
	fi; \
	echo "[wizard] Building branded install surface at $$site_repo/site"; \
	if [ ! -d "$$site_repo/site/node_modules" ]; then \
		(cd "$$site_repo/site" && pnpm install --frozen-lockfile=false) || { echo "[wizard] site dependency install failed — falling back to inline HTML"; $(PYTHON) $(GUI_INSTALLER) --source "$(SOURCE)"; exit 0; }; \
	fi; \
	(cd "$$site_repo/site" && pnpm run build) || { echo "[wizard] site build failed — falling back to inline HTML"; $(PYTHON) $(GUI_INSTALLER) --source "$(SOURCE)"; exit 0; }; \
	echo "[wizard] Launching wizard with bundle from $$site_repo/site/dist"; \
	$(PYTHON) $(GUI_INSTALLER) --source "$(SOURCE)" --bundle-dir "$$site_repo/site/dist"

gui-install: wizard

# Development helper preserved as an explicit alias for LiveInstaller work.
# `make wizard` already rebuilds the sibling site when it is available.
wizard-dev: wizard

install: init-hooks
	@if ! command -v uv >/dev/null 2>&1; then \
		echo "bootstrapping uv..."; \
		curl -LsSf https://astral.sh/uv/install.sh | sh; \
	fi; \
	export PATH="$$HOME/.local/bin:$$PATH"; \
	uv run --project $(INSTALLER_DIR) --quiet vetcoders-installer $(MANIFEST) --yes

skills:
	@$(PYTHON) $(INSTALLER) install --source "$(SOURCE)" --non-interactive

helpers:
	@bash $(SHELL_INSTALLER) --source "$(SOURCE)"

foundations:
	@bash scripts/install-foundations.sh

foundations-check:
	@bash scripts/install-foundations.sh --check

setup-dev: init-hooks
	@$(PYTHON) $(INSTALLER) install --source "$(SOURCE)" --advanced

dry-run:
	@$(PYTHON) $(INSTALLER) install --source "$(SOURCE)" --dry-run

doctor:
	@$(PYTHON) $(INSTALLER) doctor

list:
	@$(PYTHON) $(INSTALLER) list --source "$(SOURCE)"

bundle:
	@$(PYTHON) scripts/build_marketplace_bundle.py --output "$(SOURCE)/vibecrafted-framework.plugin"

bundle-check:
	@tmp_root="$${TMPDIR:-/tmp}"; \
	tmp_bundle="$$(mktemp "$$tmp_root/vibecrafted-bundle.XXXXXX")"; \
	trap 'rm -f "$$tmp_bundle"' EXIT; \
	$(PYTHON) scripts/build_marketplace_bundle.py --output "$$tmp_bundle"; \
	if cmp -s "$$tmp_bundle" "$(SOURCE)/vibecrafted-framework.plugin"; then \
		echo "Bundle is current."; \
	else \
		echo "Bundle drift detected. Run 'make bundle'."; \
		exit 1; \
	fi

version version-show:
	@version="$$(sed -n '1p' "$(VERSION_FILE)" 2>/dev/null | tr -d '[:space:]')"; \
	if [ -z "$$version" ]; then echo "VERSION file missing or empty: $(VERSION_FILE)" >&2; exit 1; fi; \
	printf "version: %s\n" "$$version"; \
	printf "tag: v%s\n" "$$version"; \
	if git rev-parse --verify "refs/tags/v$$version" >/dev/null 2>&1; then \
		echo "tag-state: exists"; \
	else \
		echo "tag-state: missing"; \
	fi

version-bump:
ifeq ($(origin VERSION),command line)
	@$(PYTHON) scripts/version_bump.py "$(VERSION)" --file "$(VERSION_FILE)"
else
	@echo "VERSION is required. Usage: make version-bump VERSION={patch|minor|major|x.y.z}" >&2 && exit 1
endif

bump-patch:
	@$(MAKE) version-bump VERSION=patch

bump-minor:
	@$(MAKE) version-bump VERSION=minor

bump-major:
	@$(MAKE) version-bump VERSION=major

semgrep:
	@if command -v semgrep >/dev/null 2>&1; then \
		semgrep scan --config auto --error --quiet --exclude-rule html.security.audit.missing-integrity.missing-integrity .; \
	else \
		uvx semgrep scan --config auto --error --quiet --exclude-rule html.security.audit.missing-integrity.missing-integrity .; \
	fi

test:
	@if command -v uv >/dev/null 2>&1; then \
		PYTHONPATH="$(SOURCE)" uv run --with pytest pytest tests/tui -q; \
	else \
		PYTHONPATH="$(SOURCE)" $(PYTHON) -m pytest tests/tui -q; \
	fi

test-skills:
	@bash tests/skill_loader_smoke.sh

# Plan 03 (META_22) — install.sh / install.ps1 cross-platform smoke.
# Host-only assertions: pre-flight, detection helpers, hint matrix, .ps1
# entry shape. Full install matrix runs in .github/workflows/install-linux.yml.
test-install:
	@bash tests/install_smoke.sh

update:
	@if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then \
		printf "Git repo detected — pulling origin/$(BRANCH)...\n"; \
		git fetch origin; \
		git checkout "$(BRANCH)" -- . 2>/dev/null || git merge --ff-only "origin/$(BRANCH)"; \
		printf "Re-installing...\n"; \
		$(PYTHON) $(INSTALLER) install --source "$(SOURCE)" --with-shell --mirror --non-interactive; \
	else \
		printf "Tarball install — re-running bootstrap installer...\n"; \
		bash "$(SOURCE)/install.sh" --ref "$(BRANCH)"; \
	fi

uninstall:
	@$(PYTHON) $(INSTALLER) uninstall

restore:
	@$(PYTHON) $(INSTALLER) restore

migrate:
	@bash scripts/migrate_agents_workspace.sh

migrate-dry:
	@bash scripts/migrate_agents_workspace.sh --dry-run

check:
	@$(PYTHON) scripts/check_shell.py
	@echo "Check complete."

iterm-plugin:
	@uv run --project vibecrafted-core --quiet python -m vibecrafted_core.iterm2_profiles install

iterm-plugin-refresh:
	@uv run --project vibecrafted-core --quiet python -m vibecrafted_core.iterm2_profiles refresh

iterm-plugin-show:
	@uv run --project vibecrafted-core --quiet python -m vibecrafted_core.iterm2_profiles show

iterm-plugin-uninstall:
	@uv run --project vibecrafted-core --quiet python -m vibecrafted_core.iterm2_profiles uninstall

demo:
	@bash scripts/vc-dashboard

demo-full:
	@bash scripts/vc-dashboard --html

init-hooks:
	@if git rev-parse --git-dir >/dev/null 2>&1; then \
		echo "Installing custom git hooks..."; \
		git config core.hooksPath scripts/hooks; \
		chmod +x scripts/hooks/pre-commit scripts/hooks/pre-push; \
		echo "Hooks installed to scripts/hooks and activated via core.hooksPath."; \
		echo "Ensuring hook toolchain..."; \
		command -v uv >/dev/null 2>&1 || { echo "  installing uv..."; curl -LsSf https://astral.sh/uv/install.sh | sh; }; \
		uvx ruff --version >/dev/null 2>&1 && echo "  ruff: ok" || echo "  [warn] ruff unavailable via uvx"; \
		command -v semgrep >/dev/null 2>&1 && echo "  semgrep: ok" || { echo "  checking semgrep via uvx..."; uvx semgrep --version >/dev/null 2>&1 && echo "  semgrep: ok via uvx" || echo "  [warn] semgrep unavailable"; }; \
		npx --yes prettier --version >/dev/null 2>&1 && echo "  prettier: ok" || echo "  [warn] prettier unavailable via npx"; \
	else \
		echo "Not a git repo — skipping hooks."; \
	fi

# -----------------------------------------------------------------------------
# Living Tree race protection (Plan 07 — kronika 2026-04-16/17 incident learning)
#
# `make commit-safe MSG="..." FILES="path1 path2"` wraps
# scripts/lib/living-tree-commit.sh: stages the named files, snapshots the
# index, commits, and verifies no concurrent agent commit interleaved
# between stage and commit. Exits nonzero on detected race.
# -----------------------------------------------------------------------------

commit-safe:
	@if [ -z "$(MSG)" ] || [ -z "$(FILES)" ]; then \
		echo "usage: make commit-safe MSG=\"<commit message>\" FILES=\"path1 path2 ...\"" >&2; \
		echo "" >&2; \
		echo "Race-protected commit helper for Living Tree workflow." >&2; \
		echo "Detects HEAD shift, staged-tree mismatch, and foreign-file" >&2; \
		echo "inclusion under concurrent agent activity." >&2; \
		exit 1; \
	fi
	@bash scripts/lib/living-tree-commit.sh "$(MSG)" -- $(FILES)

test-race-protection:
	@bash tests/race_protection_test.sh

# -----------------------------------------------------------------------------
# Plan 06 (META_22) — AGENT MODEL PARITY automated enforcement.
#
# Verifies the bash + Python parity layers (scripts/lib/spawn.sh and
# vibecrafted-core/vibecrafted_core/agent_dispatch.py) reject same-family
# downgrades, allow cross-family delegation, and honor the
# VIBECRAFTED_SPAWN_ALLOW_DOWNGRADE=1 operator override with an audit
# warning. Captures kronika 2026-04-10 doctrine.
# -----------------------------------------------------------------------------

test-parity:
	@bash tests/spawn_parity_test.sh
	@if command -v uv >/dev/null 2>&1; then \
		uv run --with pytest pytest tests/agent_dispatch_test.py -q; \
	else \
		PYTHONPATH="$(SOURCE)/vibecrafted-core" $(PYTHON) -m pytest tests/agent_dispatch_test.py -q; \
	fi

# -----------------------------------------------------------------------------
# Plan 04 — skill-authoring scaffolder.
#
# `make skill-new NAME=vc-my-skill` wraps tools/vc-skill-new.sh. The script
# enforces name validation (vc- prefix, lowercase, no collisions) and copies
# skills/_template/ with placeholder substitution. See
# docs/CONTRIBUTING-SKILLS.md for the full operator authoring guide.
# -----------------------------------------------------------------------------

skill-new:
	@if [ -z "$(NAME)" ]; then \
		echo "usage: make skill-new NAME=vc-<skill-name>" >&2; \
		echo "" >&2; \
		echo "Scaffold a new vc-* skill from skills/_template/." >&2; \
		echo "See docs/CONTRIBUTING-SKILLS.md for the authoring guide." >&2; \
		exit 2; \
	fi
	@bash tools/vc-skill-new.sh "$(NAME)"
