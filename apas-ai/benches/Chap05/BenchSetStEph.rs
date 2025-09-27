//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_set_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("SetStEph");
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("union", n), &n, |b, &len| {
        let set_a: Set<N> = Set::FromVec((0..len).collect());
        let set_b: Set<N> = Set::FromVec((len / 2..len + len / 2).collect());
        b.iter(|| {
            let result = set_a.union(&set_b);
            black_box(result)
        })
    });

    group.bench_with_input(BenchmarkId::new("intersection", n), &n, |b, &len| {
        let set_a: Set<N> = Set::FromVec((0..len).collect());
        let set_b: Set<N> = Set::FromVec((len / 2..len + len / 2).collect());
        b.iter(|| {
            let result = set_a.intersection(&set_b);
            black_box(result)
        })
    });

    group.bench_with_input(BenchmarkId::new("CartesianProduct", n / 10), &(n / 10), |b, &len| {
        let set_a: Set<N> = Set::FromVec((0..len).collect());
        let set_b: Set<N> = Set::FromVec((0..len).collect());
        b.iter(|| {
            let result = set_a.CartesianProduct(&set_b);
            black_box(result)
        })
    });

    group.bench_with_input(BenchmarkId::new("partition", n / 20), &(n / 20), |b, &len| {
        let main_set: Set<N> = Set::FromVec((0..len * 4).collect());
        let mut parts: Set<Set<N>> = SetLit![];
        for i in 0..4 {
            let subset: Set<N> = Set::FromVec((i * len..(i + 1) * len).collect());
            let _ = parts.insert(subset);
        }
        b.iter(|| {
            let result = main_set.partition(&parts);
            black_box(result)
        })
    });

    group.bench_with_input(BenchmarkId::new("FromVec", n), &n, |b, &len| {
        let vec_data: Vec<N> = (0..len).collect();
        b.iter(|| {
            let result = Set::FromVec(vec_data.clone());
            black_box(result)
        })
    });

    group.bench_with_input(BenchmarkId::new("mem", n), &n, |b, &len| {
        let set: Set<N> = Set::FromVec((0..len).collect());
        b.iter(|| {
            let mut found = 0;
            for i in 0..len {
                if set.mem(&i) == true {
                    found += 1;
                }
            }
            black_box(found)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_set_operations);
criterion_main!(benches);
