use apas_ai::Types::{N, B};
use apas_ai::{ArrayPerS, ArraySeqPerTrait, ArraySeqPerChap18Trait, ArraySeqPerChap19Trait};
use std::sync::Mutex;

#[test]
fn test_map_and_select_and_append() {
    let a = <ArrayPerS<N> as ArraySeqPerChap19Trait>::tabulate(|i| i + 1, 3);
    let b = <ArrayPerS<N> as ArraySeqPerChap19Trait>::map(&a, |x| x * 2);
    assert_eq!(b, ArrayPerS::from_vec(vec![2, 4, 6]));
    assert_eq!(<ArrayPerS<N> as ArraySeqPerChap19Trait>::select(&a, &b, 0), Some(&1));
    let c = <ArrayPerS<N> as ArraySeqPerChap19Trait>::append(&a, &b);
    assert_eq!(c, ArrayPerS::from_vec(vec![1, 2, 3, 2, 4, 6]));
}

#[test]
fn test_deflate_and_filter() {
    let y = <ArrayPerS<N> as ArraySeqPerChap19Trait>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &6);
    assert_eq!(y, ArrayPerS::from_vec(vec![6]));
    let a = <ArrayPerS<N> as ArraySeqPerChap19Trait>::tabulate(|i| i + 1, 10);
    let evens = <ArrayPerS<N> as ArraySeqPerChap19Trait>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArrayPerS::from_vec(vec![2, 4, 6, 8, 10]));
}

#[test]
fn test_iterate_reduce_scan_flatten() {
    let a = <ArrayPerS<N> as ArraySeqPerChap19Trait>::tabulate(|i| i + 1, 5);
    let sum = <ArrayPerS<N> as ArraySeqPerChap19Trait>::iterate(&a, |acc: &N, x: &N| acc + x, 0);
    assert_eq!(sum, 15);
    let sum_fn = |x: &N, y: &N| x + y;
    let r = <ArrayPerS<N> as ArraySeqPerChap19Trait>::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let (prefixes, total) = <ArrayPerS<N> as ArraySeqPerChap19Trait>::scan(&a, &sum_fn, 0);
    assert_eq!(total, 15);
    assert_eq!(*prefixes.nth(4), 10);
    let nested: ArrayPerS<ArrayPerS<N>> = ArrayPerS::from_vec(vec![
        <ArrayPerS<N> as ArraySeqPerChap19Trait>::tabulate(|i| i + 1, 2),
        <ArrayPerS<N> as ArraySeqPerChap19Trait>::tabulate(|i| i + 3, 2),
    ]);
    let flat = <ArrayPerS<N> as ArraySeqPerChap19Trait>::flatten(&nested);
    assert_eq!(flat, ArrayPerS::from_vec(vec![1, 2, 3, 4]));
}

#[test]
fn test_inject_and_parallel() {
    let values = <ArrayPerS<N> as ArraySeqPerChap19Trait>::tabulate(|i| i, 6);
    let changes: ArrayPerS<(N, N)> = ArrayPerS::from_vec(vec![(2, 99), (2, 7), (4, 20)]);
    let serial = <ArrayPerS<N> as ArraySeqPerChap19Trait>::inject(&values, &changes);
    let parallel = <ArrayPerS<N> as ArraySeqPerChap19Trait>::inject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);

    let n = values.length();
    let with_num: ArrayPerS<Mutex<(N, N)>> = <ArrayPerS<Mutex<(N, N)>> as ArraySeqPerChap19Trait>::tabulate(
        |i| Mutex::new((values.nth(i).clone(), n)), n);
    <ArrayPerS<Mutex<(N, N)>> as ArraySeqPerChap19Trait>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 1);
    <ArrayPerS<Mutex<(N, N)>> as ArraySeqPerChap19Trait>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 0);
    let guard = with_num.nth(2).lock().unwrap();
    assert_eq!(guard.0, 99);
}


