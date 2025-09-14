use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::AVLTreeSeqPer::AVLTreeSeqPer::*;
use apas_ai::AVLTreeSeqPerChap19::AVLTreeSeqPerChap19Trait;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_build_and_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchAVLTreeSeqPer");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("tabulate_then_contains", n), &n, |b, &len| {
        b.iter(|| {
            let t: AVLTreeSeqPerS<N> = <AVLTreeSeqPerS<N> as AVLTreeSeqPerChap19Trait>::tabulate(|i| i, len);
            let hit = <AVLTreeSeqPerS<N> as AVLTreeSeqPerTrait<N>>::isSingleton(&t) == B::True; // cheap read
            black_box((t.length(), hit))
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_build_and_contains);
criterion_main!(benches);


