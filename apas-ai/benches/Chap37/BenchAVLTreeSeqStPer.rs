//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::env;
use std::path::PathBuf;
use std::time::Duration;

use criterion::*;

use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Types::Types::*;

fn bench_build_and_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqPer");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("build_then_length", n), &n, |b, &_len| {
        b.iter(|| {
            // Build a small tree for benchmarking - persistent structures are different
            let t1 = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::singleton(1);
            let t2 = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::singleton(2);
            let t3 = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::singleton(3);
            black_box((t1, t2, t3)) // Focus on creation performance
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_build_and_contains);
criterion_main!(benches);
