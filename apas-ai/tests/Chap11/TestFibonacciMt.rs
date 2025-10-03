//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap11::FibonacciMt::FibonacciMt::*;
use apas_ai::Types::Types::*;

#[test]
fn fib_base_cases() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
}

#[test]
fn fib_small_values() {
    assert_eq!(fib(5), 5);
    assert_eq!(fib(6), 8);
    assert_eq!(fib(7), 13);
}

#[test]
fn fib_moderate_values() {
    assert_eq!(fib(10), 55);
    assert_eq!(fib(12), 144);
    assert_eq!(fib(13), 233);
}

#[test]
fn trait_and_inherent_agree() {
    for n in 0..=10 {
        assert_eq!(fib(n), fib(n)); // Both are now the same free function
    }
}
