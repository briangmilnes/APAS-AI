//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.8: Brute Force Primality Test (isPrime).

pub mod Exercise21_8 {

use crate::Types::Types::*;
use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    pub trait Exercise21_8Trait {
        /// Helper function to check divisibility
        /// APAS: Work Θ(1), Span Θ(1)
        fn is_divisible(n: N, i: N) -> B;
        
        /// Exercise 21.8 / Algorithm 21.4 (Brute Force Primality Test)
        /// APAS: Work Θ(√n), Span Θ(lg n)
        fn is_prime(n: N) -> B;
    }

    /// Helper function to check divisibility
    pub fn is_divisible(n: N, i: N) -> B {
        if n % i == 0 {
            true
        } else {
            false
        }
    }

    /// Exercise 21.8 / Algorithm 21.4 (Brute Force Primality Test)
    /// isPrime n = |{ x in 1..=floor(sqrt(n)) : n mod i == 0 }| == 1
    ///
    /// Tests if a number is prime by checking divisibility up to sqrt(n).
    /// Only the divisor 1 should divide n for it to be prime.
    ///
    /// gpt-5-hard: Work: Θ(√n), Span: Θ(lg n)
    /// APAS: Work: Θ(√n), Span: Θ(lg n)
    pub fn is_prime(n: N) -> B {
        if n < 2 {
            return false;
        }
        let k: N = (n as f64).sqrt().floor() as N;
        let all: ArraySeqStPerS<B> =
            <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::tabulate(&|i| is_divisible(n, i + 1), k);
        let ones: ArraySeqStPerS<B> = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::filter(&all, &|x| *x);
        if ones.length() == 1 {
            true
        } else {
            false
        }
    }
}
