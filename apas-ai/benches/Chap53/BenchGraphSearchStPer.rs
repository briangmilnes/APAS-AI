//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chap53 GraphSearchStPer.

use apas_ai::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_ai::Chap53::GraphSearchStPer::GraphSearchStPer::*;
use apas_ai::Types::Types::*;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::time::Duration;

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

fn bench_graph_search_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("GraphSearchStPer");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    group.bench_function("complete_graph_n=20", |b| {
        b.iter_batched(
            || build_complete_graph(20),
            |graph| GraphSearchStPer::graph_search(&graph, 1, &SelectAll),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("chain_graph_n=100", |b| {
        b.iter_batched(
            || build_chain_graph(100),
            |graph| GraphSearchStPer::graph_search(&graph, 1, &SelectAll),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("reachable_complete_n=15", |b| {
        b.iter_batched(
            || build_complete_graph(15),
            |graph| GraphSearchStPer::reachable(&graph, 1),
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_graph_search_st_per);
criterion_main!(benches);
