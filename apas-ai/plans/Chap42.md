# Chap42 Implementation Plan

1. **Confirm scope & constraints**
   - Re-read `rules/` and `checklists/` (RustRules, APASRules, AlgorithmicAnalysis, ExecutionStyle, CodexExecution).
   - Log constraints: work limited to `src/tests/benches/Chap42Codex`. Note PSP step (process definition) and record estimated effort (~7 hours).
   - Confirm readiness to execute relentlessly without pause.

2. **Update shared configs**
   - Modify `Cargo.toml` (`[test]`/`[bench]` entries) and `src/lib.rs` to register Chap42Codex modules.
   - Run `RustRulesChecklist` + `APASRulesChecklist` before/after edits, then incremental `cargo build` until clean.

3. **Create `src/Chap42Codex/ArrayTableEnumStEph.rs`**
   - Implement enumerable ST table (full API) with `Vec<Option<V>>` storage and `/// Codex Work/Span` annotations.
   - Run `PerSrcFile`, `RustRules`, `APASRules`, `AlgorithmicAnalysis` checklists before/after, then `cargo build` (clean).

4. **Create `src/Chap42Codex/ArrayTableEnumMtEph.rs`**
   - Implement parallel enumerable table (scoped threads for bulk ops) with analysis annotations.
   - Run same checklists as above, then incremental `cargo build`.

5. **Create `src/Chap42Codex/AVLTreeTableStEph.rs`**
   - Adapt AVL structure to table API with ordered keys + analyses.
   - Run required checklists + incremental `cargo build`.

6. **Create `src/Chap42Codex/AVLTreeTableMtEph.rs`**
   - Multi-threaded AVL table with documented synchronization and analysis.
   - Run checklists + incremental `cargo build`.

7. **Create `src/Chap42Codex/Example42_1.rs`**
   - Demonstrate example operations, include annotations.
   - Run checklists + incremental `cargo build`.

8. **Create `tests/Chap42Codex/TestArrayTableEnumSt.rs`**
   - Run `PerTestFile`, `RustRules`, `APASRules` checklists before/after.
   - Execute `cargo nextest --no-fail-fast --no-capture --color never --test TestArrayTableEnumSt` until clean.

9. **Create `tests/Chap42Codex/TestArrayTableEnumMt.rs`**
   - Same checklists; run targeted `cargo nextest` for this test.

10. **Create `tests/Chap42Codex/TestAVLTreeTableSt.rs`**
    - Checklists + targeted `cargo nextest`.

11. **Create `tests/Chap42Codex/TestAVLTreeTableMt.rs`**
    - Checklists + targeted `cargo nextest`.

12. **Create `benches/Chap42Codex/BenchArrayTableEnumSt.rs`**
    - Run `PerBenchmark`, `RustRules`, `APASRules` checklists before/after.
    - Execute `cargo bench --bench BenchChap42CodexArrayTableEnumSt` (only this bench) until clean.

13. **Create `benches/Chap42Codex/BenchArrayTableEnumMt.rs`**
    - Checklists + targeted `cargo bench` for this file.

14. **Create `benches/Chap42Codex/BenchAVLTreeTableSt.rs`**
    - Checklists + targeted `cargo bench`.

15. **Create `benches/Chap42Codex/BenchAVLTreeTableMt.rs`**
    - Checklists + targeted `cargo bench`.

16. **Algorithmic analysis review**
    - Run `AlgorithmicAnalysisChecklist` across all new modules to ensure annotations/cost reporting are consistent.

17. **Final verification**
    - Full `CARGO_NET_OFFLINE=true cargo build` (clean) and full `CARGO_NET_OFFLINE=true cargo nextest --no-fail-fast --no-capture --color never` (clean).
    - Run all new benches sequentially; capture summaries.
    - Execute `PostPlanChecklist`.

18. **Summary & handoff**
    - Prepare final report documenting changed files, checklist outcomes, outstanding issues, actual vs estimated time (or note pending), and any pauses or mitigations.
