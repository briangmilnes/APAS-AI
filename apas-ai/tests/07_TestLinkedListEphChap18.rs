use apas_ai::Types::{N, B, O};
use apas_ai::LinkedListEph::{LinkedListEphS, LinkedListEphTrait};
use apas_ai::LinkedListEph;
use apas_ai::LinkedListEphChap18Trait;

// Eph Chap18 algorithms are not implemented; we reference expected outcomes via simple constructions.

#[test]
fn test_construct_eph_from_vec() {
    let a: LinkedListEphS<N> = LinkedListEph![1, 2, 3];
    assert_eq!(a.length(), 3);
}

#[test]
fn test_eph_is_empty_and_singleton() {
    let e: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::empty();
    assert_eq!(e.isEmpty(), B::True);
    let s = LinkedListEph![1];
    assert_eq!(s.isSingleton(), B::True);
}

#[test]
fn test_eph_set_and_subseq_copy() {
    let mut a: LinkedListEphS<N> = LinkedListEph![0; 3];
    let _ = a.set(1, 2);
    assert_eq!(*a.nth(1), 2);
    let sub = a.subseq_copy(1, 2);
    assert_eq!(sub.length(), 2);
}

#[test]
fn test_iter_inorder_collect_eph_ch18() {
    let a: LinkedListEphS<N> = LinkedListEph![1, 3, 5];
    let vals: Vec<N> = a.iter().copied().collect();
    assert_eq!(vals, vec![1, 3, 5]);
}

#[test]
fn test_tabulate_and_map_ch18() {
    let a: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphChap18Trait>::tabulate(|i| i, 5);
    let b: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphChap18Trait>::map(&a, |x| x + 1);
    let vals: Vec<N> = b.iter().copied().collect();
    assert_eq!(vals, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_append_ch18() {
    let a: LinkedListEphS<N> = LinkedListEph![1, 2];
    let b: LinkedListEphS<N> = LinkedListEph![2, 3];
    let c: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphChap18Trait>::append(&a, &b);
    let vals: Vec<N> = c.iter().copied().collect();
    assert_eq!(vals, vec![1, 2, 2, 3]);
}

#[test]
fn test_filter_ch18() {
    let a: LinkedListEphS<N> = LinkedListEph![1, 2, 3, 4];
    let b = <LinkedListEphS<N> as LinkedListEphChap18Trait>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    let vals: Vec<N> = b.iter().copied().collect();
    assert_eq!(vals, vec![2, 4]);
}

#[test]
fn test_update_ch18() {
    let mut a: LinkedListEphS<N> = <LinkedListEphS<N> as LinkedListEphTrait<N>>::new(3, 0);
    let _ = <LinkedListEphS<N> as LinkedListEphChap18Trait>::update(&mut a, (1, 7));
    let vals: Vec<N> = a.iter().copied().collect();
    assert_eq!(vals, vec![0, 7, 0]);
}

#[test]
fn test_inject_and_ninject_ch18() {
    let a: LinkedListEphS<N> = LinkedListEph![0, 0, 0, 0];
    let ups: LinkedListEphS<(N, N)> = LinkedListEph![(1, 9), (1, 5), (3, 6)];
    let inj = <LinkedListEphS<N> as LinkedListEphChap18Trait>::inject(&a, &ups);
    let ninj = <LinkedListEphS<N> as LinkedListEphChap18Trait>::ninject(&a, &ups);
    assert_eq!(inj.iter().copied().collect::<Vec<N>>(), vec![0, 9, 0, 6]);
    assert_eq!(ninj.iter().copied().collect::<Vec<N>>(), vec![0, 5, 0, 6]);
}

#[test]
fn test_iterate_reduce_scan_ch18() {
    let a: LinkedListEphS<N> = LinkedListEph![1, 2, 3];
    let sum = <LinkedListEphS<N> as LinkedListEphChap18Trait>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(sum, 6);
    let pref = <LinkedListEphS<N> as LinkedListEphChap18Trait>::iterate(&a, |acc, x| acc + x, 0);
    assert_eq!(pref, 6);
    let (prefixes, total) = <LinkedListEphS<N> as LinkedListEphChap18Trait>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(total, 6);
    assert_eq!(prefixes.iter().copied().collect::<Vec<N>>(), vec![0, 1, 3]);
}

#[test]
fn test_flatten_and_collect_ch18() {
    let x: LinkedListEphS<N> = LinkedListEph![1, 2];
    let y: LinkedListEphS<N> = LinkedListEph![3];
    let outer: LinkedListEphS<LinkedListEphS<N>> = LinkedListEph![x, y];
    let flat = <LinkedListEphS<N> as LinkedListEphChap18Trait>::flatten(&outer);
    assert_eq!(flat.iter().copied().collect::<Vec<N>>(), vec![1, 2, 3]);

    let pairs: LinkedListEphS<(N, N)> = LinkedListEph![(1, 10), (2, 20), (1, 30)];
    let grouped = <LinkedListEphS<(N, N)> as LinkedListEphChap18Trait>::collect(&pairs, |a, b| if a == b { O::Equal } else if a < b { O::Less } else { O::Greater });
    // Expect keys 1 and 2 with their grouped lists
    assert_eq!(<LinkedListEphS<(N, LinkedListEphS<N>)> as LinkedListEphTrait<(N, LinkedListEphS<N>)>>::length(&grouped), 2);
}

