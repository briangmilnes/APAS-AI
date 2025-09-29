use std::time::Duration;

use apas_ai::Chap41Codex::AVLTreeSetStEph::AVLTreeSetStEph::{AVLTreeSetStEph, AVLTreeSetStEphTrait};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn build_st(len: usize) -> AVLTreeSetStEph<i32> {
    let mut set = AVLTreeSetStEph::empty();
    for value in 0..(len as i32) {
        set.insert(value);
    }
    set
}

fn bench_avl_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetSt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[256usize, 512] {
        group.bench_with_input(BenchmarkId::new("union", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let a = build_st(len);
                    let b = build_st(len / 2);
                    (a, b)
                },
                |(a, b)| black_box(a.union(&b)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("intersection", n), &n, |b, &len| {
            b.iter_batched(
                || {
                    let mut a = AVLTreeSetStEph::empty();
                    let mut b = AVLTreeSetStEph::empty();
                    for value in 0..(len as i32) {
                        if value % 2 == 0 {
                            a.insert(value);
                        }
                        if value % 3 == 0 {
                            b.insert(value);
                        }
                    }
                    (a, b)
                },
                |(a, b)| black_box(a.intersection(&b)),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_avl_st);
criterion_main!(benches);
