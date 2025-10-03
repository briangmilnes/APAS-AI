//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for MatrixChainStPer

use apas_ai::Chap50::MatrixChainStPer::MatrixChainStPer::{MatrixChainStPerS, MatrixChainStPerTrait, MatrixDim};
use apas_ai::MatrixChainStPerLit;
use criterion::*;
use std::time::Duration;

fn create_random_dimensions(n: usize) -> Vec<MatrixDim> {
    (0..n)
        .map(|i| {
            let rows = 10 + (i * 7) % 50;
            let cols = 10 + ((i + 1) * 11) % 50;
            MatrixDim { rows, cols }
        })
        .collect()
}

fn bench_matrix_chain_st_per_optimal_cost(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStPer_optimal_cost");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [5, 10, 15, 20].iter() {
        let dimensions = create_random_dimensions(*size);
        let chain = MatrixChainStPerS::from_dimensions(dimensions);

        group.bench_with_input(BenchmarkId::new("random", size), size, |b, _| {
            b.iter(|| black_box(chain.optimal_cost()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_matrix_chain_st_per_optimal_cost);
criterion_main!(benches);
