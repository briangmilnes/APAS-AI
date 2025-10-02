//! Copyright © 2025 Russ Eddington. All rights reserved.
//!
//! Benchmarks for PathWeightUtilsStEph.

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;
use apas_ai::Chap56::PathWeightUtilsStEph::PathWeightUtilsStEph::{
    path_weight_float, path_weight_int, validate_subpath_property_float, validate_subpath_property_int,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use ordered_float::OrderedFloat;
use std::time::Duration;

fn bench_path_weight_int(c: &mut Criterion) {
    let sizes = vec![10, 50, 100];

    for n in sizes {
        let mut weights = vec![];
        for i in 0..n {
            let mut row = vec![i64::MAX; n];
            if i + 1 < n {
                row[i + 1] = 1;
            }
            weights.push(ArraySeqStEphS::from_vec(row));
        }
        let weights = ArraySeqStEphS::from_vec(weights);

        let path_vec: Vec<usize> = (0..n).collect();
        let path = ArraySeqStPerS::from_vec(path_vec);

        c.bench_function(&format!("path_weight_int_n{}", n), |b| {
            b.iter(|| black_box(path_weight_int(black_box(&path), black_box(&weights))))
        });
    }
}

fn bench_path_weight_float(c: &mut Criterion) {
    let sizes = vec![10, 50, 100];

    for n in sizes {
        let mut weights = vec![];
        for i in 0..n {
            let mut row = vec![OrderedFloat(f64::INFINITY); n];
            if i + 1 < n {
                row[i + 1] = OrderedFloat(1.5);
            }
            weights.push(ArraySeqStEphS::from_vec(row));
        }
        let weights = ArraySeqStEphS::from_vec(weights);

        let path_vec: Vec<usize> = (0..n).collect();
        let path = ArraySeqStPerS::from_vec(path_vec);

        c.bench_function(&format!("path_weight_float_n{}", n), |b| {
            b.iter(|| black_box(path_weight_float(black_box(&path), black_box(&weights))))
        });
    }
}

fn bench_validate_subpath_int(c: &mut Criterion) {
    let sizes = vec![10, 50, 100];

    for n in sizes {
        let mut weights = vec![];
        for i in 0..n {
            let mut row = vec![i64::MAX; n];
            if i + 1 < n {
                row[i + 1] = 1;
            }
            weights.push(ArraySeqStEphS::from_vec(row));
        }
        let weights = ArraySeqStEphS::from_vec(weights);

        let distances_vec: Vec<i64> = (0..n as i64).collect();
        let distances = ArraySeqStEphS::from_vec(distances_vec);

        let path_vec: Vec<usize> = (0..n).collect();
        let path = ArraySeqStPerS::from_vec(path_vec);

        c.bench_function(&format!("validate_subpath_int_n{}", n), |b| {
            b.iter(|| {
                black_box(validate_subpath_property_int(
                    black_box(&path),
                    black_box(&distances),
                    black_box(&weights),
                ))
            })
        });
    }
}

fn bench_validate_subpath_float(c: &mut Criterion) {
    let sizes = vec![10, 50, 100];

    for n in sizes {
        let mut weights = vec![];
        for i in 0..n {
            let mut row = vec![OrderedFloat(f64::INFINITY); n];
            if i + 1 < n {
                row[i + 1] = OrderedFloat(1.5);
            }
            weights.push(ArraySeqStEphS::from_vec(row));
        }
        let weights = ArraySeqStEphS::from_vec(weights);

        let distances_vec: Vec<OrderedFloat<f64>> = (0..n).map(|i| OrderedFloat(i as f64 * 1.5)).collect();
        let distances = ArraySeqStEphS::from_vec(distances_vec);

        let path_vec: Vec<usize> = (0..n).collect();
        let path = ArraySeqStPerS::from_vec(path_vec);

        c.bench_function(&format!("validate_subpath_float_n{}", n), |b| {
            b.iter(|| {
                black_box(validate_subpath_property_float(
                    black_box(&path),
                    black_box(&distances),
                    black_box(&weights),
                ))
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_millis(300)).measurement_time(std::time::Duration::from_secs(1)).sample_size(30);
    targets = bench_path_weight_int, bench_path_weight_float, bench_validate_subpath_int, bench_validate_subpath_float
}
criterion_main!(benches);
