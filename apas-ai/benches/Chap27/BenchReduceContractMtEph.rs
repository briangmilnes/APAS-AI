//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for parallel reduce using contraction (Chapter 27).

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
use apas_ai::Chap27::ReduceContractMtEph::ReduceContractMtEph::ReduceContractMtEphTrait;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use std::time::Duration;

fn bench_reduce_contract_parallel_100(c: &mut Criterion) {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 100);
    c.bench_function("reduce_contract_mt_100", |b| {
        b.iter(|| ArraySeqMtEphS::reduce_contract_parallel(black_box(&a), Arc::new(|x: &usize, y: &usize| x + y), 0))
    });
}

fn bench_reduce_contract_parallel_1000(c: &mut Criterion) {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 1000);
    c.bench_function("reduce_contract_mt_1000", |b| {
        b.iter(|| ArraySeqMtEphS::reduce_contract_parallel(black_box(&a), Arc::new(|x: &usize, y: &usize| x + y), 0))
    });
}

fn bench_reduce_contract_parallel_10000(c: &mut Criterion) {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 10000);
    c.bench_function("reduce_contract_mt_10000", |b| {
        b.iter(|| ArraySeqMtEphS::reduce_contract_parallel(black_box(&a), Arc::new(|x: &usize, y: &usize| x + y), 0))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(1000))
        .measurement_time(Duration::from_secs(6))
        .sample_size(30);
    targets = bench_reduce_contract_parallel_100, bench_reduce_contract_parallel_1000, bench_reduce_contract_parallel_10000
}

criterion_main!(benches);
