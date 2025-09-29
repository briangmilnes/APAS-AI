//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 45: Priority Queue implementations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use apas_ai::Chap45::UnsortedListPQ::UnsortedListPQ::*;
use apas_ai::Chap45::SortedListPQ::SortedListPQ::*;
use apas_ai::Chap45::BalancedTreePQ::BalancedTreePQ::*;
use apas_ai::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
use apas_ai::Chap45::LeftistHeapPQ::LeftistHeapPQ::*;
use apas_ai::Chap45::HeapsortExample::HeapsortExample::*;

fn bench_insert_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Priority Queue Insert");
    
    for size in [10, 50, 100, 500].iter() {
        let data: Vec<i32> = (0..*size).rev().collect(); // Reverse order for worst case
        
        group.bench_with_input(BenchmarkId::new("UnsortedList", size), size, |b, _| {
            b.iter(|| {
                let mut pq = UnsortedListPQ::empty();
                for &item in &data {
                    pq = pq.insert(black_box(item));
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("SortedList", size), size, |b, _| {
            b.iter(|| {
                let mut pq = SortedListPQ::empty();
                for &item in &data {
                    pq = pq.insert(black_box(item));
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BalancedTree", size), size, |b, _| {
            b.iter(|| {
                let mut pq = BalancedTreePQ::empty();
                for &item in &data {
                    pq = pq.insert(black_box(item));
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), size, |b, _| {
            b.iter(|| {
                let mut pq = BinaryHeapPQ::empty();
                for &item in &data {
                    pq = pq.insert(black_box(item));
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("LeftistHeap", size), size, |b, _| {
            b.iter(|| {
                let mut pq = LeftistHeapPQ::empty();
                for &item in &data {
                    pq = pq.insert(black_box(item));
                }
                black_box(pq)
            })
        });
    }
    
    group.finish();
}

fn bench_delete_min_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Priority Queue DeleteMin");
    
    for size in [10, 50, 100, 500].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        // Pre-build priority queues for fair comparison
        let unsorted_pq = UnsortedListPQ::from_vec(data.clone());
        let sorted_pq = SortedListPQ::from_vec(data.clone());
        let balanced_pq = BalancedTreePQ::from_vec(data.clone());
        let binary_pq = BinaryHeapPQ::from_vec(data.clone());
        let leftist_pq = LeftistHeapPQ::from_vec(data.clone());
        
        group.bench_with_input(BenchmarkId::new("UnsortedList", size), size, |b, _| {
            b.iter(|| {
                let mut pq = unsorted_pq.clone();
                while !pq.is_empty() {
                    let (new_pq, _) = pq.delete_min();
                    pq = new_pq;
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("SortedList", size), size, |b, _| {
            b.iter(|| {
                let mut pq = sorted_pq.clone();
                while !pq.is_empty() {
                    let (new_pq, _) = pq.delete_min();
                    pq = new_pq;
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BalancedTree", size), size, |b, _| {
            b.iter(|| {
                let mut pq = balanced_pq.clone();
                while !pq.is_empty() {
                    let (new_pq, _) = pq.delete_min();
                    pq = new_pq;
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), size, |b, _| {
            b.iter(|| {
                let mut pq = binary_pq.clone();
                while !pq.is_empty() {
                    let (new_pq, _) = pq.delete_min();
                    pq = new_pq;
                }
                black_box(pq)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("LeftistHeap", size), size, |b, _| {
            b.iter(|| {
                let mut pq = leftist_pq.clone();
                while !pq.is_empty() {
                    let (new_pq, _) = pq.delete_min();
                    pq = new_pq;
                }
                black_box(pq)
            })
        });
    }
    
    group.finish();
}

fn bench_meld_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Priority Queue Meld");
    
    for size in [10, 50, 100].iter() {
        let data1: Vec<i32> = (0..*size).collect();
        let data2: Vec<i32> = (*size..(2 * size)).collect();
        
        group.bench_with_input(BenchmarkId::new("UnsortedList", size), size, |b, _| {
            let pq1 = UnsortedListPQ::from_vec(data1.clone());
            let pq2 = UnsortedListPQ::from_vec(data2.clone());
            b.iter(|| {
                black_box(pq1.meld(&pq2))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("SortedList", size), size, |b, _| {
            let pq1 = SortedListPQ::from_vec(data1.clone());
            let pq2 = SortedListPQ::from_vec(data2.clone());
            b.iter(|| {
                black_box(pq1.meld(&pq2))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BalancedTree", size), size, |b, _| {
            let pq1 = BalancedTreePQ::from_vec(data1.clone());
            let pq2 = BalancedTreePQ::from_vec(data2.clone());
            b.iter(|| {
                black_box(pq1.meld(&pq2))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), size, |b, _| {
            let pq1 = BinaryHeapPQ::from_vec(data1.clone());
            let pq2 = BinaryHeapPQ::from_vec(data2.clone());
            b.iter(|| {
                black_box(pq1.meld(&pq2))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("LeftistHeap", size), size, |b, _| {
            let pq1 = LeftistHeapPQ::from_vec(data1.clone());
            let pq2 = LeftistHeapPQ::from_vec(data2.clone());
            b.iter(|| {
                black_box(pq1.meld(&pq2))
            })
        });
    }
    
    group.finish();
}

fn bench_heapsort_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("Heapsort Algorithms");
    
    for size in [50, 100, 200].iter() {
        let data: Vec<i32> = (0..*size).rev().collect(); // Reverse sorted for worst case
        
        group.bench_with_input(BenchmarkId::new("UnsortedList", size), size, |b, _| {
            b.iter(|| {
                black_box(Heapsort::heapsort_unsorted_list(&data))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("SortedList", size), size, |b, _| {
            b.iter(|| {
                black_box(Heapsort::heapsort_sorted_list(&data))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BalancedTree", size), size, |b, _| {
            b.iter(|| {
                black_box(Heapsort::heapsort_balanced_tree(&data))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), size, |b, _| {
            b.iter(|| {
                black_box(Heapsort::heapsort_binary_heap(&data))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("LeftistHeap", size), size, |b, _| {
            b.iter(|| {
                black_box(Heapsort::heapsort_leftist_heap(&data))
            })
        });
    }
    
    group.finish();
}

fn bench_find_min_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Priority Queue FindMin");
    
    for size in [100, 500, 1000].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        let unsorted_pq = UnsortedListPQ::from_vec(data.clone());
        let sorted_pq = SortedListPQ::from_vec(data.clone());
        let balanced_pq = BalancedTreePQ::from_vec(data.clone());
        let binary_pq = BinaryHeapPQ::from_vec(data.clone());
        let leftist_pq = LeftistHeapPQ::from_vec(data.clone());
        
        group.bench_with_input(BenchmarkId::new("UnsortedList", size), size, |b, _| {
            b.iter(|| {
                black_box(unsorted_pq.find_min())
            })
        });
        
        group.bench_with_input(BenchmarkId::new("SortedList", size), size, |b, _| {
            b.iter(|| {
                black_box(sorted_pq.find_min())
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BalancedTree", size), size, |b, _| {
            b.iter(|| {
                black_box(balanced_pq.find_min())
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), size, |b, _| {
            b.iter(|| {
                black_box(binary_pq.find_min())
            })
        });
        
        group.bench_with_input(BenchmarkId::new("LeftistHeap", size), size, |b, _| {
            b.iter(|| {
                black_box(leftist_pq.find_min())
            })
        });
    }
    
    group.finish();
}

fn bench_construction_from_sequence(c: &mut Criterion) {
    let mut group = c.benchmark_group("Priority Queue Construction");
    
    for size in [50, 100, 200].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        group.bench_with_input(BenchmarkId::new("UnsortedList", size), size, |b, _| {
            b.iter(|| {
                black_box(UnsortedListPQ::from_vec(data.clone()))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("SortedList", size), size, |b, _| {
            b.iter(|| {
                black_box(SortedListPQ::from_vec(data.clone()))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BalancedTree", size), size, |b, _| {
            b.iter(|| {
                black_box(BalancedTreePQ::from_vec(data.clone()))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), size, |b, _| {
            b.iter(|| {
                black_box(BinaryHeapPQ::from_vec(data.clone()))
            })
        });
        
        group.bench_with_input(BenchmarkId::new("LeftistHeap", size), size, |b, _| {
            b.iter(|| {
                black_box(LeftistHeapPQ::from_vec(data.clone()))
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_insert_operations,
    bench_delete_min_operations,
    bench_meld_operations,
    bench_heapsort_algorithms,
    bench_find_min_operations,
    bench_construction_from_sequence
);

criterion_main!(benches);
