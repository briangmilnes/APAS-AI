use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::AVLTreeSeq::{AVLTreeS, AVLTreeSeq};
use apas_ai::AVLTreeSeqChap18::AVLTreeSeqChap18;
use apas_ai::AVLTreeSeqChap19::AVLTreeSeqChap19;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_build_and_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeS_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("tabulate_then_contains", n), &n, |b, &len| {
        b.iter(|| {
            let t: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeqChap19>::tabulate(|i| i, len);
            let hit = <AVLTreeS<N> as AVLTreeSeq<N>>::isSingleton(&t) == apas_ai::Types::B::True; // cheap read
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


