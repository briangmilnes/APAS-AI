//! Chapter 11 — Parallel Fibonacci (multi-threaded).
//! Implements Example 11.10 using the project Parallel Pair abstraction.

pub mod FibonacciMt {
    use crate::{ParaPair, Types::Types::*};

    pub struct FibonacciMt;

    pub trait FibonacciMtTrait {
        fn fib(n: N) -> N;
    }

    impl FibonacciMt {
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
        fn fib(n: N) -> N {
            FibonacciMt::fib(n)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{FibonacciMt, FibonacciMtTrait};
        use crate::Types::Types::*;

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
        fn trait_and_inherent_agree() {
            for n in 0..=8 {
                assert_eq!(FibonacciMt::fib(n), <FibonacciMt as FibonacciMtTrait>::fib(n));
            }
        }
    }
}
