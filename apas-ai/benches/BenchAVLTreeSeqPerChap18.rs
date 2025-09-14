use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::AVLTreeSeqPer::AVLTreeSeqPer::*;
use apas_ai::AVLTreeSeqPerChap18::AVLTreeSeqPerChap18Trait;
use std::time::Duration;

fn bench_avl_per_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSeqPerChap18_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let t: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18Trait>::tabulate(|i| i, len);
            let m: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap18Trait>::map(&t, |x| x + 1);
            black_box((t.length(), m.length()))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_avl_per_ch18);
criterion_main!(benches);
