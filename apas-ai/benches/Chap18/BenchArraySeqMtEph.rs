//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_arrayseqmteph_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySeqMtEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 2_500;

    group.bench_with_input(BenchmarkId::new("new", n), &n, |b, &len| {
        b.iter(|| {
            let seq = ArraySeqMtEphS::new(len, 42);
            black_box(seq)
        })
    });

    group.bench_with_input(BenchmarkId::new("length", n), &n, |b, &len| {
        let seq = ArraySeqMtEphS::new(len, 42);
        b.iter(|| {
            let l = seq.length();
            black_box(l)
        })
    });

    group.bench_with_input(BenchmarkId::new("nth_cloned", n), &n, |b, &len| {
        let seq = ArraySeqMtEphS::new(len, 42);
        b.iter(|| {
            let mut sum = 0;
            for i in 0..len {
                sum += seq.nth_cloned(i);
            }
            black_box(sum)
        })
    });

    group.bench_with_input(BenchmarkId::new("subseq_copy", n), &n, |b, &len| {
        let seq = ArraySeqMtEphS::new(len, 42);
        b.iter(|| {
            let sub = seq.subseq_copy(len / 4, len / 2);
            black_box(sub)
        })
    });

    group.bench_with_input(BenchmarkId::new("append", n), &n, |b, &len| {
        let seq_a = ArraySeqMtEphS::new(len / 2, 42);
        let seq_b = ArraySeqMtEphS::new(len / 2, 84);
        b.iter(|| {
            let result = <ArraySeqMtEphS<N> as ArraySeqMtEphTrait<N>>::append(&seq_a, &seq_b);
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_arrayseqmteph_operations);
criterion_main!(benches);
