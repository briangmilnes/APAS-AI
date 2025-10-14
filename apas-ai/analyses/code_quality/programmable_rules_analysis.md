# Programmable Rules Analysis

This document analyzes which rules from `rules/*.md` can be implemented programmatically (fully or partially).

## Summary

- **Total Rule Categories**: 120+
- **Fully Programmable**: 28 (23%)
- **Partially Programmable**: 42 (35%)
- **Not Programmable**: 50+ (42%)

**Total Automatable (Full + Partial): ~58% of rules**

---

## 1. AlgorithmicAnalysisRules.md (44 lines)

### Fully Programmable: 0
### Partially Programmable: 3
- **Presentation Standards** - Can check for presence of summary tables, proper formatting
- **Documentation** - Can check for `/// <AGENT-NAME> Work:` comment format, presence of analysis markers
- **Algorithmic Analysis Process** - Can verify that analysis is in `chatlogs/` not test files, check file locations

### Not Programmable: 5
- Baseline Reporting - Requires algorithmic understanding
- Recurrence Handling - Requires mathematical analysis
- Data Structure Costs - Requires algorithmic knowledge
- Parallelism Details - Requires concurrency understanding
- Edge Cases and Stability - Requires understanding algorithm behavior

---

## 2. APASRules.md (195 lines)

### Fully Programmable: 6
- **Graph Notation Convention** - Grep for `A:` vs `E:` in graph macros
- **Factory Pattern Ban** - Grep for "Factory" in struct/trait/function names
- **Documentation (copyright)** - Check line 1 for copyright text
- **Defensive interfaces** - Can detect panic vs empty result patterns (partially)

### Partially Programmable: 12
- **Assistant Vec Prohibition** - Detect `vec![]`, `to_vec()`, `Vec::new()` patterns with AST/grep
- **Element Shorthands** - Check for `StT`, `MtT` usage patterns
- **Iterator-based Assertions** - Check test patterns for `iter()` usage
- **Criterion Bench Configuration** - Validate timing parameters (300ms warmup, 1s measurement, 30 samples)
- **Chapter Trait Hoisting** - Check where trait bounds are declared
- **Parallel Spawn/Join Model** - Detect rayon usage, check for `PARALLEL_THRESHOLD`
- **MT Module Discipline** - Check `*Mt*` files for `MtT` vs `StT` usage
- **Persistent Mutation Ban** - Check `*Per` files for `set`/`update`/`insert_in_place` methods
- **Benchmark Macro Usage Patterns** - Detect `from_vec` vs constructor patterns
- **Exercise Benchmark Policy** - Check for `BenchExercise_` files
- **APAS Where Clause Simplification** - Detect `where F: Fn` → `F: Pred` patterns
- **Functional Module Pattern** - Check for trait presence in modules with only free functions
- **Unit Struct Algorithm Pattern** - Detect `pub struct Name;` with impl blocks

### Not Programmable: 7
- APAS Naming Fidelity - Requires textbook knowledge
- Iteration vs. Recursion Hygiene - Requires understanding intent
- Parallel Pair Semantics - Requires algorithm understanding
- Seq-First Rule / Vec-to-Seq Rule - Requires semantic understanding
- No Free-Function Wrappers (semantic) - Requires understanding what code does

---

## 3. BenchmarkRules.md (344 lines)

### Fully Programmable: 13
- **Benchmark Timing Standards** - Validate warmup=300ms, measurement=1s, sample_size=30
- **Unique Benchmark IDs** - Check for duplicate `BenchmarkId::new(name, param)` (script exists)
- **File Organization and Naming** - Validate `benches/ChapXX/BenchModuleName.rs` structure
- **Cargo.toml Registration** - Verify all benchmark files registered (script exists)
- **Benchmark Configuration** - Check Criterion imports and setup
- **Exercise Benchmarks** - Detect `BenchExercise_X_Y.rs` files
- **Benchmark Quality Standards (warnings)** - Check for zero warnings in build output
- **Cross-Chapter Benchmarks** - Validate import paths match file location
- **Black_box usage** - Grep for `black_box` in benchmark code
- **Group configuration** - Check for explicit timing configuration
- **Iterator Benchmarks** - Check for `iter_sum`, `iter_collect` patterns
- **Audit script validation** - Run existing scripts
- **Named functions** - Check for inline closures vs named functions

### Partially Programmable: 5
- **Input Sizes** - Can detect size parameters but not validate appropriateness
- **Macro Usage** - Can detect macro patterns but not validate semantic correctness
- **Multi-Input Benchmarks** - Detect loop patterns for size variations
- **Setup outside iter** - Can check structure but not semantic correctness

### Not Programmable: 2
- Input size appropriateness - Requires performance understanding
- Benchmark measurement validity - Requires understanding what's being measured

---

## 4. CodexExecution1.md (29 lines)

### Fully Programmable: 0
### Partially Programmable: 1
- **Timestamping commands** - Can check for `date +"%d %H:%M:%S"` presence

### Not Programmable: 3
- emacs_sync workflow - Process rule
- Cargo command output - Execution behavior
- Scratch files rule - Workflow rule

---

## 5. ExecutionStyle1.md (256 lines)

### Fully Programmable: 2
- **Zero Warnings Policy** - Check build output for warnings
- **No Perl** - Grep for perl usage in scripts

### Partially Programmable: 3
- **Mandatory Build Verification** - Check for `cargo build` in CI scripts
- **Claude and sed** - Check for sed usage (partial - intent matters)
- **Tool parallelism** - Check for `-j` flags in cargo commands

### Not Programmable: 40+
Most rules in this file are AI behavior, workflow, or process rules:
- Python Script Execution Protocol
- Tool Usage Transparency
- TODO Flow rules (10+ sub-rules)
- Terminal Clear & Output
- Timestamping
- Git workflow rules
- CPR/heartbeat rules
- High-Signal Live Tracing
- etc.

---

## 6. RustRules.md (332 lines)

### Fully Programmable: 15
- **Zero Warnings Policy** - Check build output
- **Module/file layout and Mandatory Encapsulation** - Check for code outside `pub mod`
- **Integration Test Structure** - Check for `#[cfg(test)]` in `tests/` directory
- **CamlCase not SnakeCase** - Check function/struct naming
- **Contain UFCS** - Grep for `<Type as Trait>::` patterns
- **Struct Field Encapsulation** - Check for `pub` fields
- **Macro Normalization** - Check for `#[macro_export]`, `$crate::` usage
- **No trailing per-file re-exports** - Check for `pub use` at end of files
- **Iterator impls** - Check for `IntoIterator` implementations
- **Specialized import rules** - Validate `crate::` vs external crate usage
- **No extern crate** - Grep for `extern crate`
- **Type Creation Traits (derives)** - Check for `#[derive(...)]` presence
- **Result import guidance** - Check import patterns
- **Variable naming (temp/band names)** - Grep for prohibited patterns
- **PartialEq/Eq location** - Check definitions are inside module

### Partially Programmable: 18
- **Standard Library Imports** - Check import patterns and locations
- **Module import style** - Check for wildcard imports vs explicit
- **Use Lit! macros** - Detect `vec![]` vs `Lit!` usage in literals
- **Traits and Implementations** - Check for trait presence alongside implementations
- **Types and Bounds (baseline)** - Check for required trait bounds
- **Generalized lifting rule** - Check where bounds are placed (trait vs method)
- **Function argument bounds** - Check for inline bounds vs where clauses
- **Callable parameter style** - Check `impl Fn` vs generic `F: Fn`
- **Constructor No Raw Backing Collections** - Detect `Vec::new()`, `vec![]` at call sites
- **Type Conversions and Naming** - Check naming patterns (`to_*`, `as_*`, `into_*`)
- **Tests Format** - Check test structure, one test per function
- **Type Inference Cleanup** - Detect UFCS, turbofish patterns
- **Where Clause Simplification** - Detect simplifiable where clauses
- **Hash opt-in** - Check that Hash isn't over-used
- **Wildcard-first imports** - Check import patterns
- **Group imports** - Check for single grouped import lines
- **Helper function extraction** - Count call sites (limited accuracy)
- **Closure Mutation Patterns** - Detect FnMut requirements (limited)

### Not Programmable: 15+
- Terminology alignment - Documentation style
- Minimal solution philosophy - Design judgment
- Formatting discipline - Process rule
- Helper function threshold - Requires semantic understanding
- No Free-Function Wrappers (semantic) - Requires understanding functionality
- Test via Public API Only - Requires understanding what's tested
- Closure semantics - Requires understanding mutation
- Various "prefer X" rules requiring judgment

---

## Implementation Strategy

### High-Value Quick Wins (Fully Programmable)
1. **Naming Checks** (grep-based):
   - Factory pattern ban
   - CamlCase vs snake_case
   - Variable naming (temp, band names)
   - UFCS patterns

2. **Structure Checks** (AST/grep-based):
   - Code outside modules
   - cfg(test) in integration tests
   - pub fields
   - extern crate usage
   - Graph notation (A: vs E:)

3. **Benchmark Validation** (existing scripts + enhancements):
   - Timing parameters
   - Unique IDs
   - Cargo.toml registration
   - File naming/location

4. **Import/Module Checks**:
   - crate:: vs external
   - Wildcard patterns
   - Trailing re-exports

### Medium Complexity (Partially Programmable)
1. **Pattern Detection** (AST analysis):
   - vec![] vs Lit! macros
   - Vec prohibition patterns
   - Trait hoisting
   - Where clause simplification

2. **Convention Checks**:
   - StT/MtT usage in Mt files
   - set/update in Per files
   - Test structure

### Complex/Semi-Automatable
1. **Semantic Analysis** (limited AST + heuristics):
   - Free function wrappers
   - Helper function usage counts
   - Closure mutation requirements

---

## Tools Required

1. **Grep/Ripgrep** - Pattern matching (30% of rules)
2. **Tree-sitter/syn** - Rust AST parsing (40% of rules)
3. **Python Scripts** - Custom analysis logic (existing + new)
4. **Cargo Integration** - Build/test verification (10% of rules)
5. **Git Hooks** - Pre-commit validation (5% of rules)

---

## Conclusion

Approximately **58% of the APAS rules can be fully or partially automated**:
- 23% can be fully implemented programmatically
- 35% can be partially checked with programmatic tools
- 42% require human judgment, AI behavior, or workflow management

The best ROI comes from:
1. Benchmark validation (highly structured, already partially implemented)
2. Naming and structure checks (low complexity, high impact)
3. Import and module organization (clear patterns, enforceable)
4. Build verification (zero warnings policy)

The remaining rules requiring AI/human judgment include:
- Algorithmic analysis
- Design philosophy (KISS, minimal solutions)
- Workflow and process rules
- Semantic understanding of code intent

