# Plan: Create Missing Benchmarks for Chapters 57 & 58

## Overview
Create 4 missing benchmark files for Dijkstra (Chap57) and Bellman-Ford (Chap58) algorithms.
All benchmarks will use 300ms warmup and 1s measurement time per APAS rules.

## Prerequisites
- ✅ `src/Chap57/DijkstraStEphInt.rs` exists
- ✅ `src/Chap57/DijkstraStEphFloat.rs` exists
- ✅ `src/Chap58/BellmanFordStEphInt.rs` exists
- ✅ `src/Chap58/BellmanFordStEphFloat.rs` exists
- ✅ `src/Chap06/WeightedDirGraphStEphInt.rs` exists
- ✅ `src/Chap06/WeightedDirGraphStEphFloat.rs` exists

## Files to Create (4 total)

### Chapter 57 Benchmarks (Dijkstra):
1. `benches/Chap57/BenchDijkstraStEphInt.rs`
   - Benchmarks: sparse graphs, dense graphs, varying sizes (n=10,20,30)
   - Operations: full dijkstra run from source
   
2. `benches/Chap57/BenchDijkstraStEphFloat.rs`
   - Benchmarks: sparse graphs, dense graphs, varying sizes (n=10,20,30)
   - Operations: full dijkstra run from source

### Chapter 58 Benchmarks (Bellman-Ford):
3. `benches/Chap58/BenchBellmanFordStEphInt.rs`
   - Benchmarks: sparse graphs, dense graphs, negative edges, varying sizes (n=10,20,30)
   - Operations: full bellman-ford run from source
   
4. `benches/Chap58/BenchBellmanFordStEphFloat.rs`
   - Benchmarks: sparse graphs, dense graphs, negative edges, varying sizes (n=10,20,30)
   - Operations: full bellman-ford run from source

## Detailed TODO List

### Setup
- [ ] 1. Verify benches/Chap57/ directory exists
- [ ] 2. Create benches/Chap58/ directory if missing

### BenchDijkstraStEphInt.rs
- [ ] 3. Create benches/Chap57/BenchDijkstraStEphInt.rs with:
  - bench_sparse_graph_n10, n20, n30
  - bench_dense_graph_n10, n20, n30
  - 300ms warmup, 1s measurement per benchmark
- [ ] 4. Run RustRules checklist on BenchDijkstraStEphInt.rs
- [ ] 5. Run APAS checklist on BenchDijkstraStEphInt.rs
- [ ] 6. Check BenchDijkstraStEphInt.rs for Vec usage of known length, replace with sequences
- [ ] 7. Update Cargo.toml with [[bench]] entry for BenchDijkstraStEphInt
- [ ] 8. Run `cargo bench --bench BenchDijkstraStEphInt --no-run` until clean

### BenchDijkstraStEphFloat.rs
- [ ] 9. Create benches/Chap57/BenchDijkstraStEphFloat.rs with:
  - bench_sparse_graph_n10, n20, n30
  - bench_dense_graph_n10, n20, n30
  - 300ms warmup, 1s measurement per benchmark
- [ ] 10. Run RustRules checklist on BenchDijkstraStEphFloat.rs
- [ ] 11. Run APAS checklist on BenchDijkstraStEphFloat.rs
- [ ] 12. Check BenchDijkstraStEphFloat.rs for Vec usage of known length, replace with sequences
- [ ] 13. Update Cargo.toml with [[bench]] entry for BenchDijkstraStEphFloat
- [ ] 14. Run `cargo bench --bench BenchDijkstraStEphFloat --no-run` until clean

### BenchBellmanFordStEphInt.rs
- [ ] 15. Create benches/Chap58/BenchBellmanFordStEphInt.rs with:
  - bench_sparse_graph_n10, n20, n30
  - bench_dense_graph_n10, n20, n30
  - bench_negative_edges_n10, n20
  - 300ms warmup, 1s measurement per benchmark
- [ ] 16. Run RustRules checklist on BenchBellmanFordStEphInt.rs
- [ ] 17. Run APAS checklist on BenchBellmanFordStEphInt.rs
- [ ] 18. Check BenchBellmanFordStEphInt.rs for Vec usage of known length, replace with sequences
- [ ] 19. Update Cargo.toml with [[bench]] entry for BenchBellmanFordStEphInt
- [ ] 20. Run `cargo bench --bench BenchBellmanFordStEphInt --no-run` until clean

### BenchBellmanFordStEphFloat.rs
- [ ] 21. Create benches/Chap58/BenchBellmanFordStEphFloat.rs with:
  - bench_sparse_graph_n10, n20, n30
  - bench_dense_graph_n10, n20, n30
  - bench_negative_edges_n10, n20
  - 300ms warmup, 1s measurement per benchmark
- [ ] 22. Run RustRules checklist on BenchBellmanFordStEphFloat.rs
- [ ] 23. Run APAS checklist on BenchBellmanFordStEphFloat.rs
- [ ] 24. Check BenchBellmanFordStEphFloat.rs for Vec usage of known length, replace with sequences
- [ ] 25. Update Cargo.toml with [[bench]] entry for BenchBellmanFordStEphFloat
- [ ] 26. Run `cargo bench --bench BenchBellmanFordStEphFloat --no-run` until clean

### Final Verification
- [ ] 27. Run full `cargo build --release` and verify ZERO warnings
- [ ] 28. Run full `cargo nextest run` and verify all tests pass
- [ ] 29. Run quick test of each benchmark (5-10s each) to verify they complete
- [ ] 30. Run AlgorithmicAnalysis per rules/AlgorithmicAnalysisRules.md on benchmark complexity
- [ ] 31. Run PostPlanChecklist per checklists/PostPlanChecklist.md
- [ ] 32. Create summary document in chatlogs/Chap57_Chap58_Benchmarks_Summary.md

## Benchmark Design Details

### Graph Construction Patterns:
- **Sparse**: ~2-3 edges per vertex
- **Dense**: ~50% of possible edges
- **Negative edges**: Mix of positive and negative weights (no negative cycles)

### Sizes:
- Small graphs (n=10, 20, 30) to keep each benchmark under 1s
- Focus on algorithmic correctness rather than large-scale performance

### Timing Configuration (All Benchmarks):
```rust
group.warm_up_time(Duration::from_millis(300));
group.measurement_time(Duration::from_secs(1));
```

## Time Estimate
- File creation: 4 files × 5 min = 20 min
- Rule checking: 4 files × 3 min = 12 min
- Cargo.toml updates: 4 entries × 1 min = 4 min
- Compilation fixes: ~10 min
- Test runs: 4 benchmarks × 2 min = 8 min
- Final verification: 10 min
- Documentation: 5 min

**Total Estimated Time: ~70 minutes (1h 10min)**

## Can Execute Relentlessly?
✅ **YES** - All steps are deterministic and well-defined. No external dependencies or ambiguous requirements.


