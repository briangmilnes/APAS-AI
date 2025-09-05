use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::SinglyLinkedListSeq::{SinglyLinkedListS, SinglyLinkedListSeq};
use apas_ai::SinglyLinkedListSeqChap18::SinglyLinkedListSeqChap18;
use apas_ai::SinglyLinkedListSeqChap19::SinglyLinkedListSeqChap19;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_tabulate_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("SinglyLinkedListS_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: SinglyLinkedListS<N> = <SinglyLinkedListS<N> as SinglyLinkedListSeqChap19>::tabulate(|i| i, len);
            let m: SinglyLinkedListS<N> = <SinglyLinkedListS<N> as SinglyLinkedListSeqChap19>::map(&s, |x| x + 1);
            black_box((s.length(), m.length()))
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_tabulate_map);
criterion_main!(benches);


