//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chap43 OrderedSetStEph.

use apas_ai::Chap43::OrderedSetStEph::OrderedSetStEph::*;
use criterion::*;
use std::time::Duration;

fn bench_ordered_set_st_eph_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter_batched(
                || <OrderedSetStEph<i32>>::empty(),
                |mut set| {
                    for i in 0..size {
                        set.insert(black_box(i));
                    }
                    black_box(set)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_eph_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_contains");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStEph<i32>>::empty();
        for i in 0..*size {
            set.insert(i);
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

fn bench_ordered_set_st_eph_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_delete");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("delete", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set = <OrderedSetStEph<i32>>::empty();
                    for i in 0..size {
                        set.insert(i);
                    }
                    set
                },
                |mut set| {
                    for i in 0..size {
                        black_box(set.delete(&black_box(i)));
                    }
                    black_box(set)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_eph_first_last(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_first_last");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStEph<i32>>::empty();
        for i in 0..*size {
            set.insert(i);
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

fn bench_ordered_set_st_eph_previous_next(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_previous_next");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStEph<i32>>::empty();
        for i in 0..*size {
            set.insert(i * 2); // Insert even numbers
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

fn bench_ordered_set_st_eph_split_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_split_join");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("split", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set = <OrderedSetStEph<i32>>::empty();
                    for i in 0..size {
                        set.insert(i);
                    }
                    set
                },
                |mut set| {
                    let mid = size / 2;
                    black_box(set.split(&mid))
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("join", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut left = <OrderedSetStEph<i32>>::empty();
                    let mut right = <OrderedSetStEph<i32>>::empty();
                    let mid = size / 2;

                    for i in 0..mid {
                        left.insert(i);
                    }
                    for i in mid..size {
                        right.insert(i);
                    }
                    (left, right)
                },
                |(mut left, right)| {
                    left.join(right);
                    black_box(left)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_eph_get_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_get_range");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStEph<i32>>::empty();
        for i in 0..*size {
            set.insert(i);
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

fn bench_ordered_set_st_eph_rank_select(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_rank_select");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStEph<i32>>::empty();
        for i in 0..*size {
            set.insert(i);
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

fn bench_ordered_set_st_eph_split_rank(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_split_rank");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("split_rank", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set = <OrderedSetStEph<i32>>::empty();
                    for i in 0..size {
                        set.insert(i);
                    }
                    set
                },
                |mut set| {
                    let mid_rank = size / 2;
                    black_box(set.split_rank(mid_rank as usize))
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_eph_set_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_set_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("union", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set1 = <OrderedSetStEph<i32>>::empty();
                    let mut set2 = <OrderedSetStEph<i32>>::empty();

                    for i in 0..size {
                        set1.insert(i);
                        set2.insert(i + size / 2);
                    }
                    (set1, set2)
                },
                |(mut set1, set2)| {
                    set1.union(&set2);
                    black_box(set1)
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("intersection", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set1 = <OrderedSetStEph<i32>>::empty();
                    let mut set2 = <OrderedSetStEph<i32>>::empty();

                    for i in 0..size {
                        set1.insert(i);
                        set2.insert(i + size / 2);
                    }
                    (set1, set2)
                },
                |(mut set1, set2)| {
                    set1.intersection(&set2);
                    black_box(set1)
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("difference", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut set1 = <OrderedSetStEph<i32>>::empty();
                    let mut set2 = <OrderedSetStEph<i32>>::empty();

                    for i in 0..size {
                        set1.insert(i);
                        set2.insert(i + size / 2);
                    }
                    (set1, set2)
                },
                |(mut set1, set2)| {
                    set1.difference(&set2);
                    black_box(set1)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_set_st_eph_filter_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedSetStEph_filter_map");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for size in [100, 500, 1000].iter() {
        let mut set = <OrderedSetStEph<i32>>::empty();
        for i in 0..*size {
            set.insert(i);
        }

        group.bench_with_input(BenchmarkId::new("filter", size), size, |b, _size| {
            b.iter(|| black_box(set.filter(|x| x % 2 == 0)));
        });

        group.bench_with_input(BenchmarkId::new("filter", size), size, |b, _size| {
            b.iter(|| {
                let mut set_copy = set.clone();
                set_copy.filter(|x| *x % 2 == 0);
                black_box(set_copy)
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_ordered_set_st_eph_insert,
    bench_ordered_set_st_eph_contains,
    bench_ordered_set_st_eph_delete,
    bench_ordered_set_st_eph_first_last,
    bench_ordered_set_st_eph_previous_next,
    bench_ordered_set_st_eph_split_join,
    bench_ordered_set_st_eph_get_range,
    bench_ordered_set_st_eph_rank_select,
    bench_ordered_set_st_eph_split_rank,
    bench_ordered_set_st_eph_set_operations,
    bench_ordered_set_st_eph_filter_map
);

criterion_main!(benches);
