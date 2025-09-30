//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap11::FibonacciMt::FibonacciMt::{FibonacciMt, FibonacciMtTrait};
use apas_ai::Types::Types::*;

#[test]
fn fib_base_cases() {
    assert_eq!(FibonacciMt::fib(0), 0);
    assert_eq!(FibonacciMt::fib(1), 1);
}

#[test]
fn fib_small_values() {
    assert_eq!(FibonacciMt::fib(5), 5);
    assert_eq!(FibonacciMt::fib(6), 8);
    assert_eq!(FibonacciMt::fib(7), 13);
}

#[test]
fn fib_moderate_values() {
    assert_eq!(FibonacciMt::fib(10), 55);
    assert_eq!(FibonacciMt::fib(15), 610);
    assert_eq!(FibonacciMt::fib(20), 6765);
}

#[test]
fn trait_and_inherent_agree() {
    for n in 0..=10 {
        assert_eq!(FibonacciMt::fib(n), <FibonacciMt as FibonacciMtTrait>::fib(n));
    }
}
