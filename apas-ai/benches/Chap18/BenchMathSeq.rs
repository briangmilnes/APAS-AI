//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::MathSeq::MathSeq::*;
use apas_ai::MathSeqSLit;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_mathseq_basics(c: &mut Criterion) {
    let mut group = c.benchmark_group("MathSeq_ops");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 100_000;

    group.bench_with_input(BenchmarkId::new("new_then_set", n), &n, |b, &len| {
        b.iter(|| {
            let mut s = MathSeqSLit![0; len]; // *Eph: constructor + set pattern
            for i in 0..len {
                let _ = s.set(i, i);
            }
            black_box(s)
        })
    });

    group.bench_with_input(BenchmarkId::new("push_then_pop", n), &n, |b, &len| {
        b.iter(|| {
            let mut s: MathSeqS<N> = MathSeqSLit![]; // *Eph: empty constructor
            for i in 0..len {
                let _ = s.add_last(i);
            }
            for _ in 0..len {
                let _ = s.delete_last();
            }
            black_box(s.length())
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_mathseq_basics);
criterion_main!(benches);
