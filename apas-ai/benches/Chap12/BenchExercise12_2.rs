//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap12::Exercise12_2::Exercise12_2::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn bench_exercise12_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Exercise12_2");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("fetch_add_cas_contention", n), &n, |b, &len| {
        b.iter(|| {
            let counter = Arc::new(AtomicUsize::new(0));
            let mut handles = vec![];

            for _ in 0..4 {
                let counter_clone = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    for _ in 0..len {
                        counter_clone.fetch_add_cas(1);
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            let final_value = counter.load(std::sync::atomic::Ordering::Relaxed);
            black_box(final_value)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_exercise12_2);
criterion_main!(benches);
