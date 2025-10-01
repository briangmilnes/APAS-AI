//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for FlatHashTable

use apas_ai::Chap47::FlatHashTable::FlatHashTable::*;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::*;
use apas_ai::Chap47::LinearProbing::LinearProbing::*;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_flat_hash_table_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat_hash_table_operations");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);

    let probe_strategy = LinearProbingStrategy::new(DefaultHashFunction);
    let mut table = FlatHashTable::create_table(probe_strategy, 64);

    for i in 0..30 {
        let key = i.to_string();
        let value = (i * 10).to_string();
        table = table.insert(key, value);
    }

    group.bench_function("lookup_existing", |b| {
        b.iter(|| {
            for i in 0..15 {
                let key = i.to_string();
                black_box(table.lookup(&black_box(key)));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_flat_hash_table_operations);
criterion_main!(benches);
