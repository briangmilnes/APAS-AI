//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for AdvancedQuadraticProbing

use apas_ai::Chap47::AdvancedQuadraticProbing::AdvancedQuadraticProbing::AdvancedQuadraticProbingStrategy;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::FlatHashTable;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::DefaultHashFunction;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_quadratic_probing_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("quadratic_probing_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(30);

    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedQuadraticProbingStrategy<i32, _> = AdvancedQuadraticProbingStrategy::new(hash_fn);

    for table_size in [17, 31].iter() {
        group.bench_with_input(BenchmarkId::new("table_size", table_size), table_size, |b, &size| {
            b.iter(|| {
                let mut table = FlatHashTable::create_table(strategy.clone(), size);
                let num_elements = (size as f64 * 0.5) as usize;
                for i in 0..num_elements {
                    table = table.insert(black_box(i as i32), black_box(format!("value_{}", i)));
                }
                black_box(table)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_quadratic_probing_insert);
criterion_main!(benches);
