use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqPer::ArraySeqPer::ArrayPerS;
use apas_ai::ArraySeqPerChap18::ArraySeqPerChap18Trait;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

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

fn bench_build_random_s_persistent(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    let n: N = 50_000; // smaller than ephemeral: persistent updates allocate

    group.bench_with_input(BenchmarkId::new("zeros_then_persistent_update", n), &n, |b, &len| {
        b.iter(|| {
            let mut rng = LinearCongruentialGenerator32::new(0xDEADBEEF);
            let mut s: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerChap19Trait<T>>::tabulate(|_| 0, len);
            for i in 0..len {
                s = <ArrayPerS<N> as ArraySeqPerChap18Trait<T>>::update(&s, (i, rng.next_N()));
            }
            black_box(s)
        })
    });

    group.finish();

    // Print HTML report URL
    let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("target"));
    let report = target_dir.join("criterion").join("report").join("index.html");
    println!("HTML report: file://{}", report.display());
}

criterion_group!(benches, bench_build_random_s_persistent);
criterion_main!(benches);


