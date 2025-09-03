use apas_ai::{SharedSLList, SharedSList, B, N};
use apas_ai::{SharedSListChap18, SharedSListChap19};

#[test]
fn test_cons_head_tail_and_sharing() {
    let empty: SharedSLList<N> = SharedSLList::new();
    assert_eq!(SharedSList::isEmpty(&empty), B::True);
    let l1 = SharedSLList::cons(2, &empty);
    let l2 = SharedSLList::cons(1, &l1);
    assert_eq!(SharedSList::head(&l2), Some(&1));
    let t = SharedSList::tail(&l2);
    // tails should share head pointer with l1
    assert!(l1.ptr_eq_head(&t));
}

#[test]
fn test_map() {
    let empty: SharedSLList<N> = SharedSLList::new();
    let a = SharedSLList::cons(1, &SharedSLList::cons(2, &empty));
    let mapped = SharedSLList::map(&a, |x| x + 10);
    assert_eq!(SharedSList::head(&mapped), Some(&11));
}

#[test]
fn test_append() {
    let empty: SharedSLList<N> = SharedSLList::new();
    let a = SharedSLList::cons(1, &SharedSLList::cons(2, &empty));
    let b = SharedSLList::cons(3, &SharedSLList::cons(4, &empty));
    let appended = SharedSLList::append(&a, &b);
    assert_eq!(SharedSList::head(&appended), Some(&1));
    // check that b is shared as tail of appended by pointer equality on head of appended.tail.tail
    let t1 = SharedSList::tail(&appended); // 2 :: b
    let t2 = SharedSList::tail(&t1); // b
    assert!(b.ptr_eq_head(&t2));
}

#[test]
fn test_ch18_tabulate_map_filter_append() {
    let a = <SharedSLList<N> as SharedSListChap18>::tabulate(|i| i + 1, 5);
    let b = <SharedSLList<N> as SharedSListChap18>::map(&a, |x| x + 1);
    let c = <SharedSLList<N> as SharedSListChap18>::append(&a, &b);
    // filter evens
    let evens = <SharedSLList<N> as SharedSListChap18>::filter(&c, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert!(evens.length() > 0);
}

#[test]
fn test_ch19_select_scan_flatten() {
    let a = <SharedSLList<N> as SharedSListChap19>::tabulate(|i| i + 1, 4);
    let b = <SharedSLList<N> as SharedSListChap19>::map(&a, |x| x * 2);
    assert_eq!(<SharedSLList<N> as SharedSListChap19>::select(&a, &b, 0), Some(&1));
    let (_p, t) = <SharedSLList<N> as SharedSListChap19>::scan(&a, &|x: &N, y: &N| x + y, 0);
    assert_eq!(t, 10);
    // flatten [[1,2],[3,4]]
    let ab = <SharedSLList<N> as SharedSListChap19>::append(&a, &b); // just to exercise append here too
    let nested = SharedSLList::cons(a, &SharedSLList::cons(b, &SharedSLList::new()));
    let flat = <SharedSLList<N> as SharedSListChap19>::flatten(&nested);
    assert!(flat.length() >= 4);
    let _ = ab; // silence unused
}


