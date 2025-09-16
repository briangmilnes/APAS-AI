use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::SetStEphChap5_1::SetStEphChap5_1::*;
use apas_ai::RelationStEphChap5_2::RelationStEphChap5_2::*;
use apas_ai::MappingStEphChap5_5::MappingStEphChap5_5::*;
use std::time::Duration;

fn bench_mapping_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchMappingEphChap5_5");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    let n: N = 50_000;
    group.bench_with_input(BenchmarkId::new("from_relation_overwrite_dups", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<Pair<N, N>> = Set::empty();
            for i in 0..len { let _ = Set::insert(&mut pairs, Pair(i % 10_000, i)); }
            let r = Relation::FromSet(pairs);
            let m = Mapping::FromRelation(&r);
            black_box((r, m))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_mapping_build);
criterion_main!(benches);


