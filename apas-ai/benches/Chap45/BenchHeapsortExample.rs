//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Heapsort implementations using different priority queues

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

use apas_ai::Chap45::HeapsortExample::HeapsortExample::*;

fn bench_heapsort(c: &mut Criterion) {
    let mut group = c.benchmark_group("Heapsort");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [50, 100].iter() {
        let data: Vec<i32> = (0..*size).rev().collect();

        group.bench_with_input(BenchmarkId::new("UnsortedList", size), size, |b, _| {
            b.iter(|| black_box(heapsort_unsorted_list(&data)))
        });

        group.bench_with_input(BenchmarkId::new("SortedList", size), size, |b, _| {
            b.iter(|| black_box(heapsort_sorted_list(&data)))
        });

        group.bench_with_input(BenchmarkId::new("BalancedTree", size), size, |b, _| {
            b.iter(|| black_box(heapsort_balanced_tree(&data)))
        });

        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), size, |b, _| {
            b.iter(|| black_box(heapsort_binary_heap(&data)))
        });

        group.bench_with_input(BenchmarkId::new("LeftistHeap", size), size, |b, _| {
            b.iter(|| black_box(heapsort_leftist_heap(&data)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_heapsort);

criterion_main!(benches);
