//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chap53 PQMinMtEph.

use apas_ai::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
use apas_ai::Chap53::PQMinMtEph::PQMinMtEph::*;
use apas_ai::Types::Types::*;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::time::Duration;

fn vertex_priority() -> ClosurePriority<N, N, impl Fn(&N) -> N + Send + Sync + 'static> {
    ClosurePriority::new(|v: &N| *v)
}

fn build_complete_graph(n: N) -> impl Fn(&N) -> AVLTreeSetMtEph<N> + Send + Sync + 'static {
    move |v: &N| {
        let mut neighbors = AVLTreeSetMtEph::empty();
        for i in 1..=n {
            if i != *v {
                neighbors.insert(i);
            }
        }
        neighbors
    }
}

fn build_chain_graph(n: N) -> impl Fn(&N) -> AVLTreeSetMtEph<N> + Send + Sync + 'static {
    move |v: &N| {
        if *v < n {
            AVLTreeSetMtEph::singleton(v + 1)
        } else {
            AVLTreeSetMtEph::empty()
        }
    }
}

fn bench_pq_min_mt_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("PQMinMtEph");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    group.bench_function("complete_graph_n=15", |b| {
        b.iter_batched(
            || (build_complete_graph(15), vertex_priority()),
            |(graph, prio_fn)| PQMinMtEph::pq_min(graph, 1, prio_fn),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("chain_graph_n=100", |b| {
        b.iter_batched(
            || (build_chain_graph(100), vertex_priority()),
            |(graph, prio_fn)| PQMinMtEph::pq_min(graph, 1, prio_fn),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("dag_n=50", |b| {
        b.iter_batched(
            || {
                let graph = |v: &N| {
                    if *v < 50 {
                        AVLTreeSetMtEph::singleton(v + 1)
                            .union(&AVLTreeSetMtEph::singleton(v + 2))
                    } else {
                        AVLTreeSetMtEph::empty()
                    }
                };
                (graph, vertex_priority())
            },
            |(graph, prio_fn)| PQMinMtEph::pq_min(graph, 1, prio_fn),
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_pq_min_mt_eph);
criterion_main!(benches);
