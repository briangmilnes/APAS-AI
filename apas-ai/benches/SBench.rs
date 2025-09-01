use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use apas_ai::Sequences::*;

// Simple Linear Congruential Generator: x_{n+1} = (a * x_n + c) mod 2^32
struct LinearCongruentialGenerator32 { state: u32 }

impl LinearCongruentialGenerator32 {
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: seed } }
    #[inline]
    fn next_i32(&mut self) -> i32 {
        // Numerical Recipes parameters
        const A: u32 = 1664525;
        const C: u32 = 1013904223;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state as i32
    }
}

fn bench_build_random_s(c: &mut Criterion) {
    let mut group = c.benchmark_group("S_random_updates");
    let n: N = 1_000_000;

    group.bench_with_input(BenchmarkId::new("zeros_then_update", n), &n, |b, &len| {
        b.iter(|| {
            let mut rng = LinearCongruentialGenerator32::new(0xDEADBEEF);
            let mut s = <S<i32> as Sequence<i32>>::new(len, 0);
            for i in 0..len {
                // Update in place using only S APIs
                let _ = s.update((i, rng.next_i32()));
            }
            black_box(s)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_build_random_s);
criterion_main!(benches);


