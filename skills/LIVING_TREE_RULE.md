# Living Tree Rule

VetCoders work in one shared repository checkout.

Vibecrafted workflows do **not** create, switch to, or move work into git
worktrees by default. Worktrees are not a harmless implementation detail here:
they split runtime truth, hide concurrent edits, multiply merge surfaces, and
turn fast Vibecraftsmanship into branch archaeology.

## Hard Rule

- Work in the current checkout and current branch.
- Do not run `git worktree add`, create a side checkout, or relocate execution
  into another lane.
- Do not switch branches during active workflow execution.
- Do not create branches unless the operator explicitly asks for that git move.
- Re-read files before editing when time has passed or concurrent agents may be
  active.
- Treat local changes as shared work. Never stash, discard, reset, or overwrite
  changes you did not make.

## Only Exception

A worktree is allowed only when the operator explicitly says to use a worktree.
Generic requests like "isolate this", "work in parallel", "make a clean branch",
or "avoid conflicts" are not enough.

If the current substrate is too poisoned to continue safely, stop and report the
substrate failure. Do not solve substrate invalidity by escaping into a worktree.

## Why

Vibecrafting optimizes for rapid convergence on runtime truth. The pace is the
point. We do not move that fast so that a stale side tree can later force the
team into rebase drift, duplicate conflict repair, or backwards motion.

Training-data defaults about worktrees are subordinate to this repository
doctrine.
