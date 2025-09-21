use apas_ai::Chap5::RelationStEphChap5_2::RelationStEphChap5_2::*;
use apas_ai::{SetLit, PairLit};
use apas_ai::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_relation_build_and_domain_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchRelationEphChap5_2");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    let n: N = 50_000;

    group.bench_with_input(BenchmarkId::new("build_pairs_and_domain_range", n), &n, |b, &len| {
        b.iter(|| {
            let mut pairs: Set<Pair<N, N>> = SetLit![]; // Set: empty constructor
            for i in 0..len {
                let _ = Set::insert(&mut pairs, PairLit!(i, i % 128));
            }
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
