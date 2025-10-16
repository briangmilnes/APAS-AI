//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find Benchmarks (Sequential)

use std::time::Duration;

use criterion::*;

use apas_ai::Types::Types::*;
use apas_ai::Chap65::UnionFindStEph::UnionFindStEph::UnionFindStEph;

fn bench_union_find_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnionFind_Operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    // Benchmark: insert n elements
    group.bench_function("insert_100", |b| {
        b.iter(|| {
            let mut uf: UnionFindStEph<N> = UnionFindStEph::new();
            for i in 0..100 {
                uf.insert(i);
            }
        });
    });

    // Benchmark: union operations (sequential chain)
    group.bench_function("union_chain_100", |b| {
        b.iter(|| {
            let mut uf: UnionFindStEph<N> = UnionFindStEph::new();
            for i in 0..100 {
                uf.insert(i);
            }
            for i in 0..99 {
                uf.union(&i, &(i + 1));
            }
        });
    });

    // Benchmark: find operations
    group.bench_function("find_100", |b| {
        let mut uf: UnionFindStEph<N> = UnionFindStEph::new();
        for i in 0..100 {
            uf.insert(i);
        }
        for i in 0..99 {
            uf.union(&i, &(i + 1));
        }
        b.iter(|| {
            for i in 0..100 {
                let _ = uf.find(&i);
            }
        });
    });

    // Benchmark: equals operations
    group.bench_function("equals_100", |b| {
        let mut uf: UnionFindStEph<N> = UnionFindStEph::new();
        for i in 0..100 {
            uf.insert(i);
        }
        for i in 0..99 {
            uf.union(&i, &(i + 1));
        }
        b.iter(|| {
            for i in 0..50 {
                let _ = uf.equals(&i, &(i + 50));
            }
        });
    });

    // Benchmark: mixed operations (realistic workload)
    group.bench_function("mixed_operations_100", |b| {
        b.iter(|| {
            let mut uf: UnionFindStEph<N> = UnionFindStEph::new();
            for i in 0..100 {
                uf.insert(i);
            }
            for i in 0..50 {
                uf.union(&(i * 2), &(i * 2 + 1));
            }
            for i in 0..100 {
                let _ = uf.find(&i);
            }
            for i in 0..50 {
                let _ = uf.equals(&i, &(i + 1));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_union_find_operations);
criterion_main!(benches);
