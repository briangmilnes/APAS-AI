use apas_ai::SetStEphChap5_1::SetStEphChap5_1::*;
use apas_ai::Types::Types::*;
use apas_ai::UnDirGraphStEphChap6_1::UnDirGraphStEphChap6_1::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_undirgraph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchUnDirGraphEphChap6_1");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    let n: N = 50_000;
    group.bench_with_input(BenchmarkId::new("build_vertices_edges", n), &n, |b, &len| {
        b.iter(|| {
            let mut v: Set<N> = Set::empty();
            for i in 0..len {
                let _ = Set::insert(&mut v, i);
            }
            let mut e: Set<Pair<N, N>> = Set::empty();
            for i in 0..len {
                let _ = Set::insert(&mut e, Pair(i.min((i + 1) % len), i.max((i + 1) % len)));
            }
            let g = <UnDirGraphStEph<N> as UnDirGraphStEphChap6_1Trait<N>>::FromSets(v, e);
            black_box(g)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_undirgraph_build);
criterion_main!(benches);
