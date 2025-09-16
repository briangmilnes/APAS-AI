//! Tests for ArraySeqMtPerChap18 multithreaded algorithms.

pub mod TestArraySeqMtPerChap18 {
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::ArraySeqMtPerChap18::ArraySeqMtPerChap18::*;
use apas_ai::ArraySeqMtPer; // macro import

#[test]
fn test_tabulate_basic() {
    let a: ArrayMtPerS<N> = ArrayMtPerS::tabulate(|i| i, 5);
    assert_eq!(a.length(), 5);
    
    // Use set comparison since MT results may vary in order
    assert!(ArraySeqSetEq(
        a.length(),
        |i| *a.nth(i),
        5,
        |i| i
    ));
}

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }
    
    let fibs: ArrayMtPerS<N> = ArrayMtPerS::tabulate(fib, 8);
    assert_eq!(fibs.length(), 8);
    
    // Expected fibonacci sequence: [0, 1, 1, 2, 3, 5, 8, 13]
    let expected = [0, 1, 1, 2, 3, 5, 8, 13];
    assert!(ArraySeqSetEq(
        fibs.length(),
        |i| *fibs.nth(i),
        expected.len(),
        |i| expected[i]
    ));
}

#[test]
fn test_tabulate_empty() {
    let empty: ArrayMtPerS<N> = ArrayMtPerS::tabulate(|i| i * 2, 0);
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.isEmpty(), B::True);
}

#[test]
fn test_tabulate_single() {
    let single: ArrayMtPerS<N> = ArrayMtPerS::tabulate(|_| 42, 1);
    assert_eq!(single.length(), 1);
    assert_eq!(*single.nth(0), 42);
}

#[test]
fn test_tabulate_string() {
    let strings: ArrayMtPerS<String> = ArrayMtPerS::tabulate(
        |i| format!("item_{}", i), 
        4
    );
    assert_eq!(strings.length(), 4);
    
    // Check that all expected strings are present (order may vary in MT)
    let expected = ["item_0", "item_1", "item_2", "item_3"];
    assert!(ArraySeqSetEq(
        strings.length(),
        |i| strings.nth(i).clone(),
        expected.len(),
        |i| expected[i].to_string()
    ));
}

#[test]
fn test_tabulate_boolean() {
    let bools: ArrayMtPerS<B> = ArrayMtPerS::tabulate(
        |i| if i % 2 == 0 { B::True } else { B::False }, 
        6
    );
    assert_eq!(bools.length(), 6);
    
    // Expected: [True, False, True, False, True, False]
    let expected = [B::True, B::False, B::True, B::False, B::True, B::False];
    assert!(ArraySeqSetEq(
        bools.length(),
        |i| *bools.nth(i),
        expected.len(),
        |i| expected[i]
    ));
}

#[test]
fn test_tabulate_squares() {
    let squares: ArrayMtPerS<N> = ArrayMtPerS::tabulate(|i| i * i, 5);
    assert_eq!(squares.length(), 5);
    
    // Expected: [0, 1, 4, 9, 16]
    let expected = [0, 1, 4, 9, 16];
    assert!(ArraySeqSetEq(
        squares.length(),
        |i| *squares.nth(i),
        expected.len(),
        |i| expected[i]
    ));
}

#[test]
fn test_tabulate_large() {
    let large: ArrayMtPerS<N> = ArrayMtPerS::tabulate(|i| i + 100, 1000);
    assert_eq!(large.length(), 1000);
    assert_eq!(*large.nth(0), 100);
    assert_eq!(*large.nth(999), 1099);
    
    // Check a few random elements
    assert_eq!(*large.nth(500), 600);
    assert_eq!(*large.nth(123), 223);
}

}
