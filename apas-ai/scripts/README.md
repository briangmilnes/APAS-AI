# APAS Scripts Directory

Organized scripts for Rust tooling, APAS code review, and project utilities.

## New Infrastructure (2025-10-14)

### Shared Library: `lib/review_utils.py`
All review scripts now use a common library providing:
- **Standardized arguments**: `--file <path>` and `--dry-run`
- **File discovery**: Smart file finding with single-file mode
- **Path handling**: Consistent relative path display
- **Reporting**: Unified violation output format

### Analysis: `analyze/`
Renamed from `counting/` for better semantics.
- **pareto_violations.py**: Runs all reviews and generates Pareto chart showing which issues are most impactful (80/20 rule)
- **review_clippy.py**: Pareto analysis of Clippy warnings
- **count_*.sh**: Various LOC/pattern counters

### Top-Level Review: `review.py`
Master review runner that:
- Calls APAS and Rust review suites
- Outputs to both stdout AND `analyses/review.txt`
- Provides historical record of review results

## Directory Structure

### APAS/
APAS-specific code review, style enforcement, and refactoring tools.

- `review_APAS.py` - Runs all APAS code reviews (src, tests, benches)

#### APAS/src/
Scripts for reviewing and fixing APAS source code conventions.

- `review_APAS_src.py` - Runs all APAS src reviews

**Code Review (review_*):**
- `review_conventions.py` - Reviews APAS conventions (graph notation A:/E:, Mt files use MtT, Per files immutability, UFCS patterns)
- `review_imports.py` - Reviews import patterns (crate:: in src/, apas_ai:: in tests/benches, wildcard usage)
- `review_naming.py` - Reviews naming conventions (Factory ban, CamelCase, prohibited variable names, file capitalization)
- `review_remaining_alias_usage.py` - Reviews remaining type alias usage patterns
- `review_structure.py` - Reviews code structure (code outside modules, cfg(test) in integration tests, pub fields, extern crate)
- `review_no_factory_pattern.py` - Checks for banned 'Factory' pattern (APASRules.md Lines 176-181)
- `review_copyright_header.py` - Checks for correct copyright on line 1 (APASRules.md Lines 190-195)
- `review_persistent_immutability.py` - Checks *Per files for immutability (no &mut self, no set/update) (APASRules.md Lines 49-53)
- `review_graph_notation.py` - Checks directed graphs use A:, undirected use E: (APASRules.md Lines 60-72)
- `review_vec_usage.py` - Flags suspicious Vec usage for manual review (APASRules.md Lines 3-16)
- `review_mt_discipline.py` - Checks *Mt* files use MtT not StT (APASRules.md Lines 44-47)
- `review_parallel_model.py` - Checks for spawn/join (no rayon, no thresholds) (APASRules.md Lines 39-42)
- `review_apas_where_clauses.py` - Checks Fn(&T)->B should be Pred<T> (APASRules.md Lines 96-101)
- `review_functional_modules.py` - Checks functional modules have typeless traits (APASRules.md Lines 103-175)
- `review_unit_structs.py` - Checks unit structs with algorithms (APASRules.md Lines 183-188)

**Import Pattern Analysis (find_*):**
- `find_and_fix_ufcs_aliases.py` - Finds and fixes UFCS type alias patterns
- `find_duplicate_chap_imports.py` - Finds duplicate chapter imports across the codebase
- `find_duplicate_imports.py` - Finds duplicate import statements
- `find_missing_traits.py` - Finds missing trait imports
- `find_multi_import_patterns.py` - Finds multi-import patterns requiring consolidation
- `find_specific_imports.py` - Finds specific import patterns for analysis

**Import Pattern Fixes (fix_*):**
- `fix_duplicate_imports.py` - Removes duplicate import statements
- `fix_imports.py` - Standardizes import patterns to APAS conventions
- `fix_missing_trait_imports.py` - Adds missing trait imports
- `fix_pattern1_duplicate_chapters.py` - Fixes pattern 1 duplicate chapter imports
- `fix_pattern2_specific_imports.py` - Fixes pattern 2 specific imports
- `fix_pattern3_types_imports.py` - Fixes pattern 3 types imports
- `fix_super_imports.py` - Fixes super module import patterns
- `fix_ufcs_delegation.py` - Fixes UFCS delegation patterns
- `revert_aggressive_wildcards.py` - Reverts overly aggressive wildcard imports
- `revert_pattern1.py` - Reverts pattern 1 changes when needed

**APAS-Specific Fixes:**
- `fix_flathash_methods.py` - Fixes FlatHash-specific method calls

#### APAS/tests/
Scripts for reviewing and fixing APAS test code.

- `review_APAS_tests.py` - Runs all APAS test reviews

- `find_multi_imports_tests_benches.py` - Finds multi-import patterns in tests and benchmarks
- `fix_all_test_issues.py` - Batch fixes for common test issues
- `fix_integration_test_structure.py` - Fixes integration test structure (removes cfg(test) modules)
- `fix_remaining_test_errors.py` - Fixes remaining test compilation errors
- `fix_test_indentation.py` - Fixes test code indentation issues
- `renumber_test_files.sh` - Renumbers test files for consistent naming

#### APAS/benches/
Scripts for reviewing and managing APAS benchmark code.

- `review_APAS_benches.py` - Runs all APAS benchmark reviews

- `review_cargo_bench_names.py` - Reviews benchmark registration in Cargo.toml
- `review_duplicate_ids.py` - Reviews for duplicate benchmark IDs
- `review_timing_params.py` - Reviews benchmark timing parameters (300ms warmup, 1s measurement, 30 samples)
- `rename_benches.sh` - Renames benchmark files to follow APAS conventions
- `rename_tests_and_benches.sh` - Renames both test and benchmark files

---

### rust/
General Rust tooling and utilities, not APAS-specific.

- `review_rust.py` - Runs all Rust code reviews (src, tests, benches)

**Cross-Cutting Reviews (check all of src/, tests/, benches/):**
- `review_no_extern_crate.py` - Reviews for forbidden 'extern crate' usage (RustRules.md Line 86)
- `review_no_ufcs_call_sites.py` - Reviews for UFCS usage at call sites (RustRules.md Lines 309-320)
- `review_import_order.py` - Reviews import ordering: std → external → internal, with blank lines (RustRules.md Lines 50, 75-86)
- `review_camelcase.py` - Reviews file names for CamelCase convention (RustRules.md Lines 303-306)

**Cross-Cutting Fixes:**
- `fix_import_order.py` - Fixes import ordering and blank lines, sorts Types first

#### rust/src/
General Rust source code fixes and utilities.

- `review_rust_src.py` - Runs all Rust src reviews

**Code Structure Reviews:**
- `review_module_encapsulation.py` - Reviews that all code is within pub mod blocks (RustRules.md Lines 117-123)
- `review_variable_naming.py` - Reviews for prohibited variable names: temp_, rock bands (RustRules.md Lines 22-26)
- `review_where_clause_simplification.py` - Reviews for overly simple where clauses (RustRules.md Lines 322-329)
- `review_trait_default_pattern.py` - Reviews trait default implementations: one-liners in trait, multi-line in impl (RustRules.md Lines 136-171)
- `review_qualified_paths.py` - Reviews for fully-qualified paths (std::collections::HashMap) that should be imported
- `review_struct_file_naming.py` - Reviews that struct names match file names (e.g., RelationStEph struct in RelationStEph.rs)
- `review_trait_self_usage.py` - Reviews trait methods using concrete types (Set<T>) instead of Self in return types

**Module Registration Reviews:**
- `review_cargo.py` - Reviews Cargo.toml for missing test/benchmark registrations
- `review_lib.py` - Reviews lib.rs for missing module declarations

**Compilation Fixes:**
- `fix_delete_assignments.py` - Fixes delete assignment syntax errors
- `fix_delete_tuples.py` - Fixes tuple deletion syntax errors
- `fix_dereference_issues.py` - Fixes dereference operator issues
- `fix_remaining_compilation_errors.py` - Fixes remaining general compilation errors

**Trait Pattern Fixes:**
- `fix_trait_method_duplication.py` - Removes inherent methods that duplicate trait methods (RustRules.md Lines 136-171)
- `align_trait_arrows.py` - Aligns -> arrows in trait method signatures for readability

#### rust/tests/
General Rust test running utilities.

- `review_rust_tests.py` - Runs all Rust test reviews

**Test Structure Reviews:**
- `review_integration_test_structure.py` - Reviews for #[cfg(test)] in integration tests (RustRules.md Lines 292-298)
- `review_test_modules.py` - Reviews that all test files compile

**Test Utilities:**
- `nextest.sh` - Wrapper for cargo nextest with project-specific flags
- `test_single_file.py` - Tests a single Rust file in isolation

#### rust/benches/
General Rust benchmark running and management utilities.

- `review_rust_benches.py` - Runs all Rust benchmark reviews
- `review_bench_modules.py` - Reviews that all benchmark files compile
- `audit_benchmarks.sh` - Audits first 50 benchmark files with timeouts, reports slow benchmarks
- `benchmark.sh` - Basic benchmark runner
- `kill_benchmarks.sh` - Kills running benchmark processes
- `run_benchmarks_batch.sh` - Runs benchmarks in batches
- `run_benchmarks_with_timeout.sh` - Runs benchmarks with configurable timeouts

---

### benches/
Benchmark-specific utilities (used by both rust/ and APAS/ benchmarks).

- `audit_first_n.sh` - Audits first N benchmark files
- `audit_one_benchmark.sh` - Audits a single benchmark file with proper timeout
- `count_benchmark_runs.py` - Counts actual benchmark runs in a file (for timeout calculation)
- `count_benchmarks.py` - Counts benchmark functions in a file

---

### counting/
Counting and metrics utilities for codebase analysis.

- `count_as.sh` - Counts 'as' keyword usage in codebase
- `count_loc.sh` - Counts lines of code
- `count_vec.sh` - Counts Vec usage patterns
- `count_where.sh` - Counts where clause usage
- `review_clippy.py` - Pareto analysis of Clippy warnings from analyses/clippy.txt

---

### lint/
Legacy lint directory (being migrated to APAS/src/).

- `README.md` - Documentation for lint scripts (historical)

---

### tmp/
**Temporary one-time scripts only.**

Scripts that solve a specific problem once and are done. Examples:
- Migration scripts (rename files following new convention)
- One-time import pattern fixes
- Revert scripts for undoing mistakes
- Batch compilation error fixes
- Test structure migrations

See `tmp/README.md` for details. Clean out periodically.

---

## Top-Level Scripts

General project utilities and cross-cutting tools.

**Core Development:**
- `build.py` - Build project with cargo build
- `build_tests.py` - Build all tests without running (cargo test --no-run)
- `build_benches.py` - Build all benchmarks without running (cargo bench --no-run)
- `test.py` - Run tests with cargo nextest (--no-fail-fast)
- `bench.py` - Run benchmarks with cargo bench -j 10

**Code Quality:**
- `clippy.py` - Run Clippy linter, output to analyses/clippy.txt (Emacs compile mode compatible)
- `review.py` - Master script that runs all code reviews (APAS + Rust)
- `format.sh` - Runs rustfmt on the codebase

**Development Environment:**
- `generate_tags.sh` - Generates ctags/rusty-tags for editor navigation
- `install_ubuntu_tools.sh` - Installs required Ubuntu development tools

---

## Usage Patterns

### Core Development Workflow
```bash
# Build the project
./scripts/build.py

# Run tests
./scripts/test.py

# Run benchmarks
./scripts/bench.py

# Format code
./scripts/format.sh
```

### Running Code Reviews
```bash
# Run all code reviews (APAS + Rust)
./scripts/review.py

# Run only APAS reviews
./scripts/APAS/review_APAS.py

# Run only APAS src reviews
./scripts/APAS/src/review_APAS_src.py

# Run specific review
./scripts/APAS/src/review_naming.py
./scripts/APAS/src/review_conventions.py
```

### Finding Issues
```bash
# Find import issues
./scripts/APAS/src/find_duplicate_imports.py
./scripts/APAS/src/find_missing_traits.py

# Find patterns
./scripts/APAS/src/find_multi_import_patterns.py
```

### Fixing Issues
```bash
# Fix imports
./scripts/APAS/src/fix_imports.py
./scripts/APAS/src/fix_duplicate_imports.py

# Fix compilation errors
./scripts/rust/src/fix_remaining_compilation_errors.py
```

### Running Tests and Benchmarks
```bash
# Run tests
./scripts/rust/tests/nextest.sh

# Run benchmarks with audit
./scripts/rust/benches/audit_benchmarks.sh

# Audit specific benchmark
./scripts/benches/audit_one_benchmark.sh benches/Chap18/BenchArraySeqStEph.rs
```

### Counting and Metrics
```bash
# Count lines of code
./scripts/counting/count_loc.sh

# Count Vec usage
./scripts/counting/count_vec.sh

# Count where clauses
./scripts/counting/count_where.sh
```

---

## Organization Principles

1. **Language-specific**: `rust/` for general Rust tools, `APAS/` for APAS-specific conventions
2. **Mirror project structure**: `*/src/`, `*/tests/`, `*/benches/` mirror the main project layout
3. **Naming convention**: All scripts use `snake_case` (Python PEP 8 and Bash convention)
4. **Prefix patterns**:
   - `review_*` - Code review and validation (read-only analysis)
   - `find_*` - Search and pattern matching (read-only analysis)
   - `fix_*` - Automated fixes (modifies files)
   - `count_*` - Metrics and counting (read-only analysis)
5. **Shared utilities**: `benches/` and `counting/` provide shared tools used across contexts

---

## Development Workflow

1. Write code → Run `review_*` scripts to check conventions
2. Find issues → Use `find_*` scripts to locate problems
3. Fix automatically → Run `fix_*` scripts to correct issues
4. Verify → Re-run `review_*` scripts to confirm
5. Test → Use `rust/tests/` and `rust/benches/` utilities
6. Analyze → Use `counting/` scripts for metrics

---

### onetime/
One-time analysis scripts created during development. These are inline Python/shell scripts that were saved for documentation and future reference.

- `count_qualified_paths_by_directory.py` - Counts files with qualified path violations by directory (src/tests/benches). Used for planning fix_qualified_paths batch processing.
- `get_qualified_paths_files.py` - Extracts sorted list of files with qualified path violations by directory. Used for planning batch fix execution.
- `fix_qualified_paths_batch.py` - Batch processes files with qualified path violations in groups of 5, compiling after each batch. Used for the qualified paths refactoring task.

---

## See Also

- `analyses/` - Output directory for analysis results
- `rules/` - APAS rules and conventions documentation
- `checklists/` - Code review checklists

