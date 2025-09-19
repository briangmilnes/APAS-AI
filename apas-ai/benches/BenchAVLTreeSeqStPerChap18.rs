use apas_ai::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::AVLTreeSeqStPerChap18Trait;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_avl_per_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSeqPerChap18_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i, len);
            let m: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::map(&t, |x| x + 1);
            black_box((t.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_avl_per_ch18);
criterion_main!(benches);
