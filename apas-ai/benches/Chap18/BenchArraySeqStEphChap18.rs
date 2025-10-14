//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
use apas_ai::Types::Types::*;
use criterion::*;
use std::time::Duration;

fn bench_tabulate_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqStEphChap18");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    let n: N = 5_000;

    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i, len);
            let m: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::map(&s, &|x| x + 1);
            black_box((s.length(), m.length()))
        })
    });

    group.bench_with_input(BenchmarkId::new("subseq", n), &n, |b, &len| {
        let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i, len);
        b.iter(|| {
            let sub = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::subseq(&s, len / 4, len / 2);
            black_box(sub)
        })
    });

    group.bench_with_input(BenchmarkId::new("append", n), &n, |b, &len| {
        let a: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i, len / 2);
        let seq_b: ArraySeqStEphS<N> =
            <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i + len / 2, len / 2);
        b.iter(|| {
            let result = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::append(&a, &seq_b);
            black_box(result)
        })
    });

    group.bench_with_input(BenchmarkId::new("filter", n), &n, |b, &len| {
        let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i, len);
        b.iter(|| {
            let evens =
                <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::filter(&s, &|x| if *x % 2 == 0 { true } else { false });
            black_box(evens)
        })
    });

    group.bench_with_input(BenchmarkId::new("flatten", n / 10), &(n / 10), |b, &len| {
        let nested: ArraySeqStEphS<ArraySeqStEphS<N>> =
            <ArraySeqStEphS<ArraySeqStEphS<N>> as ArraySeqStEphTrait<ArraySeqStEphS<N>>>::tabulate(
                &|i| <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|j| i * 10 + j, 10),
                len,
            );
        b.iter(|| {
            let flat = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::flatten(&nested);
            black_box(flat)
        })
    });

    group.bench_with_input(BenchmarkId::new("reduce", n), &n, |b, &len| {
        let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i + 1, len);
        b.iter(|| {
            let sum = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::reduce(&s, &|x, y| x + y, 0);
            black_box(sum)
        })
    });

    group.bench_with_input(BenchmarkId::new("scan", n), &n, |b, &len| {
        let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i + 1, len);
        b.iter(|| {
            let (prefixes, final_sum) = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::scan(&s, &|x, y| x + y, 0);
            black_box((prefixes, final_sum))
        })
    });

    group.bench_with_input(BenchmarkId::new("iterate", n), &n, |b, &len| {
        let s: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::tabulate(&|i| i + 1, len);
        b.iter(|| {
            let sum = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::iterate(&s, &|acc, x| acc + x, 0);
            black_box(sum)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_tabulate_map);
criterion_main!(benches);
