pub mod TestLinkedListPerChap19 {
use apas_ai::Types::Types::*;
use apas_ai::LinkedListPer::LinkedListPer::*;
use apas_ai::LinkedListPer; // macro import
use apas_ai::LinkedListPerChap18::LinkedListPerChap18Trait;
use apas_ai::LinkedListPerChap19::LinkedListPerChap19Trait;

#[test]
fn test_select() {
    let a = LinkedListPer![1, 2, 3];
    let b = LinkedListPer![4, 5];

    let sel0 = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::select(&a, &b, 0);
    let sel3 = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::select(&a, &b, 3);
    let sel5 = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::select(&a, &b, 5);
    assert_eq!(sel0, Some(1));
    assert_eq!(sel3, Some(4));
    assert_eq!(sel5, None);
}

#[test]
fn test_append_variants() {
    let a = LinkedListPer![1, 2, 3];
    let b = LinkedListPer![4, 5];
    let c = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::append(&a, &b);
    let c2 = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::append2(&a, &b);
    assert_eq!(c, LinkedListPer![1, 2, 3, 4, 5]);
    assert_eq!(c2, LinkedListPer![1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate() {
    let one = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::deflate(|x| if *x > 0 { B::True } else { B::False }, &7);
    assert_eq!(one, LinkedListPer![7]);
    let none = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::deflate(|x| if *x > 0 { B::True } else { B::False }, &0);
    assert_eq!(none, LinkedListPerS::new(0,0));
}

#[test]
fn test_map() {
    let a = LinkedListPer![1, 2, 3];
    let b = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::map(&a, |x| x + 1);
    assert_eq!(b, LinkedListPer![2, 3, 4]);
}

#[test]
fn test_iterate_and_reduce() {
    let a = LinkedListPer![1, 2, 3];
    let sum = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::iterate(&a, |acc, x| acc + x, 0);
    assert_eq!(sum, 6);
    let red = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(red, 6);
}

#[test]
fn test_scan() {
    let a = LinkedListPer![1, 2, 3];
    let (prefixes, total) = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(prefixes, LinkedListPer![0, 1, 3]);
    assert_eq!(total, 6);
}

#[test]
fn test_flatten() {
    let nested = LinkedListPer![
        LinkedListPer![1, 2],
        LinkedListPer![3]
    ];
    let flat = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::flatten(&nested);
    assert_eq!(flat, LinkedListPer![1, 2, 3]);
}

#[test]
fn test_inject() {
    let values = LinkedListPer![10, 20, 30];
    let changes = LinkedListPer![(1, 99)];
    let injected = <LinkedListPerS<N> as LinkedListPerChap19Trait<T>>::inject(&values, &changes);
    assert_eq!(injected, LinkedListPer![10, 99, 30]);
}
}

