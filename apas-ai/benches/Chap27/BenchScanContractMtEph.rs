//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for parallel scan using contraction (Chapter 27).

use std::sync::Arc;
use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::{
    ArraySeqMtEphBaseTrait, ArraySeqMtEphRedefinableTrait, ArraySeqMtEphS,
};
use apas_ai::Chap27::ScanContractMtEph::ScanContractMtEph::*;

fn bench_scan_contract_parallel_100(c: &mut Criterion) {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 100);
    c.bench_function("scan_contract_mt_100", |b| {
        b.iter(|| ArraySeqMtEphS::scan_contract_parallel(black_box(&a), Arc::new(|x: &usize, y: &usize| x + y), 0))
    });
}

fn bench_scan_contract_parallel_1000(c: &mut Criterion) {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 1000);
    c.bench_function("scan_contract_mt_1000", |b| {
        b.iter(|| ArraySeqMtEphS::scan_contract_parallel(black_box(&a), Arc::new(|x: &usize, y: &usize| x + y), 0))
    });
}

fn bench_scan_contract_parallel_10000(c: &mut Criterion) {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 10000);
    c.bench_function("scan_contract_mt_10000", |b| {
        b.iter(|| ArraySeqMtEphS::scan_contract_parallel(black_box(&a), Arc::new(|x: &usize, y: &usize| x + y), 0))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(300))
        .measurement_time(Duration::from_secs(1))
        .sample_size(30);
    targets = bench_scan_contract_parallel_100, bench_scan_contract_parallel_1000, bench_scan_contract_parallel_10000
}

criterion_main!(benches);
