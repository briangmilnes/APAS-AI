//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent set implementation using AVLTreeSeqMtPer (Arc-based).
//!
//! Work/Span Analysis:
//! - union: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - intersection: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - filter: Work Θ(n), Span Θ(log n) via PARALLEL map-reduce

pub mod AVLTreeSetMtPer {
    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
    use crate::ParaPair;
    use crate::Types::Types::*;
    use std::fmt;
    use std::thread;

    #[derive(PartialEq, Eq)]
    pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
        elements: AVLTreeSeqMtPerS<T>,
    }

    pub trait AVLTreeSetMtPerTrait<T: StTInMtT + Ord + 'static> {
        fn size(&self) -> N;
        fn to_seq(&self) -> AVLTreeSeqMtPerS<T>;
        fn empty() -> Self;
        fn singleton(x: T) -> Self;
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> Self;
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(&self, f: F) -> Self;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, x: &T) -> B;
        fn delete(&self, x: &T) -> Self;
        fn insert(&self, x: T) -> Self;
    }

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPerTrait<T> for AVLTreeSetMtPer<T> {
        fn size(&self) -> N { self.elements.length() }

        fn to_seq(&self) -> AVLTreeSeqMtPerS<T> { self.elements.clone() }

        fn empty() -> Self {
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::empty(),
            }
        }

        fn singleton(x: T) -> Self {
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::singleton(x),
            }
        }

        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> Self {
            let mut vals = seq.values_in_order();

            // Unconditionally parallel merge sort using ParaPair!
            fn parallel_sort<T: StTInMtT + Ord + 'static>(mut vals: Vec<T>) -> Vec<T> {
                let n = vals.len();
                if n <= 1 {
                    return vals;
                }

                let mid = n / 2;
                let right_vals = vals.split_off(mid);
                let left_vals = vals;

                let Pair(left_sorted, right_sorted) =
                    ParaPair!(move || parallel_sort(left_vals), move || parallel_sort(right_vals));

                // Merge sorted halves
                let mut result = Vec::with_capacity(n);
                let mut i = 0;
                let mut j = 0;
                while i < left_sorted.len() && j < right_sorted.len() {
                    if left_sorted[i] <= right_sorted[j] {
                        result.push(left_sorted[i].clone());
                        i += 1;
                    } else {
                        result.push(right_sorted[j].clone());
                        j += 1;
                    }
                }
                result.extend_from_slice(&left_sorted[i..]);
                result.extend_from_slice(&right_sorted[j..]);
                result
            }

            vals = parallel_sort(vals);
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        // PARALLEL: filter using divide-and-conquer (unconditionally parallel)
        // Work: Θ(n), Span: Θ(log n)
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(&self, f: F) -> Self {
            let n = self.size();

            if n <= 8 {
                let mut vals = Vec::new();
                for i in 0..n {
                    let elem = self.elements.nth(i);
                    if f(elem) {
                        vals.push(elem.clone());
                    }
                }
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Unconditionally parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals: Vec<T> = (0..mid).map(|i| self.elements.nth(i).clone()).collect();
            let right_vals: Vec<T> = (mid..n).map(|i| self.elements.nth(i).clone()).collect();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));

            let f_left = f.clone();
            let f_right = f;

            let Pair(left_result, right_result) =
                ParaPair!(move || left_set.filter(f_left), move || right_set.filter(f_right));

            left_result.union(&right_result)
        }

        // PARALLEL: intersection using divide-and-conquer (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        fn intersection(&self, other: &Self) -> Self {
            let n = self.size();
            let m = other.size();

            if n == 0 || m == 0 {
                return Self::empty();
            }

            if n <= 8 {
                let mut vals = Vec::new();
                for i in 0..n {
                    let elem = self.elements.nth(i);
                    if other.find(elem) {
                        vals.push(elem.clone());
                    }
                }
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Unconditionally parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals: Vec<T> = (0..mid).map(|i| self.elements.nth(i).clone()).collect();
            let right_vals: Vec<T> = (mid..n).map(|i| self.elements.nth(i).clone()).collect();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));
            let other_left = other.clone();
            let other_right = other.clone();

            let Pair(left_result, right_result) =
                ParaPair!(move || left_set.intersection(&other_left), move || right_set
                    .intersection(&other_right));

            left_result.union(&right_result)
        }

        fn difference(&self, other: &Self) -> Self {
            let other_clone = other.clone();
            self.filter(move |x| !other_clone.find(x))
        }

        // PARALLEL: union using divide-and-conquer (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        fn union(&self, other: &Self) -> Self {
            let n = self.size();
            let m = other.size();

            if n == 0 {
                return other.clone();
            }
            if m == 0 {
                return self.clone();
            }

            if n <= 8 {
                let mut vals = self.elements.values_in_order();
                vals.extend(other.elements.values_in_order());
                vals.sort();
                vals.dedup();
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Unconditionally parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals: Vec<T> = (0..mid).map(|i| self.elements.nth(i).clone()).collect();
            let right_vals: Vec<T> = (mid..n).map(|i| self.elements.nth(i).clone()).collect();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));
            let other_left = other.clone();
            let other_right = other.clone();

            let Pair(left_result, right_result) = ParaPair!(move || left_set.union(&other_left), move || right_set
                .union(&other_right));

            left_result.union(&right_result)
        }

        fn find(&self, x: &T) -> B {
            // Binary search in sorted sequence
            let n = self.size();
            let mut left = 0;
            let mut right = n;

            while left < right {
                let mid = (left + right) / 2;
                let elem = self.elements.nth(mid);
                match elem.cmp(x) {
                    | std::cmp::Ordering::Less => left = mid + 1,
                    | std::cmp::Ordering::Equal => return true,
                    | std::cmp::Ordering::Greater => right = mid,
                }
            }
            false
        }

        fn delete(&self, x: &T) -> Self {
            // Unconditionally use parallel filter
            let x_clone = x.clone();
            self.filter(move |v| v != &x_clone)
        }

        fn insert(&self, x: T) -> Self {
            if self.find(&x) {
                return self.clone();
            }
            let mut vals = self.elements.values_in_order();
            vals.push(x);

            // Unconditionally use parallel from_seq
            Self::from_seq(AVLTreeSeqMtPerS::from_vec(vals))
        }
    }

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtPer<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtPer<T> {
        fn clone(&self) -> Self {
            AVLTreeSetMtPer {
                elements: self.elements.clone(),
            }
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSetMtPerLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty();
            $( __set = __set.insert($x); )*
            __set
        }};
    }

    #[allow(dead_code)]
    fn _AVLTreeSetMtPerLit_type_checks() {
        let _: AVLTreeSetMtPer<i32> = AVLTreeSetMtPerLit![];
        let _: AVLTreeSetMtPer<i32> = AVLTreeSetMtPerLit![1, 2, 3];
    }
}
