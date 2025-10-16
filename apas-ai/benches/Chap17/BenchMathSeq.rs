//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::env;
use std::path::PathBuf;
use std::time::Duration;

use criterion::*;

use apas_ai::Chap17::MathSeq::MathSeq::*;
use apas_ai::MathSeqSLit;
use apas_ai::Types::Types::*;

fn bench_mathseq_basics(c: &mut Criterion) {
    let mut group = c.benchmark_group("MathSeq_ops");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    let n: N = 10_000;

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

    group.bench_with_input(BenchmarkId::new("subseq_copy", n), &n, |b, &len| {
        let s = MathSeqSLit![42; len];
        b.iter(|| {
            let sub = s.subseq_copy(len / 4, len / 2);
            black_box(sub)
        })
    });

    group.bench_with_input(BenchmarkId::new("domain", n), &n, |b, &len| {
        let s = MathSeqSLit![42; len];
        b.iter(|| {
            let dom = s.domain();
            black_box(dom)
        })
    });

    group.bench_with_input(BenchmarkId::new("range", n), &n, |b, &len| {
        let mut s = MathSeqSLit![0; len];
        for i in 0..len {
            let _ = s.set(i, i % 100); // Create some duplicates
        }
        b.iter(|| {
            let rng = s.range();
            black_box(rng)
        })
    });

    group.bench_with_input(BenchmarkId::new("multiset_range", n), &n, |b, &len| {
        let mut s = MathSeqSLit![0; len];
        for i in 0..len {
            let _ = s.set(i, i % 50); // Create duplicates for counting
        }
        b.iter(|| {
            let multi = s.multiset_range();
            black_box(multi)
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
