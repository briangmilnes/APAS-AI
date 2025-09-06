use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::{SLPersistent, SinglyListPersistentTrait, SinglyListPersistentChap18Trait};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_sll_persistent_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("SinglyLinkedListPersistent_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    let n: N = 20_000; // persistent rebuild per update; keep modest

    group.bench_with_input(BenchmarkId::new("new_then_updates", n), &n, |b, &len| {
        b.iter(|| {
            let mut s: SLPersistent<N> = <SLPersistent<N> as SinglyListPersistentTrait<N>>::new(len, 0);
            // perform updates at rising indices (rebuild each time)
            for i in 0..len { s = <SLPersistent<N> as SinglyListPersistentChap18Trait>::update(&s, (i, i)); }
            black_box(s.length())
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_sll_persistent_ops);
criterion_main!(benches);


