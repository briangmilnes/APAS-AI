# APAS Scripts Directory

Organized scripts for Rust tooling, APAS code review, and project utilities.

## Directory Structure

### APAS/
APAS-specific code review, style enforcement, and refactoring tools.

#### APAS/src/
Scripts for reviewing and fixing APAS source code conventions.

**Code Review (review_*):**
- `review_conventions.py` - Reviews APAS conventions (graph notation A:/E:, Mt files use MtT, Per files immutability, UFCS patterns)
- `review_imports.py` - Reviews import patterns (crate:: in src/, apas_ai:: in tests/benches, wildcard usage)
- `review_naming.py` - Reviews naming conventions (Factory ban, CamelCase, prohibited variable names, file capitalization)
- `review_remaining_alias_usage.py` - Reviews remaining type alias usage patterns
- `review_structure.py` - Reviews code structure (code outside modules, cfg(test) in integration tests, pub fields, extern crate)

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

- `find_multi_imports_tests_benches.py` - Finds multi-import patterns in tests and benchmarks
- `fix_all_test_issues.py` - Batch fixes for common test issues
- `fix_integration_test_structure.py` - Fixes integration test structure (removes cfg(test) modules)
- `fix_remaining_test_errors.py` - Fixes remaining test compilation errors
- `fix_test_indentation.py` - Fixes test code indentation issues
- `renumber_test_files.sh` - Renumbers test files for consistent naming

#### APAS/benches/
Scripts for reviewing and managing APAS benchmark code.

- `review_cargo_bench_names.py` - Reviews benchmark registration in Cargo.toml
- `review_duplicate_ids.py` - Reviews for duplicate benchmark IDs
- `review_timing_params.py` - Reviews benchmark timing parameters (300ms warmup, 1s measurement, 30 samples)
- `rename_benches.sh` - Renames benchmark files to follow APAS conventions
- `rename_tests_and_benches.sh` - Renames both test and benchmark files

---

### rust/
General Rust tooling and utilities, not APAS-specific.

#### rust/src/
General Rust source code fixes and utilities.

- `fix_delete_assignments.py` - Fixes delete assignment syntax errors
- `fix_delete_tuples.py` - Fixes tuple deletion syntax errors
- `fix_dereference_issues.py` - Fixes dereference operator issues
- `fix_remaining_compilation_errors.py` - Fixes remaining general compilation errors

#### rust/tests/
General Rust test running utilities.

- `nextest.sh` - Wrapper for cargo nextest with project-specific flags
- `test_single_file.py` - Tests a single Rust file in isolation

#### rust/benches/
General Rust benchmark running and management utilities.

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

---

### lint/
Legacy lint directory (being migrated to APAS/src/).

- `README.md` - Documentation for lint scripts (historical)

---

### src/
Legacy source utilities directory (empty, historically used).

---

### tests/
Legacy test utilities directory (empty, historically used).

---

## Top-Level Scripts

General project utilities and cross-cutting tools.

- `format.sh` - Runs rustfmt on the codebase
- `generate_tags.sh` - Generates ctags/rusty-tags for editor navigation
- `install_ubuntu_tools.sh` - Installs required Ubuntu development tools
- `review_codebase.sh` - Master script that runs all APAS code review checks

---

## Usage Patterns

### Running Code Reviews
```bash
# Run all APAS code reviews
./scripts/review_codebase.sh

# Run specific review
python3 scripts/APAS/src/review_naming.py
python3 scripts/APAS/src/review_conventions.py
```

### Finding Issues
```bash
# Find import issues
python3 scripts/APAS/src/find_duplicate_imports.py
python3 scripts/APAS/src/find_missing_traits.py

# Find patterns
python3 scripts/APAS/src/find_multi_import_patterns.py
```

### Fixing Issues
```bash
# Fix imports
python3 scripts/APAS/src/fix_imports.py
python3 scripts/APAS/src/fix_duplicate_imports.py

# Fix compilation errors
python3 scripts/rust/src/fix_remaining_compilation_errors.py
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

## See Also

- `analyses/` - Output directory for analysis results
- `rules/` - APAS rules and conventions documentation
- `checklists/` - Code review checklists

