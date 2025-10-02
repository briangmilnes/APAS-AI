//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_undirgraph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchUnDirGraphEphChap6_1");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 50_000;
    group.bench_with_input(BenchmarkId::new("build_vertices_edges", n), &n, |b, &len| {
        b.iter(|| {
            let mut v: Set<N> = SetLit![]; // Set: empty constructor
            for i in 0..len {
                let _ = Set::insert(&mut v, i);
            }
            let mut e: Set<Edge<N>> = SetLit![]; // Set: empty constructor
            for i in 0..len {
                let _ = Set::insert(&mut e, Edge(i.min((i + 1) % len), i.max((i + 1) % len)));
            }
            let g = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(v, e);
            black_box(g)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_undirgraph_build);
criterion_main!(benches);
