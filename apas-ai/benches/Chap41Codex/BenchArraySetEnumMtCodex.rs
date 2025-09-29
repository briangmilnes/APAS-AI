use std::time::Duration;

use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
use apas_ai::Chap41Codex::ArraySetEnumMtEph::ArraySetEnumMtEph::{ArraySetEnumMtEph, ArraySetEnumMtEphTrait};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn bench_array_enum_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &universe in &[512usize, 2_048] {
        group.bench_with_input(BenchmarkId::new("filter", universe), &universe, |b, &u| {
            b.iter_batched(
                || ArraySetEnumMtEph::from_seq(u, &ArraySeqStEphS::from_vec((0..u).collect())),
                |set| {
                    let predicate = |index: usize| index % 5 == 0;
                    black_box(set.filter(&predicate))
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_array_enum_mt);
criterion_main!(benches);
