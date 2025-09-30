//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Optimal Binary Search Tree implementations using Vec-based data structures.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_ai::{
    Chap50::{
        OptBinSearchTreeStPer::OptBinSearchTreeStPer::*,
        OptBinSearchTreeStEph::OptBinSearchTreeStEph::*,
        Probability::Probability,
    },
    prob,
};

fn bench_obst_st_per_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBST_StPer_Small");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(20);
    
    for size in [5, 10, 15].iter() {
        let keys: Vec<char> = (0..*size).map(|i| (b'A' + i as u8) as char).collect();
        let probs: Vec<Probability> = (0..*size).map(|i| prob!(0.1 + (i as f64) * 0.05)).collect();
        let obst = OBSTStPerS::from_keys_probs(keys, probs);
        
        group.bench_with_input(BenchmarkId::new("optimal_cost", size), size, |b, _| {
            b.iter(|| {
                black_box(obst.optimal_cost())
            })
        });
    }
    group.finish();
}

fn bench_obst_st_eph_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBST_StEph_Small");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(20);
    
    for size in [5, 10, 15].iter() {
        let keys: Vec<char> = (0..*size).map(|i| (b'A' + i as u8) as char).collect();
        let probs: Vec<Probability> = (0..*size).map(|i| prob!(0.1 + (i as f64) * 0.05)).collect();
        let mut obst = OBSTStEphS::from_keys_probs(keys, probs);
        
        group.bench_with_input(BenchmarkId::new("optimal_cost", size), size, |b, _| {
            b.iter(|| {
                black_box(obst.optimal_cost())
            })
        });
    }
    group.finish();
}

fn bench_obst_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBST_Construction");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(20);
    
    for size in [10, 20, 30].iter() {
        let keys: Vec<char> = (0..*size).map(|i| (b'A' + i as u8) as char).collect();
        let probs: Vec<Probability> = (0..*size).map(|i| prob!(0.1 + (i as f64) * 0.02)).collect();
        
        group.bench_with_input(BenchmarkId::new("StPer_from_keys_probs", size), size, |b, _| {
            b.iter(|| {
                black_box(OBSTStPerS::from_keys_probs(keys.clone(), probs.clone()))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("StEph_from_keys_probs", size), size, |b, _| {
            b.iter(|| {
                black_box(OBSTStEphS::from_keys_probs(keys.clone(), probs.clone()))
            })
        });
    }
    group.finish();
}

fn bench_obst_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBST_Iteration");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(20);
    
    for size in [50, 100, 200].iter() {
        let keys: Vec<char> = (0..*size).map(|i| (b'A' + (i % 26) as u8) as char).collect();
        let probs: Vec<Probability> = (0..*size).map(|i| prob!(0.01 + (i as f64) * 0.001)).collect();
        let obst = OBSTStPerS::from_keys_probs(keys, probs);
        
        group.bench_with_input(BenchmarkId::new("into_iter", size), size, |b, _| {
            b.iter(|| {
                let sum: f64 = obst.clone().into_iter().map(|kp| kp.prob.0).sum();
                black_box(sum)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("iter_ref", size), size, |b, _| {
            b.iter(|| {
                let sum: f64 = (&obst).into_iter().map(|kp| kp.prob.0).sum();
                black_box(sum)
            })
        });
    }
    group.finish();
}

fn bench_obst_probability_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBST_Probability_Ops");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(30);
    
    let p1 = prob!(0.3);
    let p2 = prob!(0.7);
    
    group.bench_function("probability_add", |b| {
        b.iter(|| {
            black_box(p1 + p2)
        })
    });
    
    group.bench_function("probability_sub", |b| {
        b.iter(|| {
            black_box(p2 - p1)
        })
    });
    
    group.bench_function("probability_cmp", |b| {
        b.iter(|| {
            black_box(p1 < p2)
        })
    });
    
    group.finish();
}

fn bench_obst_vec_vs_traditional(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBST_Vec_Performance");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(20);
    
    for size in [8, 12, 16].iter() {
        let keys: Vec<char> = (0..*size).map(|i| (b'A' + i as u8) as char).collect();
        let probs: Vec<Probability> = (0..*size).map(|i| prob!(0.1 + (i as f64) * 0.03)).collect();
        let obst = OBSTStPerS::from_keys_probs(keys, probs);
        
        // Benchmark direct Vec indexing performance
        group.bench_with_input(BenchmarkId::new("vec_direct_access", size), size, |b, _| {
            b.iter(|| {
                let mut sum = prob!(0.0);
                for i in 0..obst.keys().len() {
                    sum = sum + obst.keys()[i].prob;
                }
                black_box(sum)
            })
        });
        
        // Benchmark iterator performance
        group.bench_with_input(BenchmarkId::new("vec_iterator", size), size, |b, _| {
            b.iter(|| {
                let sum = obst.keys().iter().fold(prob!(0.0), |acc, kp| acc + kp.prob);
                black_box(sum)
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_obst_st_per_small,
    bench_obst_st_eph_small,
    bench_obst_construction,
    bench_obst_iteration,
    bench_obst_probability_ops,
    bench_obst_vec_vs_traditional
);
criterion_main!(benches);