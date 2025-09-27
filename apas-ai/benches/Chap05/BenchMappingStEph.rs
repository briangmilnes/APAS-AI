//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::MappingStEph::MappingStEph::*;
use apas_ai::Chap05::RelationStEph::RelationStEph::*;
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Types::Types::*;
use apas_ai::{PairLit, SetLit};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_mapping_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchMappingEphChap5_5");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(1000));

    let n: N = 50_000;
    group.bench_with_input(BenchmarkId::new("from_relation_overwrite_dups", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<Pair<N, N>> = SetLit![]; // Set: empty constructor
            for i in 0..len {
                let _ = Set::insert(&mut pairs, PairLit!(i % 10_000, i));
            }
            let r = Relation::FromSet(pairs);
            let m = Mapping::FromRelation(&r);
            black_box((r, m))
        })
    });

    // Add domain() benchmark - O(|m|) operation
    group.bench_with_input(BenchmarkId::new("domain", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<Pair<N, N>> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut pairs, PairLit!(i % 10_000, i));
            }
            let r = Relation::FromSet(pairs);
            let m = Mapping::FromRelation(&r);
            let domain_set = m.domain();
            black_box(domain_set)
        })
    });

    // Add range() benchmark - O(|m|) operation
    group.bench_with_input(BenchmarkId::new("range", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<Pair<N, N>> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut pairs, PairLit!(i % 10_000, i));
            }
            let r = Relation::FromSet(pairs);
            let m = Mapping::FromRelation(&r);
            let range_set = m.range();
            black_box(range_set)
        })
    });

    // Add mem() benchmark - O(1) but worth testing for hash performance
    group.bench_with_input(BenchmarkId::new("mem", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<Pair<N, N>> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut pairs, PairLit!(i % 10_000, i));
            }
            let r = Relation::FromSet(pairs);
            let m = Mapping::FromRelation(&r);
            let hit = m.mem(&(len / 2), &(len / 2));
            black_box(hit)
        })
    });

    // Add iter() benchmark - O(|m|) operation
    group.bench_with_input(BenchmarkId::new("iter", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<Pair<N, N>> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut pairs, PairLit!(i % 10_000, i));
            }
            let r = Relation::FromSet(pairs);
            let m = Mapping::FromRelation(&r);
            let count = m.iter().count();
            black_box(count)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_mapping_build);
criterion_main!(benches);
