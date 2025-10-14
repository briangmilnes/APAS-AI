# RustRules Automation - Violations Found

## Summary

8 automated RustRules review scripts created and executed:

| Script | Rule | Violations | Status |
|--------|------|------------|--------|
| review_no_extern_crate.py | Line 86 | 0 | ✅ PASS |
| review_no_ufcs_call_sites.py | Lines 309-320 | 12 | ❌ FAIL |
| review_import_order.py | Line 50 | 168 | ❌ FAIL |
| review_camelcase.py | Lines 303-306 | 0 | ✅ PASS |
| review_variable_naming.py | Lines 22-26 | 0 | ✅ PASS |
| review_module_encapsulation.py | Lines 117-123 | 0 | ✅ PASS |
| review_where_clause_simplification.py | Lines 322-329 | 50 | ❌ FAIL |
| review_integration_test_structure.py | Lines 292-298 | 51 | ❌ FAIL |

**Total Violations: 281**

## Violation Details

### 1. UFCS at Call Sites (12 violations)
**Rule**: Replace `<Type as Trait>::method(...)` with method-call syntax

**Locations:**
- `src/Chap41/ArraySetEnumMtEph.rs`: 2 (macro-generated)
- `tests/TestTypes.rs`: 10 (primitive type conversions)

**Fix Needed**: Replace UFCS with method syntax or inherent constructors

---

### 2. Import Order (168 violations)
**Rule**: std → external → crate imports

**Breakdown:**
- src/: 27 files (external after crate)
- tests/: 7 files (std after crate)
- benches/: 134 files (std after crate)

**Pattern**: Most benches have `use std::time::Duration;` after criterion imports

**Fix Needed**: Reorder imports to match standard

---

### 3. Integration Test Structure (51 violations)
**Rule**: No `#[cfg(test)]` in integration tests

**Files:** 51 test files across multiple chapters

**Fix Needed**: Remove `#[cfg(test)]` wrapper modules, move tests to root level

---

### 4. Where Clause Simplification (50 violations)
**Rule**: Inline simple single-bound where clauses into generic parameters

**Pattern**: Functions with `where T: SingleTrait` that could be `<T: SingleTrait>`

**Examples:**
- `fn filter<F>(...) where F: FnMut(&T) -> bool` → `fn filter<F: FnMut(&T) -> bool>(...)`
- `fn from_sorted_iter<I>(...) where I: IntoIterator<Item = T>` → `fn from_sorted_iter<I: IntoIterator<Item = T>>(...)`

**Files**: 50 functions across multiple chapters (Chap37, Chap39, Chap45, Chap53, etc.)

**Fix Needed**: Inline simple bounds from where clause to generic parameter list

---

## Passing Reviews

### ✅ No extern crate
Clean! No legacy `extern crate` usage found.

### ✅ CamelCase File Names
All files properly capitalized.

### ✅ Variable Naming
No temp_ or rock band variable names.

### ✅ Module Encapsulation
All code properly inside `pub mod` blocks (except lib.rs/main.rs).

---

## Next Steps

User requested: **Don't fix anything yet** - just identify violations.

When ready to fix:
1. Import order (168) - automated fix possible
2. Integration test structure (51) - automated fix possible
3. Where clause simplification (50) - automated fix possible
4. UFCS call sites (12) - requires case-by-case review

