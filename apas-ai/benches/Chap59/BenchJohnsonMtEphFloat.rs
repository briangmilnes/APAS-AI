use std::time::Duration;

use criterion::*;

use apas_ai::Types::Types::OrderedF64;
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
use apas_ai::Chap59::JohnsonMtEphFloat::JohnsonMtEphFloat::johnson_apsp;
use apas_ai::SetLit;

fn generate_sparse_graph_negative(n: usize) -> WeightedDirGraphMtEphFloat<usize> {
    let mut vertices = Set::empty();
    for i in 0..n {
        vertices.insert(i);
    }

    let mut edges = Set::empty();
    for i in 0..n {
        let j = (i + 1) % n;
        let weight = if i % 3 == 0 {
            OrderedF64::from(-2.5)
        } else {
            OrderedF64::from(5.2)
        };
        edges.insert((i, j, weight));

        if i < n - 1 {
            let k = (i + 2) % n;
            edges.insert((i, k, OrderedF64::from(8.3)));
        }
    }

    WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges)
}

fn generate_dense_graph_negative(n: usize) -> WeightedDirGraphMtEphFloat<usize> {
    let mut vertices = Set::empty();
    for i in 0..n {
        vertices.insert(i);
    }

    let mut edges = Set::empty();
    for i in 0..n {
        for j in 0..n {
            if i != j && (i * 7 + j * 11) % 2 == 0 {
                let weight = if (i + j) % 3 == 0 {
                    OrderedF64::from(-1.5)
                } else {
                    OrderedF64::from(3.7)
                };
                edges.insert((i, j, weight));
            }
        }
    }

    WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges)
}

fn bench_johnson_sparse(c: &mut Criterion) {
    let mut group = c.benchmark_group("johnson_mt_eph_float_sparse");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20, 30].iter() {
        let graph = generate_sparse_graph_negative(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = johnson_apsp(black_box(&graph));
                black_box(result);
            });
        });
    }
    group.finish();
}

fn bench_johnson_dense(c: &mut Criterion) {
    let mut group = c.benchmark_group("johnson_mt_eph_float_dense");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 20, 30].iter() {
        let graph = generate_dense_graph_negative(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = johnson_apsp(black_box(&graph));
                black_box(result);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_johnson_sparse, bench_johnson_dense);
criterion_main!(benches);
