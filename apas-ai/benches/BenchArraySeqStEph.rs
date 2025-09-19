use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::ArraySeqStEphChap18Trait;
use apas_ai::ArraySeqStEphChap19Trait;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

// Simple Linear Congruential Generator: x_{n+1} = (a * x_n + c) mod 2^32
struct LinearCongruentialGenerator32 {
    state: u32,
}

impl LinearCongruentialGenerator32 {
    fn new(seed: u32) -> LinearCongruentialGenerator32 {
        LinearCongruentialGenerator32 { state: seed }
    }
    #[inline]
    fn next_N(&mut self) -> N {
        // Numerical Recipes parameters
        const A: u32 = 1664525;
        const C: u32 = 1013904223;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state as N
    }
}

fn bench_build_random_s(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySeqEph_random_updates");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 100_000;

    group.bench_with_input(BenchmarkId::new("zeros_then_update", n), &n, |b, &len| {
        b.iter(|| {
            let mut rng = LinearCongruentialGenerator32::new(0xDEADBEEF);
            let mut s = <ArraySeqStEphS<N> as ArraySeqStEphChap19Trait<N>>::tabulate(|_| 0, len);
            for i in 0..len {
                let _ = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::update(&mut s, (i, rng.next_N()));
            }
            black_box(s)
        })
    });

    group.finish();

    // Print HTML report URL (criterion html_reports is enabled in Cargo.toml)
    let target_dir: PathBuf =
        env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_build_random_s);
criterion_main!(benches);
