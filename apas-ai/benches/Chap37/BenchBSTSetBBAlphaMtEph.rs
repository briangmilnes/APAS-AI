//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for BSTSetBBAlphaMtEph

use std::time::Duration;

use apas_ai::BSTSetBBAlphaMtEphLit;
use apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt as BBAlphaSet;
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

trait BenchSet: Sized {
    fn empty() -> Self;
    fn insert_value(&mut self, value: i32);
    fn union_with(&self, other: &Self) -> Self;
    fn difference_with(&self, other: &Self) -> Self;
    fn filter_divisible_by(&self, divisor: i32) -> Self;
    fn reduce_sum(&self) -> i32;
}

fn build_pair<S: BenchSet>(len: usize) -> (S, S) {
    let mut a = S::empty();
    let mut b = S::empty();
    for value in 0..len {
        let v = value as i32;
        a.insert_value(v);
        if value % 2 == 0 {
            b.insert_value(v);
        }
    }
    (a, b)
}

fn build_single<S: BenchSet>(len: usize) -> S {
    let mut set = S::empty();
    for value in 0..len {
        set.insert_value(value as i32);
    }
    set
}

fn bench_set_variants<S: BenchSet>(c: &mut Criterion, label: &str) {
    let mut group = c.benchmark_group(label);
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &len in &[128usize, 256] {
        group.bench_with_input(BenchmarkId::new("union", len), &len, |b, &input| {
            b.iter_batched(
                || build_pair::<S>(input),
                |(set_a, set_b)| black_box(set_a.union_with(&set_b)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("difference", len), &len, |b, &input| {
            b.iter_batched(
                || build_pair::<S>(input),
                |(set_a, set_b)| black_box(set_a.difference_with(&set_b)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("filter", len), &len, |b, &input| {
            b.iter_batched(
                || build_single::<S>(input),
                |set| black_box(set.filter_divisible_by(3)),
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("reduce", len), &len, |b, &input| {
            b.iter_batched(
                || build_single::<S>(input),
                |set| black_box(set.reduce_sum()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

impl BenchSet for BBAlphaSet<i32> {
    fn empty() -> Self { BSTSetBBAlphaMtEphLit![] }

    fn insert_value(&mut self, value: i32) { self.insert(value); }

    fn union_with(&self, other: &Self) -> Self { self.union(other) }

    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }

    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor == 0) }

    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }
}

fn bench_bbalpha_set(c: &mut Criterion) { bench_set_variants::<BBAlphaSet<i32>>(c, "BSTSetBBAlphaMtEph"); }

criterion_group!(benches, bench_bbalpha_set);
criterion_main!(benches);
