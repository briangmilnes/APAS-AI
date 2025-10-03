//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for sequential reduce using contraction (Chapter 27).

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
use apas_ai::Chap27::ReduceContractStEph::ReduceContractStEph::ReduceContractStEphTrait;
use criterion::*;
use std::time::Duration;

fn bench_reduce_contract_100(c: &mut Criterion) {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 100);
    c.bench_function("reduce_contract_st_100", |b| {
        b.iter(|| ArraySeqStEphS::reduce_contract(black_box(&a), &|x, y| x + y, 0))
    });
}

fn bench_reduce_contract_1000(c: &mut Criterion) {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 1000);
    c.bench_function("reduce_contract_st_1000", |b| {
        b.iter(|| ArraySeqStEphS::reduce_contract(black_box(&a), &|x, y| x + y, 0))
    });
}

fn bench_reduce_contract_10000(c: &mut Criterion) {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 10000);
    c.bench_function("reduce_contract_st_10000", |b| {
        b.iter(|| ArraySeqStEphS::reduce_contract(black_box(&a), &|x, y| x + y, 0))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(300))
        .measurement_time(Duration::from_secs(1))
        .sample_size(30);
    targets = bench_reduce_contract_100, bench_reduce_contract_1000, bench_reduce_contract_10000
}

criterion_main!(benches);
