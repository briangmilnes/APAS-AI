# Test Coverage Methodology

## The Problem
Writing tests without verifying coverage leads to:
- Incomplete coverage (tests don't exercise all code paths)
- Wasted effort (moving to new files before completing current ones)
- False sense of progress (lots of tests ≠ good coverage)

## The Solution: Complete-One-File-At-A-Time

### Core Principle
**Never move to the next file until the current file reaches 100% coverage.**

### The Workflow

```
┌─────────────────────────────────────────────┐
│  1. Pick ONE target file                    │
└────────────────┬────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────┐
│  2. Write initial tests                     │
│     - Cover basic functionality             │
│     - Cover main API methods                │
└────────────────┬────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────┐
│  3. Check coverage                          │
│     cargo llvm-cov --test TestXXX -j 10     │
│     Look for: X% coverage, Y lines uncovered│
└────────────────┬────────────────────────────┘
                 │
                 ▼
        ┌────────┴────────┐
        │  100% coverage? │
        └────────┬────────┘
                 │
        ┌────────┴────────┐
        │ NO              │ YES
        ▼                 │
┌──────────────────────┐  │
│  4. Find missing     │  │
│     lines            │  │
│     --show-missing-  │  │
│     lines            │  │
└──────────┬───────────┘  │
           │              │
           ▼              │
┌──────────────────────┐  │
│  5. Add targeted     │  │
│     tests            │  │
│     - Cover edge     │  │
│       cases          │  │
│     - Cover error    │  │
│       paths          │  │
│     - Test trait     │  │
│       methods        │  │
│       directly       │  │
└──────────┬───────────┘  │
           │              │
           ▼              │
┌──────────────────────┐  │
│  6. Verify 100%      │  │
│     Re-run coverage  │  │
└──────────┬───────────┘  │
           │              │
           └──────────────┤
                          ▼
                ┌─────────────────────┐
                │  7. Move to next    │
                │     file ONLY NOW   │
                └─────────────────────┘
```

## Key Commands

### Check coverage for a specific test
```bash
cd /path/to/project
cargo llvm-cov --test TestFileName -j 10 2>&1 | grep "TargetFile.rs" | sed 's/\x1b\[[0-9;]*m//g'
```

### Find missing lines
```bash
cargo llvm-cov --test TestFileName -j 10 --show-missing-lines 2>&1 | grep -A 5 "TargetFile.rs"
```

Output format:
```
/path/to/TargetFile.rs: 108, 148, 155, 162
```

### Read specific uncovered lines
```bash
# Read lines 105-115 to understand context
```

## Common Coverage Gaps

### 1. Macros Can Bypass Code Paths
**Problem**: `MappingLit![]` calls `FromRelation(&RelationLit![])`, not `empty()`

**Solution**: Test both macro AND underlying trait method:
```rust
// Via macro
let m: Mapping<N, &str> = MappingLit![];

// Via trait method directly
let m: Mapping<N, &str> = <Mapping<N, &str> as MappingStEphTrait<N, &str>>::empty();
```

### 2. Edge Cases Often Missed
- Negative inputs
- Empty collections
- Boundary values (0, MAX, MIN)
- Error paths

### 3. Trait Implementations
- `IntoIterator` for owned, `&`, and `&mut`
- `Clone`, `Debug`, `Display`, `PartialEq`
- `Default`

## Anti-Patterns to Avoid

### ❌ BAD: Spray-and-Pray Testing
```
Write 50 tests across 5 files
→ Check coverage once at the end
→ Discover many files have <80% coverage
→ Hard to fix because context is lost
```

### ✅ GOOD: Focused Iteration
```
Pick File A
→ Write 10 tests
→ Check: 85% coverage, 15 lines uncovered
→ Add 4 targeted tests
→ Check: 100% coverage ✓
→ Move to File B
```

## Metrics for Success

| Metric | Target | Why |
|--------|--------|-----|
| Coverage per file | 100% | Complete confidence in code |
| Files completed | Track count | Measure real progress |
| Tests per file | Varies | Quality over quantity |
| Time to 100% | Document | Learn efficiency |

## Example Session Log

```
File: SubsetSumStEph.rs
Initial: 73% coverage (27 lines uncovered)
After round 1: 85% coverage (15 lines uncovered)
  - Added: empty multiset, duplicates, large targets
After round 2: 100% coverage ✓
  - Added: negative target check, IntoIterator impls
Result: 27 tests, 100% coverage
Time: 15 minutes
```

## When to Move On

**ONLY move to the next file when:**
1. ✅ Coverage = 100.00%
2. ✅ All tests pass
3. ✅ No compiler warnings in test file

**Never move on with:**
- ❌ "Good enough" at 95%
- ❌ "Just a few lines" uncovered
- ❌ "I'll come back to it later"

## Integration with Workflow

This methodology fits into the broader coverage improvement workflow:

1. **Select target files** (e.g., via Pareto analysis of uncovered lines)
2. **Apply this methodology** to each file sequentially
3. **Track progress** in coverage reports
4. **Commit** when groups of files reach 100%

---

**Created**: 2025-10-15
**Lesson learned from**: SubsetSumStEph coverage session
**Key insight**: "Lots of tests ≠ good coverage" - verify at each step

