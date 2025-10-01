//! Copyright © 2025 Russ Eddington. All rights reserved.
//!
//! Benchmarks for SSSPResultStEphInt.

use apas_ai::Chap56::SSSPResultStEphInt::SSSPResultStEphInt;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_new(c: &mut Criterion) {
    let sizes = vec![100, 500, 1000];

    for n in sizes {
        c.bench_function(&format!("new_n{}", n), |b| {
            b.iter(|| black_box(SSSPResultStEphInt::new(black_box(n), black_box(0))))
        });
    }
}

fn bench_set_distance(c: &mut Criterion) {
    let sizes = vec![100, 500, 1000];

    for n in sizes {
        let mut result = SSSPResultStEphInt::new(n, 0);

        c.bench_function(&format!("set_distance_n{}", n), |b| {
            b.iter(|| {
                for i in 0..n {
                    black_box(result.set_distance(black_box(i), black_box(i as i64)));
                }
            })
        });
    }
}

fn bench_set_predecessor(c: &mut Criterion) {
    let sizes = vec![100, 500, 1000];

    for n in sizes {
        let mut result = SSSPResultStEphInt::new(n, 0);

        c.bench_function(&format!("set_predecessor_n{}", n), |b| {
            b.iter(|| {
                for i in 1..n {
                    black_box(result.set_predecessor(black_box(i), black_box(i - 1)));
                }
            })
        });
    }
}

fn bench_extract_path(c: &mut Criterion) {
    let sizes = vec![10, 50, 100];

    for n in sizes {
        let mut result = SSSPResultStEphInt::new(n, 0);
        for i in 1..n {
            result.set_distance(i, i as i64);
            result.set_predecessor(i, i - 1);
        }

        c.bench_function(&format!("extract_path_n{}", n), |b| {
            b.iter(|| black_box(result.extract_path(black_box(n - 1))))
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(1)).measurement_time(std::time::Duration::from_secs(6)).sample_size(30);
    targets = bench_new, bench_set_distance, bench_set_predecessor, bench_extract_path
}
criterion_main!(benches);
