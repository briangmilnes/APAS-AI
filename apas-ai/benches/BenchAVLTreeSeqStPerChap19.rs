use apas_ai::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS;
use apas_ai::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_ai::AVLTreeSeqStPerChap19Trait;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_build_and_read_persistent(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqPerChap19");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("tabulate_then_nth", n), &n, |b, &len| {
        b.iter(|| {
            let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap19Trait<N>>::tabulate(|i| i, len);
            let v = *<AVLTreeSeqStPerS<N> as AVLTreeSeqStPerTrait<N>>::nth(&t, len - 1);
            black_box((t.length(), v))
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_build_and_read_persistent);
criterion_main!(benches);
