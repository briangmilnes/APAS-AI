# APAS Rules Automation Analysis

## Summary

**Total Rules:** 20 distinct rules in APASRules.md

**Automatable:** 10 rules (50%)
**Partially Automatable:** 6 rules (30%)
**Not Automatable:** 4 rules (20%)

**Existing Scripts:** 12 review scripts
**Coverage:** ~60% of automatable rules implemented

---

## Automatable Rules (10)

### ✅ IMPLEMENTED (6)

1. **Criterion Bench Configuration** (Lines 29-32)
   - Script: `review_timing_params.py` ✓
   - Checks: warmup ≤1s, measurement ≈6s, sample size ≈30, total ≤10s

2. **APAS Naming Fidelity** (Lines 24-25) 
   - Script: `review_naming.py` ✓
   - Checks: APAS naming conventions match text

3. **Element Shorthands** (Lines 18-22)
   - Script: `review_missing_traits.py` ✓
   - Checks: Required traits imported

4. **Import Structure** (Lines implied)
   - Scripts: `review_imports.py`, `review_specific_imports.py`, `review_duplicate_imports.py`, `review_duplicate_chap_imports.py` ✓
   - Checks: Import structure, duplicates, chapter imports

5. **General Conventions** (Lines various)
   - Script: `review_conventions.py` ✓
   - Checks: General APAS code conventions

6. **Code Structure** (Lines various)
   - Script: `review_structure.py` ✓
   - Checks: Module structure rules

### ❌ NOT YET IMPLEMENTED (4)

7. **Persistent Mutation Ban** (Lines 49-53)
   - **HIGH VALUE** - Check `*Per` files for:
     - No `&mut self` methods
     - No `set()`/`update()`/`insert_in_place()` methods
     - No exposed slices `&[T]` or `&mut [T]`
   - **Script needed:** `review_persistent_immutability.py`

8. **Graph Notation Convention** (Lines 60-72)
   - **MEDIUM VALUE** - Check:
     - Directed graph macros use `A:` (arcs)
     - Undirected graph macros use `E:` (edges)
   - **Script needed:** `review_graph_notation.py`

9. **Factory Pattern Ban** (Lines 176-181)
   - **EASY** - Search for "Factory" in:
     - Struct names
     - Trait names
     - Function names
   - **Script needed:** `review_no_factory_pattern.py`

10. **Documentation** (Lines 190-195)
    - **EASY** - Check all `.rs` files for:
      - Line 1: `//! Copyright (C) 2025 Acar, Blelloch and Milnes...`
      - Line 2-4: Module summary
    - **Script needed:** `review_copyright_header.py`

---

## Partially Automatable Rules (6)

11. **Vec Prohibition** (Lines 3-16)
    - Can detect new `Vec`, `vec![]`, `to_vec()` usage
    - Hard to distinguish "temporary builders" from violations
    - **Possible script:** `review_vec_usage.py` (with manual review)

12. **MT Module Discipline** (Lines 44-47)
    - Can check `*Mt*` files use `MtT` not `StT`
    - Hard to verify thread-safety semantically
    - **Possible script:** `review_mt_discipline.py`

13. **Parallel Spawn/Join Model** (Lines 39-42)
    - Can detect rayon usage
    - Can detect PARALLEL_THRESHOLD
    - Hard to verify spawn/join pattern correctness
    - **Possible script:** `review_parallel_model.py`

14. **APAS Where Clause Simplification** (Lines 96-101)
    - Can detect `Fn(&T) -> B` vs `Pred<T>`
    - Can detect redundant bounds
    - Requires semantic analysis
    - **Possible script:** `review_apas_where_clauses.py`

15. **Functional Module Pattern** (Lines 103-175)
    - Can detect modules with only free functions
    - Can check trait signatures match
    - Hard to verify "typeless trait" requirement
    - **Possible script:** `review_functional_modules.py`

16. **Unit Struct Algorithm Pattern** (Lines 183-188)
    - Can detect unit structs with impl blocks
    - Hard to distinguish "should convert" vs "should keep"
    - **Possible script:** `review_unit_structs.py`

---

## Not Automatable (4)

17. **Chapter Trait Hoisting** (Lines 34-37)
    - Requires understanding method semantics
    - Manual design decision

18. **Iteration vs. Recursion Hygiene** (Lines 55-58)
    - Stylistic choice, context-dependent
    - Requires understanding algorithm structure

19. **Benchmark Macro Usage Patterns** (Lines 74-81)
    - Context-dependent (what's being benchmarked)
    - Requires understanding intent

20. **Defensive interfaces** (Lines 92-94)
    - Semantic rule about behavior
    - Requires understanding intended behavior

---

## Recommended Implementation Priority

### Phase 1: Easy Wins (2 scripts, ~2 hours)
1. `review_no_factory_pattern.py` - Simple grep for "Factory"
2. `review_copyright_header.py` - Check line 1 of all files

### Phase 2: High Value (2 scripts, ~4 hours)
3. `review_persistent_immutability.py` - Enforce Per/Eph distinction
4. `review_graph_notation.py` - Check A: vs E: in graph macros

### Phase 3: Advanced (4 scripts, ~8 hours)
5. `review_mt_discipline.py` - Check MtT usage in *Mt* files
6. `review_apas_where_clauses.py` - Simplify where clauses
7. `review_functional_modules.py` - Check typeless traits
8. `review_vec_usage.py` - Flag suspicious Vec usage

---

## Current Coverage

**Existing Scripts (12):**
- ✅ `review_timing_params.py`
- ✅ `review_naming.py`
- ✅ `review_missing_traits.py`
- ✅ `review_imports.py`
- ✅ `review_specific_imports.py`
- ✅ `review_duplicate_imports.py`
- ✅ `review_duplicate_chap_imports.py`
- ✅ `review_conventions.py`
- ✅ `review_structure.py`
- ✅ `review_cargo_bench_names.py`
- ✅ `review_duplicate_ids.py`
- ✅ `review_remaining_alias_usage.py`

**Missing High-Value Scripts (4):**
- ❌ `review_persistent_immutability.py`
- ❌ `review_graph_notation.py`
- ❌ `review_no_factory_pattern.py`
- ❌ `review_copyright_header.py`

**Total Possible:** ~18 scripts (10 fully automatable + 6 partially + 2 helpers)
**Currently Have:** 12 scripts
**Coverage:** ~67% of maximum possible automation

