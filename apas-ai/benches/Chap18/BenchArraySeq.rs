//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap18::ArraySeq::ArraySeq::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_arrayseq_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySeq");
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_secs(1));
    
    let n: N = 5_000;
    
    group.bench_with_input(BenchmarkId::new("tabulate", n), &n, |b, &len| {
        b.iter(|| {
            let seq = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i, len);
            black_box(seq)
        })
    });

    group.bench_with_input(BenchmarkId::new("map", n), &n, |b, &len| {
        let seq = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i, len);
        b.iter(|| {
            let mapped = <ArraySeqS<N> as ArraySeq<N>>::map(&seq, &|x| x * 2);
            black_box(mapped)
        })
    });

    group.bench_with_input(BenchmarkId::new("filter", n), &n, |b, &len| {
        let seq = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i, len);
        b.iter(|| {
            let filtered = <ArraySeqS<N> as ArraySeq<N>>::filter(&seq, &|x| if *x % 2 == 0 { true } else { false });
            black_box(filtered)
        })
    });

    group.bench_with_input(BenchmarkId::new("reduce", n), &n, |b, &len| {
        let seq = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i + 1, len);
        b.iter(|| {
            let sum = <ArraySeqS<N> as ArraySeq<N>>::reduce(&seq, &|x, y| x + y, 0);
            black_box(sum)
        })
    });

    group.bench_with_input(BenchmarkId::new("scan", n), &n, |b, &len| {
        let seq = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i + 1, len);
        b.iter(|| {
            let (prefixes, final_sum) = <ArraySeqS<N> as ArraySeq<N>>::scan(&seq, &|x, y| x + y, 0);
            black_box((prefixes, final_sum))
        })
    });

    group.bench_with_input(BenchmarkId::new("append", n), &n, |b, &len| {
        let seq_a = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i, len / 2);
        let seq_b = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i + len / 2, len / 2);
        b.iter(|| {
            let result = <ArraySeqS<N> as ArraySeq<N>>::append(&seq_a, &seq_b);
            black_box(result)
        })
    });

    group.bench_with_input(BenchmarkId::new("subseq", n), &n, |b, &len| {
        let seq = <ArraySeqS<N> as ArraySeq<N>>::tabulate(&|i| i, len);
        b.iter(|| {
            let sub = <ArraySeqS<N> as ArraySeq<N>>::subseq(&seq, len / 4, len / 2);
            black_box(sub)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_arrayseq_operations);
criterion_main!(benches);
