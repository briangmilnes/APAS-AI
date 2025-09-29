use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use apas_ai::Chap43Claude::OrderedTableStEph::*;
use apas_ai::Types::Types::*;
use std::time::Duration;

fn bench_ordered_table_st_eph_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter_batched(
                || OrderedTableStEph::empty(),
                |mut table| {
                    for i in 0..size {
                        table.insert(black_box(i), black_box(format!("value_{}", i)), |_old, new| new.clone());
                    }
                    black_box(table)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_lookup");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let mut table = OrderedTableStEph::empty();
        for i in 0..*size {
            table.insert(i, format!("value_{}", i), |_old, new| new.clone());
        }
        
        group.bench_with_input(BenchmarkId::new("lookup", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(table.lookup(&black_box(i)));
                }
            });
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_delete");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("delete", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut table = OrderedTableStEph::empty();
                    for i in 0..size {
                        table.insert(i, format!("value_{}", i), |_old, new| new.clone());
                    }
                    table
                },
                |mut table| {
                    for i in 0..size {
                        black_box(table.delete(&black_box(i)));
                    }
                    black_box(table)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_first_last_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_first_last_key");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let mut table = OrderedTableStEph::empty();
        for i in 0..*size {
            table.insert(i, format!("value_{}", i), |_old, new| new.clone());
        }
        
        group.bench_with_input(BenchmarkId::new("first_key", size), size, |b, _size| {
            b.iter(|| {
                black_box(table.first_key())
            });
        });
        
        group.bench_with_input(BenchmarkId::new("last_key", size), size, |b, _size| {
            b.iter(|| {
                black_box(table.last_key())
            });
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_previous_next_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_previous_next_key");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let mut table = OrderedTableStEph::empty();
        for i in 0..*size {
            table.insert(i * 2, format!("value_{}", i * 2), |_old, new| new.clone()); // Insert even numbers
        }
        
        group.bench_with_input(BenchmarkId::new("previous_key", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(table.previous_key(&black_box(i * 2 + 1)));
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("next_key", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(table.next_key(&black_box(i * 2 + 1)));
                }
            });
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_split_join_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_split_join_key");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("split_key", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut table = OrderedTableStEph::empty();
                    for i in 0..size {
                        table.insert(i, format!("value_{}", i), |_old, new| new.clone());
                    }
                    table
                },
                |mut table| {
                    let mid = size / 2;
                    black_box(table.split_key(&mid))
                },
                criterion::BatchSize::SmallInput,
            );
        });
        
        group.bench_with_input(BenchmarkId::new("join_key", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut left = OrderedTableStEph::empty();
                    let mut right = OrderedTableStEph::empty();
                    let mid = size / 2;
                    
                    for i in 0..mid {
                        left.insert(i, format!("value_{}", i), |_old, new| new.clone());
                    }
                    for i in mid..size {
                        right.insert(i, format!("value_{}", i), |_old, new| new.clone());
                    }
                    (left, right)
                },
                |(mut left, mut right)| {
                    left.join_key(&mut right);
                    black_box(left)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_get_key_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_get_key_range");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let mut table = OrderedTableStEph::empty();
        for i in 0..*size {
            table.insert(i, format!("value_{}", i), |_old, new| new.clone());
        }
        
        group.bench_with_input(BenchmarkId::new("get_key_range", size), size, |b, &size| {
            b.iter(|| {
                let start = size / 4;
                let end = 3 * size / 4;
                black_box(table.get_key_range(&start, &end))
            });
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_rank_select_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_rank_select_key");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let mut table = OrderedTableStEph::empty();
        for i in 0..*size {
            table.insert(i, format!("value_{}", i), |_old, new| new.clone());
        }
        
        group.bench_with_input(BenchmarkId::new("rank_key", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(table.rank_key(&black_box(i)));
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("select_key", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    black_box(table.select_key(black_box(i)));
                }
            });
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_split_rank_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_split_rank_key");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("split_rank_key", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut table = OrderedTableStEph::empty();
                    for i in 0..size {
                        table.insert(i, format!("value_{}", i), |_old, new| new.clone());
                    }
                    table
                },
                |mut table| {
                    let mid_rank = size / 2;
                    black_box(table.split_rank_key(mid_rank))
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_table_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_table_operations");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let mut table = OrderedTableStEph::empty();
        for i in 0..*size {
            table.insert(i, i * 10, |_old, new| new.clone());
        }
        
        group.bench_with_input(BenchmarkId::new("filter", size), size, |b, _size| {
            b.iter(|| {
                black_box(table.filter(|k, _v| k % 2 == 0))
            });
        });
        
        group.bench_with_input(BenchmarkId::new("map", size), size, |b, _size| {
            b.iter(|| {
                black_box(table.map(|k, v| format!("{}:{}", k, v)))
            });
        });
        
        group.bench_with_input(BenchmarkId::new("reduce", size), size, |b, _size| {
            b.iter(|| {
                black_box(table.reduce(0, |acc, _k, v| acc + v))
            });
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_collect(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_collect");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let mut table = OrderedTableStEph::empty();
        for i in 0..*size {
            table.insert(i, format!("value_{}", i), |_old, new| new.clone());
        }
        
        group.bench_with_input(BenchmarkId::new("collect", size), size, |b, _size| {
            b.iter(|| {
                black_box(table.collect())
            });
        });
    }
    group.finish();
}

fn bench_ordered_table_st_eph_from_sorted_entries(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderedTableStEph_from_sorted_entries");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_millis(1000));
    
    for size in [100, 500, 1000].iter() {
        let entries: Vec<Pair<i32, String>> = (0..*size)
            .map(|i| Pair(i, format!("value_{}", i)))
            .collect();
        
        group.bench_with_input(BenchmarkId::new("from_sorted_entries", size), &entries, |b, entries| {
            b.iter(|| {
                black_box(OrderedTableStEph::from_sorted_entries(entries.clone()))
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_ordered_table_st_eph_insert,
    bench_ordered_table_st_eph_lookup,
    bench_ordered_table_st_eph_delete,
    bench_ordered_table_st_eph_first_last_key,
    bench_ordered_table_st_eph_previous_next_key,
    bench_ordered_table_st_eph_split_join_key,
    bench_ordered_table_st_eph_get_key_range,
    bench_ordered_table_st_eph_rank_select_key,
    bench_ordered_table_st_eph_split_rank_key,
    bench_ordered_table_st_eph_table_operations,
    bench_ordered_table_st_eph_collect,
    bench_ordered_table_st_eph_from_sorted_entries
);

criterion_main!(benches);
