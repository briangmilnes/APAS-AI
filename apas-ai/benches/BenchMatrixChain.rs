//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Matrix Chain Multiplication using Vec-based implementations.

use apas_ai::Chap50::MatrixChainStEph::MatrixChainStEph::{
    MatrixChainStEphS, MatrixChainStEphTrait, MatrixDim as MatrixChainStEphMatrixDim,
};
use apas_ai::Chap50::MatrixChainStPer::MatrixChainStPer::{MatrixChainStPerS, MatrixChainStPerTrait, MatrixDim};
use apas_ai::MatrixChainStPerLit;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

/// Create random matrix dimensions for benchmarking
fn create_random_dimensions(n: usize) -> Vec<MatrixDim> {
    (0..n)
        .map(|i| {
            let rows = 10 + (i * 7) % 50; // Vary rows from 10 to 59
            let cols = 10 + ((i + 1) * 11) % 50; // Vary cols from 10 to 59
            MatrixDim { rows, cols }
        })
        .collect()
}

/// Create uniform matrix dimensions for benchmarking
fn create_uniform_dimensions(n: usize, rows: usize, cols: usize) -> Vec<MatrixDim> {
    (0..n).map(|_| MatrixDim { rows, cols }).collect()
}

/// Create random matrix dimensions for ephemeral benchmarking
fn create_random_dimensions_eph(n: usize) -> Vec<MatrixChainStEphMatrixDim> {
    (0..n)
        .map(|i| {
            let rows = 10 + (i * 7) % 50; // Vary rows from 10 to 59
            let cols = 10 + ((i + 1) * 11) % 50; // Vary cols from 10 to 59
            MatrixChainStEphMatrixDim { rows, cols }
        })
        .collect()
}

/// Benchmark MatrixChainStPerS::optimal_cost with different input sizes
fn bench_matrix_chain_st_per_optimal_cost(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStPer_optimal_cost");

    for size in [5, 10, 15, 20].iter() {
        let dimensions = create_random_dimensions(*size);
        let chain = MatrixChainStPerS::from_dimensions(dimensions);

        group.bench_with_input(BenchmarkId::new("random", size), size, |b, _| {
            b.iter(|| black_box(chain.optimal_cost()))
        });
    }

    group.finish();
}

/// Benchmark MatrixChainStPerS::optimal_cost with uniform dimensions
fn bench_matrix_chain_st_per_uniform(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStPer_uniform");

    for size in [5, 10, 15, 20].iter() {
        let dimensions = create_uniform_dimensions(*size, 20, 20);
        let chain = MatrixChainStPerS::from_dimensions(dimensions);

        group.bench_with_input(BenchmarkId::new("uniform_20x20", size), size, |b, _| {
            b.iter(|| black_box(chain.optimal_cost()))
        });
    }

    group.finish();
}

/// Benchmark MatrixChainStEphS::optimal_cost with different input sizes
fn bench_matrix_chain_st_eph_optimal_cost(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStEph_optimal_cost");

    for size in [5, 10, 15, 20].iter() {
        let dimensions = create_random_dimensions_eph(*size);
        let mut chain = MatrixChainStEphS::from_dimensions(dimensions);

        group.bench_with_input(BenchmarkId::new("random", size), size, |b, _| {
            b.iter(|| black_box(chain.optimal_cost()))
        });
    }

    group.finish();
}

/// Benchmark MatrixChainStPerS construction from dimensions
fn bench_matrix_chain_st_per_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStPer_construction");

    for size in [10, 50, 100, 200].iter() {
        let dimensions = create_random_dimensions(*size);

        group.bench_with_input(BenchmarkId::new("from_dimensions", size), &dimensions, |b, dims| {
            b.iter(|| black_box(MatrixChainStPerS::from_dimensions(dims.clone())))
        });
    }

    group.finish();
}

/// Benchmark MatrixChainStEphS construction from dimensions
fn bench_matrix_chain_st_eph_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStEph_construction");

    for size in [10, 50, 100, 200].iter() {
        let dimensions = create_random_dimensions_eph(*size);

        group.bench_with_input(BenchmarkId::new("from_dimensions", size), &dimensions, |b, dims| {
            b.iter(|| black_box(MatrixChainStEphS::from_dimensions(dims.clone())))
        });
    }

    group.finish();
}

/// Benchmark MatrixChainStPerS iteration
fn bench_matrix_chain_st_per_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStPer_iteration");

    for size in [100, 500, 1000].iter() {
        let dimensions = create_random_dimensions(*size);
        let chain = MatrixChainStPerS::from_dimensions(dimensions);

        group.bench_with_input(BenchmarkId::new("into_iter", size), &chain, |b, chain| {
            b.iter(|| {
                let sum: usize = chain.clone().into_iter().map(|dim| dim.rows * dim.cols).sum();
                black_box(sum)
            })
        });
    }

    group.finish();
}

/// Benchmark MatrixChainStEphS mutation operations
fn bench_matrix_chain_st_eph_mutation(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChainStEph_mutation");

    group.bench_function("set_dimension", |b| {
        b.iter(|| {
            let dimensions = create_random_dimensions_eph(100);
            let mut chain = MatrixChainStEphS::from_dimensions(dimensions);
            for i in 0..chain.num_matrices() {
                chain.set_dimension(i, MatrixChainStEphMatrixDim { rows: 25, cols: 25 });
            }
            black_box(chain.num_matrices())
        })
    });

    group.finish();
}

/// Benchmark macro construction
fn bench_matrix_chain_macro(c: &mut Criterion) {
    let mut group = c.benchmark_group("MatrixChain_macro");

    group.bench_function("small_macro", |b| {
        b.iter(|| {
            let chain = MatrixChainStPerLit![
                dims: [(10, 20), (20, 30), (30, 40), (40, 50)]
            ];
            black_box(chain.optimal_cost())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_matrix_chain_st_per_optimal_cost,
    bench_matrix_chain_st_per_uniform,
    bench_matrix_chain_st_eph_optimal_cost,
    bench_matrix_chain_st_per_construction,
    bench_matrix_chain_st_eph_construction,
    bench_matrix_chain_st_per_iteration,
    bench_matrix_chain_st_eph_mutation,
    bench_matrix_chain_macro
);

criterion_main!(benches);
