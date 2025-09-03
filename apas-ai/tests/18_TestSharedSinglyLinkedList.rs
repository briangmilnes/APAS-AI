use apas_ai::{SharedSLList, SharedSList, B};

#[test]
fn test_cons_head_tail_and_sharing() {
    let empty = <SharedSLList<usize> as SharedSList<usize>>::empty();
    assert_eq!(<SharedSLList<usize> as SharedSList<usize>>::isEmpty(&empty), B::True);
    let l1 = <SharedSLList<usize> as SharedSList<usize>>::cons(2, &empty);
    let l2 = <SharedSLList<usize> as SharedSList<usize>>::cons(1, &l1);
    assert_eq!(<SharedSLList<usize> as SharedSList<usize>>::head(&l2), Some(&1));
    let t = <SharedSLList<usize> as SharedSList<usize>>::tail(&l2);
    // tails should share head pointer with l1
    assert!(l1.ptr_eq_head(&t));
}

#[test]
fn test_map() {
    let empty = <SharedSLList<usize> as SharedSList<usize>>::empty();
    let a = <SharedSLList<usize> as SharedSList<usize>>::cons(1, &<SharedSLList<usize> as SharedSList<usize>>::cons(2, &empty));
    let mapped = <SharedSLList<usize> as SharedSList<usize>>::map(&a, |x| x + 10);
    assert_eq!(<SharedSLList<usize> as SharedSList<usize>>::head(&mapped), Some(&11));
}

#[test]
fn test_append() {
    let empty = <SharedSLList<usize> as SharedSList<usize>>::empty();
    let a = <SharedSLList<usize> as SharedSList<usize>>::cons(1, &<SharedSLList<usize> as SharedSList<usize>>::cons(2, &empty));
    let b = <SharedSLList<usize> as SharedSList<usize>>::cons(3, &<SharedSLList<usize> as SharedSList<usize>>::cons(4, &empty));
    let appended = <SharedSLList<usize> as SharedSList<usize>>::append(&a, &b);
    assert_eq!(<SharedSLList<usize> as SharedSList<usize>>::head(&appended), Some(&1));
    // check that b is shared as tail of appended by pointer equality on head of appended.tail.tail
    let t1 = <SharedSLList<usize> as SharedSList<usize>>::tail(&appended); // 2 :: b
    let t2 = <SharedSLList<usize> as SharedSList<usize>>::tail(&t1); // b
    assert!(b.ptr_eq_head(&t2));
}


