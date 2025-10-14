//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for AdvancedLinearProbing

use apas_ai::Chap47::AdvancedLinearProbing::AdvancedLinearProbing::AdvancedLinearProbingStrategy;
use apas_ai::Chap47::FlatHashTable::FlatHashTable::FlatHashTable;
use apas_ai::Chap47::HashFunctionTraits::HashFunctionTraits::DefaultHashFunction;
use criterion::*;
use std::time::Duration;

fn bench_linear_probing_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("linear_probing_insert");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    let hash_fn = DefaultHashFunction;
    let strategy: AdvancedLinearProbingStrategy<i32, _> = AdvancedLinearProbingStrategy::new(hash_fn);

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

criterion_group!(benches, bench_linear_probing_insert);
criterion_main!(benches);
