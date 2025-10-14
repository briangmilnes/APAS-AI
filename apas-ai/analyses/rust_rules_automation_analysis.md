# RustRules.md - Automation Analysis

## Fully Automatable Rules (High Priority)

### 1. **Variable Naming Discipline** (Lines 22-26)
- **Rule**: No "temp" variables, no rock band names
- **Check**: Scan for `temp_`, `led_zeppelin`, `pink_floyd`, etc.
- **Script**: `scripts/rust/src/review_variable_naming.py`
- **Priority**: HIGH - Easy pattern matching

### 2. **Zero Warnings Policy** (Lines 32-41)
- **Rule**: ALL CODE MUST COMPILE WITH ZERO WARNINGS
- **Check**: Run `cargo build` and check for `warning:` in output
- **Script**: Already done via `cargo build` - just need to enforce
- **Priority**: HIGH - Already checked by CI

### 3. **Import Order** (Line 50)
- **Rule**: Specific import ordering (std → external → crate)
- **Check**: Parse imports and verify order
- **Script**: `scripts/rust/src/review_import_order.py`
- **Priority**: HIGH - Clear pattern

### 4. **No `extern crate`** (Line 86)
- **Rule**: Never use `extern crate`
- **Check**: Grep for `extern crate`
- **Script**: `scripts/rust/src/review_no_extern_crate.py`
- **Priority**: HIGH - Simple grep

### 5. **Mandatory Encapsulation** (Lines 117-123)
- **Rule**: ALL CODE MUST BE WITHIN `pub mod M{...}`
- **Check**: Parse file, ensure no free definitions (except main.rs/lib.rs)
- **Script**: `scripts/rust/src/review_module_encapsulation.py`
- **Priority**: HIGH - Critical structural rule

### 6. **Integration Test Structure** (Lines 292-298)
- **Rule**: No `#[cfg(test)]` in integration tests
- **Check**: Scan `tests/` for `#[cfg(test)]` modules
- **Script**: `scripts/rust/tests/review_integration_test_structure.py`
- **Priority**: HIGH - Prevents test discovery issues

### 7. **CamelCase Naming** (Lines 303-306)
- **Rule**: Functions/structures use CamelCase
- **Check**: Parse identifiers, verify case
- **Script**: `scripts/rust/src/review_camelcase.py`
- **Priority**: MEDIUM - Style consistency

### 8. **No Free-Function Wrappers** (Lines 252-258)
- **Rule**: Don't create free functions that just forward to methods
- **Check**: Detect functions that only call one method
- **Script**: `scripts/rust/src/review_no_wrapper_functions.py`
- **Priority**: MEDIUM - AST analysis needed

### 9. **No UFCS at Call Sites** (Lines 309-320)
- **Rule**: Replace `<Type as Trait>::method(...)` with method syntax
- **Check**: Grep for `<.*as.*>::` pattern
- **Script**: `scripts/rust/src/review_no_ufcs_call_sites.py`
- **Priority**: MEDIUM - Pattern matching

### 10. **Where Clause Simplification** (Lines 322-329)
- **Rule**: Inline simple bounds instead of where clauses
- **Check**: Find `where` with single simple bound
- **Script**: `scripts/rust/src/review_where_clause_simplification.py`
- **Priority**: MEDIUM - AST parsing

## Partially Automatable Rules

### 11. **Helper Function Threshold** (Lines 113-115)
- **Rule**: No helper unless used 3+ times
- **Check**: Count function call sites
- **Script**: `scripts/rust/src/review_helper_function_usage.py`
- **Priority**: LOW - Requires call graph analysis
- **Note**: Can detect, but judgment needed

### 12. **Trait Requirement** (Line 127)
- **Rule**: Every public API needs a public trait
- **Check**: Find pub fns without corresponding trait
- **Script**: `scripts/rust/src/review_trait_requirement.py`
- **Priority**: MEDIUM - Complex AST analysis
- **Note**: Can flag missing traits, but needs human review

### 13. **Baseline Bounds** (Lines 136-141)
- **Rule**: Types must have `Eq + Clone + Copy + Display + Debug + Sized`
- **Check**: Parse trait bounds, verify baseline
- **Script**: `scripts/rust/src/review_baseline_bounds.py`
- **Priority**: MEDIUM - AST parsing
- **Note**: Can detect violations, exceptions need judgment

## Not Easily Automatable (Human Judgment Required)

### 14. **Minimal Solution / KISS** (Lines 8-14)
- **Reason**: Requires understanding intent and design alternatives
- **Approach**: Code review only

### 15. **Closure Mutation Patterns** (Lines 16-20)
- **Reason**: Requires semantic analysis of closure capture
- **Approach**: Can flag `FnMut` usage for review

### 16. **Generalized Lifting Rule** (Lines 142-150)
- **Reason**: Requires understanding which bounds are "common to all items"
- **Approach**: Can detect inconsistencies, but judgment needed

### 17. **Elegance / Over-engineering** (Lines 9-14)
- **Reason**: Subjective design assessment
- **Approach**: Architecture review only

## Recommended Implementation Order

1. **Phase 1 (Quick Wins)**:
   - `review_no_extern_crate.py` - Simple grep
   - `review_integration_test_structure.py` - Simple pattern
   - `review_variable_naming.py` - Pattern matching

2. **Phase 2 (Structural)**:
   - `review_module_encapsulation.py` - Parse structure
   - `review_import_order.py` - Parse imports
   - `review_no_ufcs_call_sites.py` - Pattern matching

3. **Phase 3 (Advanced)**:
   - `review_camelcase.py` - Identifier analysis
   - `review_where_clause_simplification.py` - AST parsing
   - `review_trait_requirement.py` - API analysis

## Tools Needed
- **Simple**: `grep`, regex patterns
- **Medium**: File parsing, line-by-line analysis
- **Complex**: AST parsing (consider `syn` crate or tree-sitter-rust)

