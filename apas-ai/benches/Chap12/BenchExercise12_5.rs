//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::hint::spin_loop;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use criterion::*;

use apas_ai::Chap12::Exercise12_5::Exercise12_5::*;
use apas_ai::Types::Types::*;

fn bench_exercise12_5(c: &mut Criterion) {
    let mut group = c.benchmark_group("Exercise12_5");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 31;
    group.bench_with_input(BenchmarkId::new("concurrent_stack", n), &n, |b, &len| {
        b.iter(|| {
            let stack = Arc::new(ConcurrentStackMt::new());
            let mut handles = vec![];

            // Producer threads
            for i in 0..2 {
                let stack_clone = Arc::clone(&stack);
                let handle = thread::spawn(move || {
                    for j in 0..len {
                        stack_clone.push(i * len + j);
                    }
                });
                handles.push(handle);
            }

            // Consumer threads
            for _ in 0..2 {
                let stack_clone = Arc::clone(&stack);
                let handle = thread::spawn(move || {
                    for _ in 0..len {
                        while stack_clone.pop().is_none() {
                            spin_loop();
                        }
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            black_box(stack)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_exercise12_5);
criterion_main!(benches);
