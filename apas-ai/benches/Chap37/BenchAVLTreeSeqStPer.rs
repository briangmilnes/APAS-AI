//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::AVLTreeSeqStPerTrait;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_build_and_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqPer");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("tabulate_then_contains", n), &n, |b, &len| {
        b.iter(|| {
            let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::tabulate(|i| i, len);
            let hit = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::isSingleton(&t) == B::True; // cheap read
            black_box((t.length(), hit))
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
