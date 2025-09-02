use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::N;
use apas_ai::ArraySeq::{ArrayS, ArraySeq};
use std::env;
use std::path::PathBuf;

// Simple Linear Congruential Generator: x_{n+1} = (a * x_n + c) mod 2^32
struct LinearCongruentialGenerator32 { state: u32 }

impl LinearCongruentialGenerator32 {
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: seed } }
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
    let mut group = c.benchmark_group("ArrayS_random_updates");
    group.sample_size(100);
    let n: N = 100_000;

    group.bench_with_input(BenchmarkId::new("zeros_then_update", n), &n, |b, &len| {
        b.iter(|| {
            let mut rng = LinearCongruentialGenerator32::new(0xDEADBEEF);
            let mut s = <ArrayS<N> as ArraySeq<N>>::new(len, 0);
            for i in 0..len {
                // Update in place using only S APIs
                let _ = s.update((i, rng.next_N()));
            }
            black_box(s)
        })
    });

    group.finish();

    // Print HTML report URL (criterion html_reports is enabled in Cargo.toml)
    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_build_random_s);
criterion_main!(benches);


