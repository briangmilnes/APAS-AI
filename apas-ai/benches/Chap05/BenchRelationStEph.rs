//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::RelationStEph::RelationStEph::*;
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Types::Types::*;
use apas_ai::{PairLit, SetLit};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_relation_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("RelationStEph");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("FromSet", n), &n, |b, &len| {
        let pairs: Set<Pair<N, N>> = Set::FromVec((0..len).map(|i| PairLit![i, i * 2]).collect());
        b.iter(|| {
            let relation = Relation::FromSet(pairs.clone());
            black_box(relation)
        })
    });

    group.bench_with_input(BenchmarkId::new("domain", n), &n, |b, &len| {
        let pairs: Set<Pair<N, N>> = Set::FromVec((0..len).map(|i| PairLit![i, i * 2]).collect());
        let relation = Relation::FromSet(pairs);
        b.iter(|| {
            let dom = relation.domain();
            black_box(dom)
        })
    });

    group.bench_with_input(BenchmarkId::new("range", n), &n, |b, &len| {
        let pairs: Set<Pair<N, N>> = Set::FromVec((0..len).map(|i| PairLit![i, i * 2]).collect());
        let relation = Relation::FromSet(pairs);
        b.iter(|| {
            let rng = relation.range();
            black_box(rng)
        })
    });

    group.bench_with_input(BenchmarkId::new("mem", n), &n, |b, &len| {
        let pairs: Set<Pair<N, N>> = Set::FromVec((0..len).map(|i| PairLit![i, i * 2]).collect());
        let relation = Relation::FromSet(pairs);
        b.iter(|| {
            let mut found = 0;
            for i in 0..len {
                if relation.mem(&i, &(i * 2)) == true {
                    found += 1;
                }
            }
            black_box(found)
        })
    });

    group.bench_with_input(BenchmarkId::new("iter", n), &n, |b, &len| {
        let pairs: Set<Pair<N, N>> = Set::FromVec((0..len).map(|i| PairLit![i, i * 2]).collect());
        let relation = Relation::FromSet(pairs);
        b.iter(|| {
            let mut count = 0;
            for _pair in relation.iter() {
                count += 1;
            }
            black_box(count)
        })
    });

    group.bench_with_input(BenchmarkId::new("size", n), &n, |b, &len| {
        let pairs: Set<Pair<N, N>> = Set::FromVec((0..len).map(|i| PairLit![i, i * 2]).collect());
        let relation = Relation::FromSet(pairs);
        b.iter(|| {
            let sz = relation.size();
            black_box(sz)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_relation_operations);
criterion_main!(benches);
