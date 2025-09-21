use std::time::Duration;

use apas_ai::Chap11::FibonacciMt::FibonacciMt::{FibonacciMt, FibonacciMtTrait};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_fibonacci_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibonacciMt");
    group.sample_size(30);
    group.warm_up_time(Duration::from_millis(800));
    group.measurement_time(Duration::from_secs(6));

    for &n in &[20usize, 24, 28] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &input| {
            b.iter(|| black_box(FibonacciMt::fib(input)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_fibonacci_mt);
criterion_main!(benches);
