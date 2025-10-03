//! Copyright © 2025 APAS-VERUS. All rights reserved.
//!
//! Benchmarks for StackStEph

use apas_ai::Chap57::StackStEph::StackStEph::StackStEph;
use criterion::*;
use std::time::Duration;

fn bench_push(c: &mut Criterion) {
    let sizes = [100, 500, 1000];

    for n in sizes {
        c.bench_function(&format!("stack_push_n{}", n), |b| {
            b.iter(|| {
                let mut stack = StackStEph::new();
                for i in 0..n {
                    stack.push(black_box(i));
                }
                black_box(stack)
            })
        });
    }
}

fn bench_pop(c: &mut Criterion) {
    let sizes = [100, 500, 1000];

    for n in sizes {
        c.bench_function(&format!("stack_pop_n{}", n), |b| {
            b.iter_batched(
                || {
                    let mut stack = StackStEph::new();
                    for i in 0..n {
                        stack.push(i);
                    }
                    stack
                },
                |mut stack| {
                    for _ in 0..n {
                        black_box(stack.pop());
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn bench_push_pop_mixed(c: &mut Criterion) {
    let sizes = [100, 500, 1000];

    for n in sizes {
        c.bench_function(&format!("stack_push_pop_mixed_n{}", n), |b| {
            b.iter(|| {
                let mut stack = StackStEph::new();
                for i in 0..n {
                    stack.push(black_box(i));
                    if i % 3 == 0 {
                        black_box(stack.pop());
                    }
                }
                black_box(stack)
            })
        });
    }
}

fn bench_peek(c: &mut Criterion) {
    let sizes = [100, 500, 1000];

    for n in sizes {
        c.bench_function(&format!("stack_peek_n{}", n), |b| {
            let mut stack = StackStEph::new();
            for i in 0..n {
                stack.push(i);
            }
            b.iter(|| black_box(stack.peek()))
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_secs(1))
        .sample_size(30);
    targets = bench_push, bench_pop, bench_push_pop_mixed, bench_peek
}
criterion_main!(benches);
