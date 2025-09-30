//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Subset Sum.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

use apas_ai::{
    Chap49::SubsetSumStPer::SubsetSumStPer::*,
    Chap49::SubsetSumStEph::SubsetSumStEph::*,
    Chap49::SubsetSumMtPer::SubsetSumMtPer::*,
    Chap49::SubsetSumMtEph::SubsetSumMtEph::*,
    Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS,
    Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS,
    Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS,
    Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS,
    ArraySeqStPerS, ArraySeqStEphS, ArraySeqMtEphSLit,
    SubsetSumStPerLit, SubsetSumStEphLit, SubsetSumMtPerLit, SubsetSumMtEphLit,
};

fn bench_subset_sum_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_st_per");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    // Small problem size
    let small_solver = SubsetSumStPerLit![1, 2, 3, 4, 5];
    group.bench_function("small_target_8", |b| {
        b.iter(|| black_box(small_solver.subset_sum(black_box(8))))
    });
    
    // Medium problem size
    let medium_solver = SubsetSumStPerLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    group.bench_function("medium_target_25", |b| {
        b.iter(|| black_box(medium_solver.subset_sum(black_box(25))))
    });
    
    // Large problem size
    let large_data: Vec<i32> = (1..=20).collect();
    let large_solver = SubsetSumStPerS::from_multiset(
        ArraySeqStPerS::from_vec(large_data)
    );
    group.bench_function("large_target_100", |b| {
        b.iter(|| black_box(large_solver.subset_sum(black_box(100))))
    });
    
    // Iterator benchmark
    group.bench_function("iter_sum_small", |b| {
        b.iter(|| {
            let sum: i32 = black_box(&small_solver).into_iter().sum();
            black_box(sum)
        })
    });
    
    group.finish();
}

fn bench_subset_sum_st_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_st_eph");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    // Small problem with mutation
    group.bench_function("small_with_mutation", |b| {
        b.iter(|| {
            let mut solver = SubsetSumStEphLit![1, 2, 3, 4, 5];
            let result1 = black_box(solver.subset_sum(black_box(8)));
            solver.set(0, 10); // Mutate first element
            let result2 = black_box(solver.subset_sum(black_box(15)));
            black_box((result1, result2))
        })
    });
    
    // Medium problem size
    let mut medium_solver = SubsetSumStEphLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    group.bench_function("medium_target_30", |b| {
        b.iter(|| black_box(medium_solver.subset_sum(black_box(30))))
    });
    
    group.finish();
}

fn bench_subset_sum_mt_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_mt_per");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    // Small parallel problem
    let small_solver = SubsetSumMtPerLit![1, 2, 3, 4, 5];
    group.bench_function("small_parallel_target_8", |b| {
        b.iter(|| black_box(small_solver.subset_sum(black_box(8))))
    });
    
    // Medium parallel problem
    let medium_solver = SubsetSumMtPerLit![1, 2, 3, 4, 5, 6, 7, 8];
    group.bench_function("medium_parallel_target_20", |b| {
        b.iter(|| black_box(medium_solver.subset_sum(black_box(20))))
    });
    
    // Multiset access benchmark (Mt types don't support IntoIterator)
    group.bench_function("multiset_access_parallel", |b| {
        b.iter(|| {
            let multiset = black_box(&small_solver).multiset();
            let mut sum = 0;
            for i in 0..multiset.length() {
                sum += *multiset.nth(i);
            }
            black_box(sum)
        })
    });
    
    group.finish();
}

fn bench_subset_sum_mt_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_mt_eph");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    // Small parallel problem with mutation
    group.bench_function("small_parallel_with_mutation", |b| {
        b.iter(|| {
            let mut solver = SubsetSumMtEphLit![1, 2, 3, 4, 5];
            let result1 = black_box(solver.subset_sum(black_box(8)));
            solver.set(0, 10); // Mutate first element
            let result2 = black_box(solver.subset_sum(black_box(15)));
            black_box((result1, result2))
        })
    });
    
    // Medium parallel problem
    let mut medium_solver = SubsetSumMtEphLit![1, 2, 3, 4, 5, 6, 7, 8];
    group.bench_function("medium_parallel_target_25", |b| {
        b.iter(|| black_box(medium_solver.subset_sum(black_box(25))))
    });
    
    group.finish();
}

fn bench_subset_sum_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_comparison");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let target = 15;
    
    // Compare all implementations on same problem
    let st_per_solver = SubsetSumStPerS::from_multiset(
        ArraySeqStPerS::from_vec(data.clone())
    );
    let mut st_eph_solver = SubsetSumStEphS::from_multiset(
        ArraySeqStEphS::from_vec(data.clone())
    );
    let mt_per_solver = SubsetSumMtPerS::from_multiset(
        ArraySeqMtPerS::from_vec(data.clone())
    );
    let mut mt_eph_solver = SubsetSumMtEphS::from_multiset(
        ArraySeqMtEphS::from_vec(data.clone())
    );
    
    group.bench_function("st_per", |b| {
        b.iter(|| black_box(st_per_solver.subset_sum(black_box(target))))
    });
    
    group.bench_function("st_eph", |b| {
        b.iter(|| black_box(st_eph_solver.subset_sum(black_box(target))))
    });
    
    group.bench_function("mt_per", |b| {
        b.iter(|| black_box(mt_per_solver.subset_sum(black_box(target))))
    });
    
    group.bench_function("mt_eph", |b| {
        b.iter(|| black_box(mt_eph_solver.subset_sum(black_box(target))))
    });
    
    group.finish();
}

fn bench_subset_sum_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum_scaling");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    
    // Test scaling with problem size
    for size in [5, 10, 15].iter() {
        let data: Vec<i32> = (1..=*size).collect();
        let solver = SubsetSumStPerS::from_multiset(
            ArraySeqStPerS::from_vec(data)
        );
        let target = size * (size + 1) / 4; // Quarter of total sum
        
        group.bench_with_input(
            BenchmarkId::new("size", size),
            size,
            |b, _| {
                b.iter(|| black_box(solver.subset_sum(black_box(target))))
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_subset_sum_st_per,
    bench_subset_sum_st_eph,
    bench_subset_sum_mt_per,
    bench_subset_sum_mt_eph,
    bench_subset_sum_comparison,
    bench_subset_sum_scaling
);
criterion_main!(benches);

