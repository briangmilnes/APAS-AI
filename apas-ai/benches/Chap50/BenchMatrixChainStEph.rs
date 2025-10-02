//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for MatrixChainStEph

use apas_ai::Chap50::MatrixChainStEph::MatrixChainStEph::{
    MatrixChainStEphS, MatrixChainStEphTrait, MatrixDim as MatrixChainStEphMatrixDim,
};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn create_random_dimensions_eph(n: usize) -> Vec<MatrixChainStEphMatrixDim> {
    (0..n)
        .map(|i| {
            let rows = 10 + (i * 7) % 50;
            let cols = 10 + ((i + 1) * 11) % 50;
            MatrixChainStEphMatrixDim { rows, cols }
        })
        .collect()
}

fn bench_matrix_chain_st_eph_optimal_cost(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStEph_optimal_cost");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [5, 10, 15, 20].iter() {
        let dimensions = create_random_dimensions_eph(*size);
        let mut chain = MatrixChainStEphS::from_dimensions(dimensions);

        group.bench_with_input(BenchmarkId::new("random", size), size, |b, _| {
            b.iter(|| black_box(chain.optimal_cost()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_matrix_chain_st_eph_optimal_cost);
criterion_main!(benches);
