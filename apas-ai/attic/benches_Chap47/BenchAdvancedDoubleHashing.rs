//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for AdvancedDoubleHashing

use std::time::Duration;

use criterion::*;

use apas_ai::Chap47::AdvancedDoubleHashing::AdvancedDoubleHashing::AdvancedDoubleHashingStrategy;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::FlatHashTable;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::DefaultHashFunction;

fn bench_double_hashing_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("double_hashing_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let hash1 = DefaultHashFunction;
    let hash2 = DefaultHashFunction;
    let strategy: AdvancedDoubleHashingStrategy<i32, _, _> = AdvancedDoubleHashingStrategy::new(hash1, hash2);

    for table_size in [7, 11].iter() {
        group.bench_with_input(BenchmarkId::new("table_size", table_size), table_size, |b, &size| {
            b.iter(|| {
                let mut table = FlatHashTable::create_table(strategy.clone(), size);
                let num_elements = (size as f64 * 0.3) as usize;
                for i in 0..num_elements {
                    table = table.insert(black_box(i as i32), black_box(format!("value_{i}")));
                }
                black_box(table)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_double_hashing_insert);
criterion_main!(benches);
