//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::LinkedListStEph::LinkedListStEph::*;
use apas_ai::Types::Types::*;

fn bench_ll_eph_ch19(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchLinkedListEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::tabulate(&|i| i, len);
            let m: LinkedListStEphS<N> = <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::map(&s, &|x| x + 1);
            black_box((
                <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::length(&s),
                <LinkedListStEphS<N> as LinkedListStEphTrait<N>>::length(&m),
            ))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_ll_eph_ch19);
criterion_main!(benches);
