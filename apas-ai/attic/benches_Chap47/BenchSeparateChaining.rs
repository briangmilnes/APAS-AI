//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for SeparateChaining

use std::time::Duration;

use criterion::*;

use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Chap47::SeparateChaining::SeparateChaining::*;

fn bench_separate_chaining_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("separate_chaining_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    for size in [10, 25, 50].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter(|| {
                let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, DefaultHashFunction, 16);
                for i in 0..size {
                    let key = i.to_string();
                    let value = (i * 10).to_string();
                    table = table.insert(black_box(key), black_box(value));
                }
                black_box(table)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_separate_chaining_insert);
criterion_main!(benches);
