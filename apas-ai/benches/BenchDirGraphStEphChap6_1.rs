use apas_ai::DirGraphStEphChap6_1::DirGraphStEphChap6_1::*;
use apas_ai::SetStEphChap5_1::SetStEphChap5_1::*;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_dirgraph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchDirGraphStEphChap6_1");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    let n: N = 50_000;
    group.bench_with_input(BenchmarkId::new("build_vertices_arcs", n), &n, |b, &len| {
        b.iter(|| {
            let mut v: Set<N> = Set::empty();
            for i in 0..len {
                let _ = Set::insert(&mut v, i);
            }
            let mut a: Set<Pair<N, N>> = Set::empty();
            for i in 0..len {
                let _ = Set::insert(&mut a, Pair(i, (i + 1) % len));
            }
            let g = DirGraphStEph::FromSets(v, a);
            black_box(g)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_dirgraph_build);
criterion_main!(benches);
