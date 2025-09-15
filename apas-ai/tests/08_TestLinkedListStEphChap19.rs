pub mod TestLinkedListEphChap19 {
use apas_ai::Types::Types::*;
use apas_ai::LinkedListEph::LinkedListEph::*;
use apas_ai::LinkedListEph; // macro import
use apas_ai::LinkedListEphChap19::LinkedListEphChap19Trait;

#[test]
fn test_eph_set_and_nth() {
    let mut a: LinkedListEphS<N> = LinkedListEph![1; 3];
    let _ = a.set(1, 9);
    assert_eq!(*a.nth(1), 9);
}

#[test]
fn test_eph_subseq_copy_and_display_debug() {
    let a: LinkedListEphS<N> = LinkedListEph![1; 3];
    let sub = a.subseq_copy(1, 2);
    assert_eq!(sub.length(), 2);
    let v = LinkedListEph![1, 2, 3];
    assert_eq!(format!("{:?}", v), "[1, 2, 3]");
    assert_eq!(format!("{}", v), "[1, 2, 3]");
}

#[test]
fn test_iter_inorder_collect_eph_ch19() {
    let a: LinkedListEphS<N> = LinkedListEph![2, 4, 6];
    let vals: Vec<N> = a.iter().copied().collect();
    assert_eq!(vals, vec![2, 4, 6]);
}

#[test]
fn test_tabulate_map_select_append_ch19() {
    let a: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::tabulate(|i| i, 4);
    let b: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::map(&a, |x| x + 10);
    assert_eq!(b.iter().copied().collect::<Vec<N>>(), vec![10, 11, 12, 13]);
    let c = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::append(&a, &b);
    assert_eq!(c.iter().copied().collect::<Vec<N>>(), vec![0, 1, 2, 3, 10, 11, 12, 13]);
    assert_eq!(<LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::select(&a, &b, 6), Some(12));
}

#[test]
fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {
    let one = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::deflate(|x| if *x == 1 { B::True } else { B::False }, &1);
    assert_eq!(one.iter().copied().collect::<Vec<N>>(), vec![1]);
    let a: LinkedListEphS<N> = LinkedListEph![1, 2, 3, 4];
    let even = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(even.iter().copied().collect::<Vec<N>>(), vec![2, 4]);
    let sum = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(sum, 10);
    let (prefixes, total) = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(total, 10);
    assert_eq!(prefixes.iter().copied().collect::<Vec<N>>(), vec![0, 1, 3, 6]);
    let outer: LinkedListEphS<LinkedListEphS<N>> = LinkedListEph![LinkedListEph![1], LinkedListEph![2, 3]];
    let flat = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::flatten(&outer);
    assert_eq!(flat.iter().copied().collect::<Vec<N>>(), vec![1, 2, 3]);
    let ups: LinkedListEphS<(N, N)> = LinkedListEph![(1, 9), (2, 8)];
    let inj = <LinkedListEphS<N> as LinkedListEphChap19Trait<T>>::inject(&a, &ups);
    assert_eq!(inj.iter().copied().collect::<Vec<N>>(), vec![1, 9, 8, 4]);
}

}

