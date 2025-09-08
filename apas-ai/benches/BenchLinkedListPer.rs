use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::{LinkedListPerS, LinkedListPerTrait};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_sll_persistent_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListPer");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 1_000; // reduce N to ensure ≤10s total

    group.bench_with_input(BenchmarkId::new("new_then_updates", n), &n, |b, &len| {
        b.iter(|| {
            let mut s: LinkedListPerS<N> = <LinkedListPerS<N> as LinkedListPerTrait<N>>::new(len, 0);
            for i in 0..len { s = <LinkedListPerS<N> as LinkedListPerTrait<N>>::set(&s, i, i).unwrap(); }
            black_box(<LinkedListPerS<N> as LinkedListPerTrait<N>>::length(&s))
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_sll_persistent_ops);
criterion_main!(benches);


