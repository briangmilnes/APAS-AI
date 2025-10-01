//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
use apas_ai::Chap54::BFSStEph::BFSStEph;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn create_dag(n: usize) -> ArraySeqStEphS<ArraySeqStEphS<usize>> {
    ArraySeqStEphS::tabulate(
        &|i| {
            if i < n - 1 {
                ArraySeqStEphS::from_vec(vec![i + 1, (i + 2).min(n - 1)])
            } else {
                ArraySeqStEphS::from_vec(vec![])
            }
        },
        n,
    )
}

pub fn bench_bfs_st_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap54::BFSStEph");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    let g100 = create_dag(100);
    group.bench_function("bfs_dag_100", |b| {
        b.iter(|| BFSStEph::bfs(black_box(&g100), black_box(0)))
    });

    let g1000 = create_dag(1000);
    group.bench_function("bfs_dag_1000", |b| {
        b.iter(|| BFSStEph::bfs(black_box(&g1000), black_box(0)))
    });

    group.finish();
}

criterion_group!(benches, bench_bfs_st_eph);
criterion_main!(benches);
