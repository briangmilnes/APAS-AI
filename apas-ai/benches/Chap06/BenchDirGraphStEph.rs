//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::DirGraphStEph::DirGraphStEph::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use criterion::*;
use std::time::Duration;

fn bench_dirgraph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchDirGraphStEphChap6_1");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("build_vertices_arcs", n), &n, |b, &len| {
        b.iter(|| {
            let mut v: Set<N> = SetLit![]; // Set: empty constructor
            for i in 0..len {
                let _ = Set::insert(&mut v, i);
            }
            let mut a: Set<Edge<N>> = SetLit![]; // Set: empty constructor
            for i in 0..len {
                let _ = Set::insert(&mut a, Edge(i, (i + 1) % len));
            }
            let g = DirGraphStEph::FromSets(v, a);
            black_box(g)
        })
    });

    // Add vertex operations benchmark - O(1) but worth testing
    group.bench_with_input(BenchmarkId::new("vertex_operations", n), &n, |b, &len| {
        b.iter(|| {
            let mut v: Set<N> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut v, i);
            }
            let mut a: Set<Edge<N>> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut a, Edge(i, (i + 1) % len));
            }
            let g = DirGraphStEph::FromSets(v, a);
            let vertex_count = g.sizeV();
            let arc_count = g.sizeA();
            black_box((vertex_count, arc_count))
        })
    });

    // Add neighbor operations benchmark - O(|A|) operation
    group.bench_with_input(BenchmarkId::new("neighbor_operations", n), &n, |b, &len| {
        b.iter(|| {
            let mut v: Set<N> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut v, i);
            }
            let mut a: Set<Edge<N>> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut a, Edge(i, (i + 1) % len));
            }
            let g = DirGraphStEph::FromSets(v, a);
            let neighbors = g.NG(&(len / 2));
            black_box(neighbors)
        })
    });

    // Add edge checking benchmark - O(1) operation
    group.bench_with_input(BenchmarkId::new("edge_checking", n), &n, |b, &len| {
        b.iter(|| {
            let mut v: Set<N> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut v, i);
            }
            let mut a: Set<Edge<N>> = SetLit![];
            for i in 0..len {
                let _ = Set::insert(&mut a, Edge(i, (i + 1) % len));
            }
            let g = DirGraphStEph::FromSets(v, a);
            let mut edge_count = 0;
            for i in 0..len {
                if g.Neighbor(&i, &((i + 1) % len)) {
                    edge_count += 1;
                }
            }
            black_box(edge_count)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_dirgraph_build);
criterion_main!(benches);
