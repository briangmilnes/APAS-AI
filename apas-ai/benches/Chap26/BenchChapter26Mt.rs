//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::{
    ArraySeqMtPerBaseTrait, ArraySeqMtPerRedefinableTrait, ArraySeqMtPerS,
};
use apas_ai::*;

fn gen_sequence(n: usize) -> ArraySeqMtPerS<usize> { ArraySeqMtPerS::new(n, 0) }

fn bench_chapter26_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chapter26Mt");
    group.sample_size(30);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[1_024usize, 8_192] {
        group.bench_with_input(BenchmarkId::new("persistent_set", n), &n, |b, &len| {
            b.iter_batched(
                || gen_sequence(len),
                |mut seq| {
                    for i in 0..len {
                        seq = seq.set(i, len - i).unwrap();
                    }
                    black_box(seq)
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("subseq_copy", n), &n, |b, &len| {
            b.iter_batched(
                || gen_sequence(len),
                |seq| {
                    let copied = seq.subseq_copy(len / 4, len / 2);
                    black_box((seq, copied))
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_chapter26_mt);
criterion_main!(benches);
