//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chap53 PQMinStPer.

use apas_ai::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_ai::Chap53::PQMinStPer::PQMinStPer::*;
use apas_ai::Types::Types::*;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::time::Duration;

fn vertex_priority() -> ClosurePriority<N, N, impl Fn(&N) -> N> {
    ClosurePriority::new(|v: &N| *v)
}

fn build_complete_graph(n: N) -> impl Fn(&N) -> AVLTreeSetStPer<N> {
    move |v: &N| {
        let mut neighbors = AVLTreeSetStPer::empty();
        for i in 1..=n {
            if i != *v {
                neighbors = neighbors.union(&AVLTreeSetStPer::singleton(i));
            }
        }
        neighbors
    }
}

fn build_chain_graph(n: N) -> impl Fn(&N) -> AVLTreeSetStPer<N> {
    move |v: &N| {
        if *v < n {
            AVLTreeSetStPer::singleton(v + 1)
        } else {
            AVLTreeSetStPer::empty()
        }
    }
}

fn bench_pq_min_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("PQMinStPer");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    group.bench_function("complete_graph_n=15", |b| {
        b.iter_batched(
            || (build_complete_graph(15), vertex_priority()),
            |(graph, prio_fn)| PQMinStPer::pq_min(&graph, 1, &prio_fn),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("chain_graph_n=100", |b| {
        b.iter_batched(
            || (build_chain_graph(100), vertex_priority()),
            |(graph, prio_fn)| PQMinStPer::pq_min(&graph, 1, &prio_fn),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("dag_n=50", |b| {
        b.iter_batched(
            || {
                let graph = |v: &N| {
                    if *v < 50 {
                        AVLTreeSetStPer::singleton(v + 1)
                            .union(&AVLTreeSetStPer::singleton(v + 2))
                    } else {
                        AVLTreeSetStPer::empty()
                    }
                };
                (graph, vertex_priority())
            },
            |(graph, prio_fn)| PQMinStPer::pq_min(&graph, 1, &prio_fn),
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_pq_min_st_per);
criterion_main!(benches);
