use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{Chap18::ArraySeqMtPer::ArraySeqMtPer::*, Types::Types::*};

pub mod MinEditDistMtPer {
    use super::*;

    /// Persistent multi-threaded minimum edit distance solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct MinEditDistMtPerS<T: MtVal> {
        source: ArraySeqMtPerS<T>,
        target: ArraySeqMtPerS<T>,
        memo: Arc<Mutex<HashMap<(usize, usize), usize>>>,
    }

    /// Trait for parallel minimum edit distance operations
    pub trait MinEditDistMtPerTrait<T: MtVal> {
        /// Create new minimum edit distance solver
        fn new() -> Self
        where
            T: Default;
        
        /// Create from source and target sequences
        fn from_sequences(source: ArraySeqMtPerS<T>, target: ArraySeqMtPerS<T>) -> Self;
        
        /// Compute minimum edit distance with parallel dynamic programming
        /// Claude Work: O(|S|*|T|) where |S|=source length, |T|=target length
        /// Claude Span: O(|S|+|T|) with parallelism O(min(|S|,|T|))
        fn min_edit_distance(&self) -> usize
        where
            T: Send + Sync + 'static;
            
        /// Get the source sequence
        fn source(&self) -> &ArraySeqMtPerS<T>;
        
        /// Get the target sequence
        fn target(&self) -> &ArraySeqMtPerS<T>;
        
        /// Get memoization table size
        fn memo_size(&self) -> usize;
    }

    impl<T: MtVal> MinEditDistMtPerS<T> {
        /// Internal parallel recursive minimum edit distance with shared memoization
        /// Claude Work: O(|S|*|T|) - each subproblem computed once across all threads
        /// Claude Span: O(|S|+|T|) - maximum recursion depth, parallelism O(min(|S|,|T|))
        fn min_edit_distance_rec(&self, i: usize, j: usize) -> usize
        where
            T: Send + Sync + 'static,
        {
            // Check memo first (thread-safe)
            {
                let memo_guard = self.memo.lock().unwrap();
                if let Some(&result) = memo_guard.get(&(i, j)) {
                    return result;
                }
            }

            let result = match (i, j) {
                (i, 0) => i,  // Base case: need i deletions
                (0, j) => j,  // Base case: need j insertions
                (i, j) => {
                    let source_char = self.source.nth(i - 1);
                    let target_char = self.target.nth(j - 1);
                    
                    if source_char == target_char {
                        // Characters match, no edit needed
                        self.min_edit_distance_rec(i - 1, j - 1)
                    } else {
                        // Parallel evaluation of both operations
                        let self_clone1 = self.clone();
                        let self_clone2 = self.clone();
                        
                        let handle1 = thread::spawn(move || {
                            self_clone1.min_edit_distance_rec(i - 1, j)
                        });
                        
                        let handle2 = thread::spawn(move || {
                            self_clone2.min_edit_distance_rec(i, j - 1)
                        });
                        
                        let delete_cost = handle1.join().unwrap();
                        let insert_cost = handle2.join().unwrap();
                        
                        1 + std::cmp::min(delete_cost, insert_cost)
                    }
                }
            };

            // Memoize result (thread-safe)
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.insert((i, j), result);
            }
            
            result
        }
    }

    impl<T: MtVal> MinEditDistMtPerTrait<T> for MinEditDistMtPerS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                source: ArraySeqMtPerS::new(0, T::default()),
                target: ArraySeqMtPerS::new(0, T::default()),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_sequences(source: ArraySeqMtPerS<T>, target: ArraySeqMtPerS<T>) -> Self {
            Self {
                source,
                target,
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn min_edit_distance(&self) -> usize
        where
            T: Send + Sync + 'static,
        {
            // Clear memo for fresh computation
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }
            
            let source_len = self.source.length();
            let target_len = self.target.length();
            
            self.min_edit_distance_rec(source_len, target_len)
        }

        fn source(&self) -> &ArraySeqMtPerS<T> {
            &self.source
        }

        fn target(&self) -> &ArraySeqMtPerS<T> {
            &self.target
        }

        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    impl<T: MtVal> PartialEq for MinEditDistMtPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.source == other.source && self.target == other.target
        }
    }

    impl<T: MtVal> Eq for MinEditDistMtPerS<T> {}

    impl<T: MtVal> Display for MinEditDistMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_size = {
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
            };
            write!(f, "MinEditDistMtPer(source: {}, target: {}, memo_entries: {})", 
                   self.source, self.target, memo_size)
        }
    }

    // Note: IntoIterator not implemented for ArraySeqMtPerS, so we don't provide it here

    #[allow(dead_code)]
    fn _MinEditDistMtPerLit_type_checks() {
        let source = ArraySeqMtPerS::from_vec(vec!['A', 'B', 'C', 'A', 'D', 'A']);
        let target = ArraySeqMtPerS::from_vec(vec!['A', 'B', 'A', 'D', 'C']);
        let _: MinEditDistMtPerS<char> = MinEditDistMtPerS::from_sequences(source, target);
    }
}

#[macro_export]
macro_rules! MinEditDistMtPerLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistMtPer::MinEditDistMtPer::MinEditDistMtPerS::from_sequences(
            $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$($s),*]),
            $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$($t),*])
        )
    };
    () => {
        $crate::Chap49::MinEditDistMtPer::MinEditDistMtPer::MinEditDistMtPerS::new()
    };
}

