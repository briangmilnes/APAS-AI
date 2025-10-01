//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_build_and_read_persistent(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqPer");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("tabulate_then_nth", n), &n, |b, &len| {
        b.iter(|| {
            let mut t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::empty();
            for i in 0..len {
                t = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::set(&t, i, i).unwrap();
            }
            let v = *<AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::nth(&t, len - 1);
            black_box((t.length(), v))
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_build_and_read_persistent);
criterion_main!(benches);
