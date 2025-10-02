//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chap43 OrderedSetStPer.

use apas_ai::Chap43::OrderedSetStPer::OrderedSetStPer::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_ordered_set_st_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter(|| {
                let mut set = <OrderedSetStPer<i32>>::empty();
                for i in 0..size {
                    set = black_box(set.insert(black_box(i)));
                }
                black_box(set)
            });
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_contains");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStPer<i32>>::empty();
        for i in 0..*size {
            set = set.insert(i);
        }

        group.bench_with_input(BenchmarkId::new("contains", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(set.find(&black_box(i)));
                }
            });
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_delete");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("delete", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set = <OrderedSetStPer<i32>>::empty();
                    for i in 0..size {
                        set = set.insert(i);
                    }
                    set
                },
                |mut set| {
                    for i in 0..size {
                        set = black_box(set.delete(&black_box(i)));
                    }
                    black_box(set)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_first_last(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_first_last");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStPer<i32>>::empty();
        for i in 0..*size {
            set = set.insert(i);
        }

        group.bench_with_input(BenchmarkId::new("first", size), size, |b, _size| {
            b.iter(|| black_box(set.first()));
        });

        group.bench_with_input(BenchmarkId::new("last", size), size, |b, _size| {
            b.iter(|| black_box(set.last()));
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_previous_next(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_previous_next");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStPer<i32>>::empty();
        for i in 0..*size {
            set = set.insert(i * 2); // Insert even numbers
        }

        group.bench_with_input(BenchmarkId::new("previous", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(set.previous(&black_box(i * 2 + 1)));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("next", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(set.next(&black_box(i * 2 + 1)));
                }
            });
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_split_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_split_join");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("split", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set = <OrderedSetStPer<i32>>::empty();
                    for i in 0..size {
                        set = set.insert(i);
                    }
                    set
                },
                |set| {
                    let mid = size / 2;
                    black_box(set.split(&mid))
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("join", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut left = <OrderedSetStPer<i32>>::empty();
                    let mut right = <OrderedSetStPer<i32>>::empty();
                    let mid = size / 2;

                    for i in 0..mid {
                        left = left.insert(i);
                    }
                    for i in mid..size {
                        right = right.insert(i);
                    }
                    (left, right)
                },
                |(left, right)| black_box(OrderedSetStPerTrait::join(&left, &right)),
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_get_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_get_range");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStPer<i32>>::empty();
        for i in 0..*size {
            set = set.insert(i);
        }

        group.bench_with_input(BenchmarkId::new("get_range", size), size, |b, &size| {
            b.iter(|| {
                let start = size / 4;
                let end = 3 * size / 4;
                black_box(set.get_range(&start, &end))
            });
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_rank_select(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_rank_select");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStPer<i32>>::empty();
        for i in 0..*size {
            set = set.insert(i);
        }

        group.bench_with_input(BenchmarkId::new("rank", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(set.rank(&black_box(i)));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("select", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(set.select(black_box(i as usize)));
                }
            });
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_split_rank(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_split_rank");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("split_rank", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set = <OrderedSetStPer<i32>>::empty();
                    for i in 0..size {
                        set = set.insert(i);
                    }
                    set
                },
                |set| {
                    let mid_rank = size / 2;
                    black_box(set.split_rank(mid_rank as usize))
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_per_set_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStPer_set_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("union", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set1 = <OrderedSetStPer<i32>>::empty();
                    let mut set2 = <OrderedSetStPer<i32>>::empty();

                    for i in 0..size {
                        set1 = set1.insert(i);
                        set2 = set2.insert(i + size / 2);
                    }
                    (set1, set2)
                },
                |(set1, set2)| black_box(set1.union(&set2)),
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("intersection", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set1 = <OrderedSetStPer<i32>>::empty();
                    let mut set2 = <OrderedSetStPer<i32>>::empty();

                    for i in 0..size {
                        set1 = set1.insert(i);
                        set2 = set2.insert(i + size / 2);
                    }
                    (set1, set2)
                },
                |(set1, set2)| black_box(set1.intersection(&set2)),
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("difference", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set1 = <OrderedSetStPer<i32>>::empty();
                    let mut set2 = <OrderedSetStPer<i32>>::empty();

                    for i in 0..size {
                        set1 = set1.insert(i);
                        set2 = set2.insert(i + size / 2);
                    }
                    (set1, set2)
                },
                |(set1, set2)| black_box(set1.difference(&set2)),
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_ordered_set_st_per_insert,
    bench_ordered_set_st_per_contains,
    bench_ordered_set_st_per_delete,
    bench_ordered_set_st_per_first_last,
    bench_ordered_set_st_per_previous_next,
    bench_ordered_set_st_per_split_join,
    bench_ordered_set_st_per_get_range,
    bench_ordered_set_st_per_rank_select,
    bench_ordered_set_st_per_split_rank,
    bench_ordered_set_st_per_set_operations
);

criterion_main!(benches);
