//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci (multi-threaded).
//! Implements Example 11.10 using the project Parallel Pair abstraction.

pub mod FibonacciMt {

    use crate::ParaPair;
    use crate::Types::Types::*;

    pub struct FibonacciMt;

    pub trait FibonacciMtTrait {
        /// APAS: Work Θ(φⁿ), Span Θ(n)
        /// claude-4-sonet: Work Θ(φⁿ), Span Θ(n), Parallelism Θ(φⁿ/n) - parallel binary recursion via ParaPair!
        /// where φ = (1+√5)/2 ≈ 1.618 (golden ratio)
        fn fib(n: N) -> N;
    }

    impl FibonacciMt {
        /// Parallel Fibonacci using ParaPair! for symmetric binary parallelism.
        ///
        /// APAS: Work Θ(φⁿ), Span Θ(n)
        /// claude-4-sonet: Work Θ(φⁿ), Span Θ(n), Parallelism Θ(φⁿ/n) - parallel binary recursion via ParaPair!
        /// where φ = (1+√5)/2 ≈ 1.618 (golden ratio)
        ///
        /// Note: Exponential work makes this impractical for large n. This demonstrates
        /// parallel recursion patterns; real implementations use memoization or iteration.
        pub fn fib(n: N) -> N {
            if n <= 1 {
                n
            } else {
                let Pair(left, right) = ParaPair!(move || FibonacciMt::fib(n - 1), move || FibonacciMt::fib(n - 2));
                left + right
            }
        }
    }

    impl FibonacciMtTrait for FibonacciMt {
        fn fib(n: N) -> N { FibonacciMt::fib(n) }
    }
}
