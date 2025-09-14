use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::SetEphChap5_1::SetEphChap5_1::*;
use apas_ai::RelationEphChap5_2::RelationEphChap5_2::*;
use apas_ai::SetLit;
use std::time::Duration;

fn bench_relation_build_and_domain_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchRelationEphChap5_2");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    let n: N = 50_000;

    group.bench_with_input(BenchmarkId::new("build_pairs_and_domain_range", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<(N, N)> = Set::empty();
            for i in 0..len { let _ = Set::insert(&mut pairs, (i, i % 128)); }
            let r = Relation::FromSet(pairs);
            let d = r.domain();
            let g = r.range();
            black_box((r, d, g))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_relation_build_and_domain_range);
criterion_main!(benches);


