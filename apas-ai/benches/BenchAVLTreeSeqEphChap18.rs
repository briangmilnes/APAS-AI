use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::AVLTreeSeqEph::AVLTreeSeqEph::*;
use std::time::Duration;

fn bench_avl_eph_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqEphChap18");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("push_then_nth", n), &n, |b, &len| {
        b.iter(|| {
            let mut t: AVLTreeSeqEphS<N> = <AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::empty();
            for i in 0..len { t.push_back(i); }
            let v = *<AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::nth(&t, len - 1);
            black_box((<AVLTreeSeqEphS<N> as AVLTreeSeqEphTrait<N>>::length(&t), v))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_avl_eph_ch18);
criterion_main!(benches);
