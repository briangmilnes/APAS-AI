# Temporary One-Time Scripts

This directory is for **temporary, single-use scripts** that solve a specific one-time problem.

## What Goes Here

Scripts that:
- Fix a specific bug or issue once
- Migrate/rename files for a one-time refactoring
- Revert mistakes from previous operations
- Batch-fix compilation errors from a specific change
- Any other "run once and done" automation

## Examples

- Migration scripts (rename files following new convention)
- One-time import pattern fixes
- Revert scripts for undoing mistakes
- Batch compilation error fixes
- Test structure migrations

## What Does NOT Go Here

Scripts that should be **permanent** and **reusable**:
- `review_*` scripts - ongoing code review checks → `APAS/*/` or `rust/*/`
- `find_and_fix_*` scripts - reusable repair tools → `APAS/*/` or `rust/*/`
- General utilities - format, benchmarks, tests → top-level or rust/

## Cleanup

This directory should be cleaned out periodically. Once a script has served its purpose, it can be deleted or archived elsewhere if needed for reference.

## Git

Consider adding `scripts/tmp/*` to `.gitignore` if these scripts are truly ephemeral and shouldn't be tracked.

