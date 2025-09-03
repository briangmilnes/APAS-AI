use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::DoublyLinkedListSeq::{DoublyLinkedListS as ListS, DoublyLinkedListSeq as ListSeq};
use apas_ai::DoublyLinkedListSeqChap18::DoublyLinkedListSeqChap18 as ListSeqChap18;
use apas_ai::DoublyLinkedListSeqChap19::DoublyLinkedListSeqChap19 as ListSeqChap19;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn bench_listseq_basics(c: &mut Criterion) {
    let mut group = c.benchmark_group("DoublyLinkedListS_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    let n: N = 1_000;

    group.bench_with_input(BenchmarkId::new("new_then_set", n), &n, |b, &len| {
        b.iter(|| {
            let mut s = <ListS<N> as ListSeqChap19>::tabulate(|_| 0, len);
            for i in 0..len { let _ = <ListS<N> as ListSeq<N>>::set(&mut s, i, i); }
            black_box(s)
        })
    });

    group.bench_with_input(BenchmarkId::new("append_then_iterate", n), &n, |b, &len| {
        b.iter(|| {
            let a = <ListS<N> as ListSeqChap18>::tabulate(|_| 1, len / 2);
            let b2 = <ListS<N> as ListSeqChap18>::tabulate(|_| 2, len - len / 2);
            let out = <ListS<N> as ListSeqChap18>::append(&a, &b2);
            let mut sum: N = 0;
            for i in 0..out.length() { sum += *out.nth(i); }
            black_box(sum)
        })
    });

    group.finish();

    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_listseq_basics);
criterion_main!(benches);


