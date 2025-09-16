pub mod TestLinkedListPerChap19 {
use apas_ai::Types::Types::*;
use apas_ai::LinkedListStPer::LinkedListStPer::*;
use apas_ai::LinkedListStPer; // macro import
use apas_ai::LinkedListStPerChap19::LinkedListStPerChap19::*;

#[test]
fn test_select() {
    let a = LinkedListPer![1, 2, 3];
    let b = LinkedListPer![4, 5];

    let sel0 = select(&a, &b, 0);
    let sel3 = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::select(&a, &b, 3);
    let sel5 = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::select(&a, &b, 5);
    assert_eq!(sel0, Some(1));
    assert_eq!(sel3, Some(4));
    assert_eq!(sel5, None);
}

#[test]
fn test_append_variants() {
    let a = LinkedListPer![1, 2, 3];
    let b = LinkedListPer![4, 5];
    let c = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::append(&a, &b);
    let c2 = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::append2(&a, &b);
    assert_eq!(c, LinkedListStPer![1, 2, 3, 4, 5]);
    assert_eq!(c2, LinkedListStPer![1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate() {
    let one = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::deflate(|x| if *x > 0 { B::True } else { B::False }, &7);
    assert_eq!(one, LinkedListStPer![7]);
    let none = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::deflate(|x| if *x > 0 { B::True } else { B::False }, &0);
    assert_eq!(none, LinkedListStPerS::new(0,0));
}

#[test]
fn test_map() {
    let a = LinkedListPer![1, 2, 3];
    let b = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::map(&a, |x| x + 1);
    assert_eq!(b, LinkedListStPer![2, 3, 4]);
}

#[test]
fn test_iterate_and_reduce() {
    let a = LinkedListPer![1, 2, 3];
    let sum = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::iterate(&a, |acc, x| acc + x, 0);
    assert_eq!(sum, 6);
    let red = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(red, 6);
}

#[test]
fn test_scan() {
    let a = LinkedListPer![1, 2, 3];
    let (prefixes, total) = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(prefixes, LinkedListStPer![0, 1, 3]);
    assert_eq!(total, 6);
}

#[test]
fn test_flatten() {
    let nested = LinkedListStPer![
        LinkedListStPer![1, 2],
        LinkedListStPer![3]
    ];
    let flat = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::flatten(&nested);
    assert_eq!(flat, LinkedListStPer![1, 2, 3]);
}

#[test]
fn test_inject() {
    let values = LinkedListStPer![10, 20, 30];
    let changes = LinkedListStPer![Pair(1, 99)];
    let injected = <LinkedListStPerS<N> as LinkedListStPerChap19Trait<N>>::inject(&values, &changes);
    assert_eq!(injected, LinkedListStPer![10, 99, 30]);
}
}

