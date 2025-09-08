use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::AVLTreeSeqEph::{AVLTreeSeqEphS, AVLTreeSeqEphTrait};
use std::time::Duration;

fn bench_avl_eph_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqEphChap19");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("push_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let mut t: AVLTreeSeqEphS<N> = <AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::empty();
            for i in 0..len { t.push_back(i); }
            let mut m: AVLTreeSeqEphS<N> = <AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::empty();
            for i in 0..len { m.push_back(*<AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::nth(&t, i) + 1); }
            black_box((<AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::length(&t), <AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::length(&m)))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_avl_eph_ch19);
criterion_main!(benches);
