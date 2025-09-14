pub mod TestLinkedListPerChap18 {
use apas_ai::Types::Types::*;
use apas_ai::LinkedListPer::LinkedListPer::*;
use apas_ai::LinkedListPer; // macro import
use apas_ai::LinkedListPerChap18::*;

#[test]
fn test_tabulate() {
    let a = <LinkedListPerS<N> as LinkedListPerChap18Trait>::tabulate(|i| i * 2, 5);
    assert_eq!(a.length(), 5);
    assert_eq!(a.nth(3), &6);
}

#[test]
fn test_map() {
    let a = <LinkedListPerS<N> as LinkedListPerChap18Trait>::tabulate(|i| i * 2, 5);
    let b = <LinkedListPerS<N> as LinkedListPerChap18Trait>::map(&a, |x| x + 1);
    assert_eq!(b.nth(0), &1);
    assert_eq!(b.nth(4), &9);
}

#[test]
fn test_filter() {
    let a = <LinkedListPerS<N> as LinkedListPerChap18Trait>::tabulate(|i| i + 1, 5);
    let c = <LinkedListPerS<N> as LinkedListPerChap18Trait>::filter(&a, |x| if *x % 2 == 1 { B::True } else { B::False });
    assert_eq!(c, LinkedListPer![1, 3, 5]);
}

#[test]
fn test_append() {
    let a = LinkedListPer![0, 2, 4, 6, 8];
    let b = LinkedListPer![1, 3, 5, 7, 9];
    let d = <LinkedListPerS<N> as LinkedListPerChap18Trait>::append(&a, &b);
    assert_eq!(d, LinkedListPer![0, 2, 4, 6, 8, 1, 3, 5, 7, 9]);
}

#[test]
fn test_update() {
    let a = LinkedListPer![1, 2, 3, 2, 1];
    let upd = <LinkedListPerS<N> as LinkedListPerChap18Trait>::update(&a, (1, 9));
    assert_eq!(upd, LinkedListPer![1, 9, 3, 2, 1]);
}

#[test]
fn test_inject() {
    let a = LinkedListPer![1, 2, 3, 2, 1];
    let changes = LinkedListPer![(0, 7), (3, 5)];
    let inj = <LinkedListPerS<N> as LinkedListPerChap18Trait>::inject(&a, &changes);
    assert_eq!(inj, LinkedListPer![7, 2, 3, 5, 1]);
}

#[test]
fn test_ninject() {
    let a = LinkedListPer![1, 2, 3, 2, 1];
    let changes = LinkedListPer![(0, 7), (3, 5)];
    let ninj = <LinkedListPerS<N> as LinkedListPerChap18Trait>::ninject(&a, &changes);
    assert_eq!(ninj, LinkedListPer![7, 2, 3, 5, 1]);
}

#[test]
fn test_iterate() {
    let a = LinkedListPer![1, 2, 3, 2, 1];
    let sum = <LinkedListPerS<N> as LinkedListPerChap18Trait>::iterate(&a, |acc, x| acc + x, 0);
    assert_eq!(sum, 9);
}

#[test]
fn test_reduce() {
    let a = LinkedListPer![1, 2, 3, 2, 1];
    let red = <LinkedListPerS<N> as LinkedListPerChap18Trait>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(red, 9);
}

#[test]
fn test_scan() {
    let a = LinkedListPer![1, 2, 3, 2, 1];
    let (prefixes, total) = <LinkedListPerS<N> as LinkedListPerChap18Trait>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(prefixes, LinkedListPer![0, 1, 3, 6, 8]);
    assert_eq!(total, 9);
}

#[test]
fn test_flatten() {
    let nested = LinkedListPer![
        LinkedListPer![1, 2],
        LinkedListPer![3],
        LinkedListPer![4, 5],
    ];
    let flat = <LinkedListPerS<N> as LinkedListPerChap18Trait>::flatten(&nested);
    assert_eq!(flat, LinkedListPer![1, 2, 3, 4, 5]);
}

#[test]
fn test_collect() {
    let pairs = LinkedListPer![(1, 10), (2, 20), (1, 30)];
    let grouped = <LinkedListPerS<(N, N)> as LinkedListPerChap18Trait>::collect(&pairs, |a, b| if a < b { O::Less } else if a > b { O::Greater } else { O::Equal });
    // Expect two groups with keys 1 and 2
    assert_eq!(grouped.length(), 2);
}
}
