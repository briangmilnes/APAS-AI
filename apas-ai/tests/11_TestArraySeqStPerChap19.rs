pub mod TestArraySeqPerChap19 {
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::ArraySeqStPerChap18::ArraySeqStPerChap18Trait;
use apas_ai::ArraySeqStPerChap19::ArraySeqStPerChap19Trait;
use apas_ai::ArraySeqStPer; // macro import
use std::sync::Mutex;

#[test]
fn test_map_and_select_and_append() {
    let a = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|i| i + 1, 3);
    let b = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::map(&a, |x| x * 2);
    assert_eq!(b, ArraySeqStPer![2, 4, 6]);
    assert_eq!(<ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::select(&a, &b, 0), Some(&1));
    let c = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStPer![1, 2, 3, 2, 4, 6]);
}

#[test]
fn test_deflate_and_filter() {
    let y = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &6);
    assert_eq!(y, ArraySeqStPer![6]);
    let a = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|i| i + 1, 10);
    let evens = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArraySeqStPer![2, 4, 6, 8, 10]);
}

#[test]
fn test_iterate_reduce_scan_flatten() {
    let a = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|i| i + 1, 5);
    let sum = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::iterate(&a, |acc: &N, x: &N| acc + x, 0);
    assert_eq!(sum, 15);
    let sum_fn = |x: &N, y: &N| x + y;
    let r = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let (prefixes, total) = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::scan(&a, &sum_fn, 0);
    assert_eq!(total, 15);
    assert_eq!(*prefixes.nth(4), 10);
    let nested: ArrayStPerS<ArrayStPerS<N>> = ArraySeqStPer![
        <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|i| i + 1, 2),
        <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|i| i + 3, 2),
    ];
    let flat = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::flatten(&nested);
    assert_eq!(flat, ArraySeqStPer![1, 2, 3, 4]);
}

#[test]
fn test_inject_and_parallel() {
    let values = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::tabulate(|i| i, 6);
    let changes: ArrayStPerS<Pair<N, N>> = ArraySeqStPer![Pair(2, 99), Pair(2, 7), Pair(4, 20)];
    let serial = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::inject(&values, &changes);
    let parallel = <ArrayStPerS<N> as ArraySeqStPerChap19Trait<N>>::inject_parallel2(&values, &changes);
    assert_eq!(serial, parallel);

    let n = values.length();
    let with_num: ArrayStPerS<Mutex<Pair<N, N>>> = <ArrayStPerS<Mutex<Pair<N, N>>> as ArraySeqStPerChap19Trait<Mutex<Pair<N, N>>>>::tabulate(
        |i| Mutex::new(Pair(values.nth(i).clone(), n)), n);
    <ArrayStPerS<Mutex<Pair<N, N>>>> as ArraySeqStPerChap19Trait<Mutex<Pair<N, N>>>>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 1);
    <ArrayStPerS<Mutex<Pair<N, N>>>> as ArraySeqStPerChap19Trait<Mutex<Pair<N, N>>>>::AtomicWriteLowestChangeNumberWins(&with_num, &changes, 0);
    let guard = with_num.nth(2).lock().unwrap();
    assert_eq!(guard.0, 99);
}

}


