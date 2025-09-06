use apas_ai::Types::{N, B};
use apas_ai::LinkedListPer::{LinkedListPerS, LinkedListPerTrait};
use apas_ai::LinkedListPerChap18::LinkedListPerChap18Trait;
use apas_ai::LinkedListPerChap19::LinkedListPerChap19Trait;

#[test]
fn test_select() {
    let a = LinkedListPerS::from_vec(vec![1, 2, 3]);
    let b = LinkedListPerS::from_vec(vec![4, 5]);

    let sel0 = <LinkedListPerS<N> as LinkedListPerChap19Trait>::select(&a, &b, 0);
    let sel3 = <LinkedListPerS<N> as LinkedListPerChap19Trait>::select(&a, &b, 3);
    let sel5 = <LinkedListPerS<N> as LinkedListPerChap19Trait>::select(&a, &b, 5);
    assert_eq!(sel0, Some(1));
    assert_eq!(sel3, Some(4));
    assert_eq!(sel5, None);
}

#[test]
fn test_append_variants() {
    let a = LinkedListPerS::from_vec(vec![1, 2, 3]);
    let b = LinkedListPerS::from_vec(vec![4, 5]);
    let c = <LinkedListPerS<N> as LinkedListPerChap19Trait>::append(&a, &b);
    let c2 = <LinkedListPerS<N> as LinkedListPerChap19Trait>::append2(&a, &b);
    assert_eq!(c, LinkedListPerS::from_vec(vec![1, 2, 3, 4, 5]));
    assert_eq!(c2, LinkedListPerS::from_vec(vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_deflate() {
    let one = <LinkedListPerS<N> as LinkedListPerChap19Trait>::deflate(|x| if *x > 0 { B::True } else { B::False }, &7);
    assert_eq!(one, LinkedListPerS::from_vec(vec![7]));
    let none = <LinkedListPerS<N> as LinkedListPerChap19Trait>::deflate(|x| if *x > 0 { B::True } else { B::False }, &0);
    assert_eq!(none, LinkedListPerS::from_vec(Vec::new()));
}

#[test]
fn test_map() {
    let a = LinkedListPerS::from_vec(vec![1, 2, 3]);
    let b = <LinkedListPerS<N> as LinkedListPerChap19Trait>::map(&a, |x| x + 1);
    assert_eq!(b, LinkedListPerS::from_vec(vec![2, 3, 4]));
}

#[test]
fn test_iterate_and_reduce() {
    let a = LinkedListPerS::from_vec(vec![1, 2, 3]);
    let sum = <LinkedListPerS<N> as LinkedListPerChap19Trait>::iterate(&a, |acc, x| acc + x, 0);
    assert_eq!(sum, 6);
    let red = <LinkedListPerS<N> as LinkedListPerChap19Trait>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(red, 6);
}

#[test]
fn test_scan() {
    let a = LinkedListPerS::from_vec(vec![1, 2, 3]);
    let (prefixes, total) = <LinkedListPerS<N> as LinkedListPerChap19Trait>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(prefixes, LinkedListPerS::from_vec(vec![0, 1, 3]));
    assert_eq!(total, 6);
}

#[test]
fn test_flatten() {
    let nested = LinkedListPerS::from_vec(vec![
        LinkedListPerS::from_vec(vec![1, 2]),
        LinkedListPerS::from_vec(vec![3]),
    ]);
    let flat = <LinkedListPerS<N> as LinkedListPerChap19Trait>::flatten(&nested);
    assert_eq!(flat, LinkedListPerS::from_vec(vec![1, 2, 3]));
}

#[test]
fn test_inject() {
    let values = LinkedListPerS::from_vec(vec![10, 20, 30]);
    let changes = LinkedListPerS::from_vec(vec![(1, 99)]);
    let injected = <LinkedListPerS<N> as LinkedListPerChap19Trait>::inject(&values, &changes);
    assert_eq!(injected, LinkedListPerS::from_vec(vec![10, 99, 30]));
}

