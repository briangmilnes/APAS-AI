//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for UnsortedListPQ implementation

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

use apas_ai::Chap45::UnsortedListPQ::UnsortedListPQ::*;

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnsortedListPQ Insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    
    for size in [10, 50, 100].iter() {
        let data: Vec<i32> = (0..*size).rev().collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut pq = UnsortedListPQ::empty();
                for &item in &data {
                    pq = pq.insert(black_box(item));
                }
                black_box(pq)
            })
        });
    }
    
    group.finish();
}

fn bench_delete_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnsortedListPQ DeleteMin");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    
    for size in [10, 50, 100].iter() {
        let data: Vec<i32> = (0..*size).collect();
        let pq = UnsortedListPQ::from_vec(data.clone());
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut pq_copy = pq.clone();
                for _ in 0..*size {
                    let (new_pq, _) = pq_copy.delete_min();
                    pq_copy = new_pq;
                }
                black_box(pq_copy)
            })
        });
    }
    
    group.finish();
}

fn bench_meld(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnsortedListPQ Meld");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    
    for size in [10, 50, 100].iter() {
        let data1: Vec<i32> = (0..*size).collect();
        let data2: Vec<i32> = (*size..(2 * size)).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            let pq1 = UnsortedListPQ::from_vec(data1.clone());
            let pq2 = UnsortedListPQ::from_vec(data2.clone());
            b.iter(|| {
                black_box(pq1.meld(&pq2))
            })
        });
    }
    
    group.finish();
}

fn bench_find_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnsortedListPQ FindMin");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    
    for size in [100, 500].iter() {
        let data: Vec<i32> = (0..*size).collect();
        let pq = UnsortedListPQ::from_vec(data.clone());
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                black_box(pq.find_min())
            })
        });
    }
    
    group.finish();
}

fn bench_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnsortedListPQ Construction");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    
    for size in [50, 100].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                black_box(UnsortedListPQ::from_vec(data.clone()))
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_insert,
    bench_delete_min,
    bench_meld,
    bench_find_min,
    bench_construction
);

criterion_main!(benches);


