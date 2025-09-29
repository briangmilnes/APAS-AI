use std::time::Duration;

use apas_ai::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::{
    ArraySetEnumStEph, ArraySetEnumStEphTrait,
};
use apas_ai::Chap42Codex::ArrayTableEnumStEph::ArrayTableEnumStEph::{
    ArrayTableEnumStEph, ArrayTableEnumStEphTrait,
};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn dense_keys(universe: usize, step: usize) -> ArraySetEnumStEph {
    let mut set = ArraySetEnumStEph::empty(universe);
    for key in (0..universe).step_by(step) {
        let _ = set.insert(key);
    }
    set
}

fn build_table(universe: usize, step: usize) -> ArrayTableEnumStEph<i32> {
    let keys = dense_keys(universe, step);
    ArrayTableEnumStEph::tabulate(&keys, &|k| k as i32)
}

fn bench_array_table_enum_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArrayTableEnumSt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &universe in &[1_024usize, 4_096] {
        group.bench_with_input(BenchmarkId::new("union", universe), &universe, |b, &u| {
            b.iter_batched(
                || {
                    let a = build_table(u, 2);
                    let b_table = build_table(u, 3);
                    (a, b_table)
                },
                |(left, right)| black_box(left.union(&right, &|l, r| l + r)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("difference", universe), &universe, |b, &u| {
            b.iter_batched(
                || {
                    let a = build_table(u, 2);
                    let b_table = build_table(u, 4);
                    (a, b_table)
                },
                |(left, right)| black_box(left.difference(&right)),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_array_table_enum_st);
criterion_main!(benches);
