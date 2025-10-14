//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use criterion::*;

use apas_ai::Types::Types::*;
use apas_ai::Chap12::Exercise12_1::Exercise12_1::*;

fn bench_exercise12_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Exercise12_1");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 25;
    group.bench_with_input(BenchmarkId::new("spinlock_contention", n), &n, |b, &len| {
        b.iter(|| {
            let lock = Arc::new(SpinLock::new());
            let mut handles = vec![];

            for _ in 0..4 {
                let lock_clone = Arc::clone(&lock);
                let handle = thread::spawn(move || {
                    for _ in 0..len {
                        lock_clone.lock();
                        // Critical section work
                        black_box(42);
                        lock_clone.unlock();
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
            black_box(lock)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_exercise12_1);
criterion_main!(benches);
