//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Minimum Edit Distance.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

use apas_ai::{
    ArraySeqMtEphSLit, ArraySeqStEphS, ArraySeqStPerS, Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS,
    Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS, Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS,
    Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS, Chap49::MinEditDistMtEph::MinEditDistMtEph::*,
    Chap49::MinEditDistMtPer::MinEditDistMtPer::*, Chap49::MinEditDistStEph::MinEditDistStEph::*,
    Chap49::MinEditDistStPer::MinEditDistStPer::*, MinEditDistMtEphLit, MinEditDistMtPerLit, MinEditDistStEphLit,
    MinEditDistStPerLit, Types::Types::Pair,
};

fn bench_min_edit_distance_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_st_per");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    // APAS Example 49.3
    let example_solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );
    group.bench_function("apas_example_49_3", |b| {
        b.iter(|| black_box(example_solver.min_edit_distance()))
    });

    // Small sequences
    let small_solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C'],
        target: ['X', 'Y', 'Z']
    );
    group.bench_function("small_sequences", |b| {
        b.iter(|| black_box(small_solver.min_edit_distance()))
    });

    // Medium sequences
    let medium_solver = MinEditDistStPerLit!(
        source: ['k', 'i', 't', 't', 'e', 'n'],
        target: ['s', 'i', 't', 't', 'i', 'n', 'g']
    );
    group.bench_function("medium_sequences_kitten_sitting", |b| {
        b.iter(|| black_box(medium_solver.min_edit_distance()))
    });

    // Large sequences
    let source_large: Vec<char> = "abcdefghijklmnop".chars().collect();
    let target_large: Vec<char> = "axcxefxhijxlmnxp".chars().collect();
    let large_solver = MinEditDistStPerS::from_sequences(
        ArraySeqStPerS::from_vec(source_large),
        ArraySeqStPerS::from_vec(target_large),
    );
    group.bench_function("large_sequences", |b| {
        b.iter(|| black_box(large_solver.min_edit_distance()))
    });

    // Iterator benchmark
    group.bench_function("iter_pairs_small", |b| {
        b.iter(|| {
            let pairs: Vec<Pair<char, char>> = black_box(&small_solver).into_iter().collect();
            black_box(pairs.len())
        })
    });

    group.finish();
}

fn bench_min_edit_distance_st_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_st_eph");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    // Small sequences with mutation
    group.bench_function("small_with_mutation", |b| {
        b.iter(|| {
            let mut solver = MinEditDistStEphLit!(
                source: ['A', 'B', 'C'],
                target: ['A', 'B', 'D']
            );
            let result1 = black_box(solver.min_edit_distance());
            solver.set_target(2, 'C'); // Change D back to C
            let result2 = black_box(solver.min_edit_distance());
            black_box((result1, result2))
        })
    });

    // Medium sequences
    let mut medium_solver = MinEditDistStEphLit!(
        source: ['h', 'e', 'l', 'l', 'o'],
        target: ['w', 'o', 'r', 'l', 'd']
    );
    group.bench_function("medium_hello_world", |b| {
        b.iter(|| black_box(medium_solver.min_edit_distance()))
    });

    group.finish();
}

fn bench_min_edit_distance_mt_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_mt_per");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    // Small parallel problem
    let small_solver = MinEditDistMtPerLit!(
        source: ['A', 'B', 'C'],
        target: ['X', 'Y', 'Z']
    );
    group.bench_function("small_parallel", |b| {
        b.iter(|| black_box(small_solver.min_edit_distance()))
    });

    // Medium parallel problem
    let medium_solver = MinEditDistMtPerLit!(
        source: ['a', 'l', 'g', 'o', 'r', 'i', 't', 'h', 'm'],
        target: ['l', 'o', 'g', 'a', 'r', 'i', 't', 'h', 'm']
    );
    group.bench_function("medium_parallel_algorithm", |b| {
        b.iter(|| black_box(medium_solver.min_edit_distance()))
    });

    // Sequence access benchmark (Mt types don't support IntoIterator)
    group.bench_function("sequence_access_parallel", |b| {
        b.iter(|| {
            let source = black_box(&small_solver).source();
            let target = black_box(&small_solver).target();
            let mut count = 0;
            for i in 0..source.length() {
                for j in 0..target.length() {
                    if source.nth(i) == target.nth(j) {
                        count += 1;
                    }
                }
            }
            black_box(count)
        })
    });

    group.finish();
}

fn bench_min_edit_distance_mt_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_mt_eph");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    // Small parallel problem with mutation
    group.bench_function("small_parallel_with_mutation", |b| {
        b.iter(|| {
            let mut solver = MinEditDistMtEphLit!(
                source: ['A', 'B', 'C'],
                target: ['A', 'B', 'D']
            );
            let result1 = black_box(solver.min_edit_distance());
            solver.set_source(2, 'D'); // Change C to D
            let result2 = black_box(solver.min_edit_distance());
            black_box((result1, result2))
        })
    });

    // Medium parallel problem
    let mut medium_solver = MinEditDistMtEphLit!(
        source: ['d', 'y', 'n', 'a', 'm', 'i', 'c'],
        target: ['p', 'r', 'o', 'g', 'r', 'a', 'm']
    );
    group.bench_function("medium_parallel_dynamic_program", |b| {
        b.iter(|| black_box(medium_solver.min_edit_distance()))
    });

    group.finish();
}

fn bench_min_edit_distance_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_comparison");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    let source = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let target = vec!['a', 'x', 'c', 'y', 'e', 'z'];

    // Compare all implementations on same problem
    let st_per_solver = MinEditDistStPerS::from_sequences(
        ArraySeqStPerS::from_vec(source.clone()),
        ArraySeqStPerS::from_vec(target.clone()),
    );
    let mut st_eph_solver = MinEditDistStEphS::from_sequences(
        ArraySeqStEphS::from_vec(source.clone()),
        ArraySeqStEphS::from_vec(target.clone()),
    );
    let mt_per_solver = MinEditDistMtPerS::from_sequences(
        ArraySeqMtPerS::from_vec(source.clone()),
        ArraySeqMtPerS::from_vec(target.clone()),
    );
    let mut mt_eph_solver = MinEditDistMtEphS::from_sequences(
        ArraySeqMtEphS::from_vec(source.clone()),
        ArraySeqMtEphS::from_vec(target.clone()),
    );

    group.bench_function("st_per", |b| b.iter(|| black_box(st_per_solver.min_edit_distance())));

    group.bench_function("st_eph", |b| b.iter(|| black_box(st_eph_solver.min_edit_distance())));

    group.bench_function("mt_per", |b| b.iter(|| black_box(mt_per_solver.min_edit_distance())));

    group.bench_function("mt_eph", |b| b.iter(|| black_box(mt_eph_solver.min_edit_distance())));

    group.finish();
}

fn bench_min_edit_distance_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_scaling");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    // Test scaling with sequence length
    for len in [3, 6, 9].iter() {
        let source: Vec<char> = (0..*len).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let target: Vec<char> = (0..*len).map(|i| (b'x' + (i % 3) as u8) as char).collect();

        let solver =
            MinEditDistStPerS::from_sequences(ArraySeqStPerS::from_vec(source), ArraySeqStPerS::from_vec(target));

        group.bench_with_input(BenchmarkId::new("length", len), len, |b, _| {
            b.iter(|| black_box(solver.min_edit_distance()))
        });
    }

    group.finish();
}

fn bench_min_edit_distance_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_edit_distance_edge_cases");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    // Empty sequences
    let empty_solver: MinEditDistStPerS<char> = MinEditDistStPerLit!(
        source: [],
        target: []
    );
    group.bench_function("both_empty", |b| b.iter(|| black_box(empty_solver.min_edit_distance())));

    // One empty, one non-empty
    let one_empty_solver = MinEditDistStPerLit!(
        source: [],
        target: ['A', 'B', 'C']
    );
    group.bench_function("one_empty", |b| {
        b.iter(|| black_box(one_empty_solver.min_edit_distance()))
    });

    // Identical sequences
    let identical_solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C', 'D'],
        target: ['A', 'B', 'C', 'D']
    );
    group.bench_function("identical", |b| {
        b.iter(|| black_box(identical_solver.min_edit_distance()))
    });

    // Completely different sequences
    let different_solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C'],
        target: ['X', 'Y', 'Z']
    );
    group.bench_function("completely_different", |b| {
        b.iter(|| black_box(different_solver.min_edit_distance()))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_min_edit_distance_st_per,
    bench_min_edit_distance_st_eph,
    bench_min_edit_distance_mt_per,
    bench_min_edit_distance_mt_eph,
    bench_min_edit_distance_comparison,
    bench_min_edit_distance_scaling,
    bench_min_edit_distance_edge_cases
);
criterion_main!(benches);
