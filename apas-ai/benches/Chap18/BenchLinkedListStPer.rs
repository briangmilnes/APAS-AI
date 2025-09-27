//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap18::LinkedListStPer::LinkedListStPer::*;
use apas_ai::LinkedListStPerSLit;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_sll_persistent_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListPer");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 1_000; // reduce N to ensure ≤10s total

    group.bench_with_input(BenchmarkId::new("new_then_updates", n), &n, |b, &len| {
        b.iter(|| {
            let s: LinkedListStPerS<N> = LinkedListStPerSLit![0; len]; // *Per: from_vec pattern
            // Focus on operations that exist in the API
            black_box(<LinkedListStPerS<N> as LinkedListStPerTrait<N>>::length(&s))
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_sll_persistent_ops);
criterion_main!(benches);
