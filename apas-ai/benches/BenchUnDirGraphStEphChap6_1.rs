use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::SetEphChap5_1::SetEphChap5_1::*;
use apas_ai::UnDirGraphEphChap6_1::UnDirGraphEphChap6_1::*;
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
            for i in 0..len { let _ = Set::insert(&mut v, i); }
            let mut e: Set<(N,N)> = Set::empty();
            for i in 0..len { let _ = Set::insert(&mut e, (i.min((i+1)%len), i.max((i+1)%len))); }
            let g = UnDirGraphEph::FromSets(v, e);
            black_box(g)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_undirgraph_build);
criterion_main!(benches);


