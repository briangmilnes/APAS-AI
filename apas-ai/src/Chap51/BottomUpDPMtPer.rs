//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Bottom-Up Dynamic Programming - Persistent Multi-Threaded Implementation
//! 
//! This module implements the bottom-up approach to dynamic programming using
//! parallel diagonal pebbling for multi-threaded computation.

pub mod BottomUpDPMtPer {

use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use crate::Types::Types::*;
use std::cmp::{max, min};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};
use std::thread;

/// Persistent multi-threaded bottom-up DP structure (struct product record type)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BottomUpDPMtPerS {
    /// Input sequence S
    seq_s: ArraySeqMtPerS<char>,
    /// Input sequence T  
    seq_t: ArraySeqMtPerS<char>,
}

impl BottomUpDPMtPerS {
    /// Claude Work: O(1) - constant time initialization
    /// Claude Span: O(1) - constant time initialization
    pub fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> Self {
        BottomUpDPMtPerS {
            seq_s: s,
            seq_t: t,
        }
    }

    /// Compute minimum edit distance using parallel bottom-up diagonal pebbling
    /// Claude Work: O(|S|*|T|) where |S|=source length, |T|=target length
    /// Claude Span: O(|S|+|T|) with parallelism O(min(|S|,|T|))
    pub fn med_bottom_up_parallel(&self) -> usize {
        let s_len = self.seq_s.length();
        let t_len = self.seq_t.length();
        
        // Create shared DP table for parallel computation
        let table = Arc::new(Mutex::new(self.initialize_base_cases()));
        
        // Process diagonals with parallel computation within each diagonal
        for k in 1..=(s_len + t_len) {
            self.compute_diagonal_parallel(Arc::clone(&table), k);
        }
        
        // Extract result from bottom-right corner
        let final_table = table.lock().unwrap();
        final_table[s_len][t_len]
    }

    /// Initialize base cases for DP table
    /// Claude Work: O(|S|+|T|) - linear initialization
    /// Claude Span: O(|S|+|T|) - sequential initialization
    fn initialize_base_cases(&self) -> Vec<Vec<usize>> {
        let s_len = self.seq_s.length();
        let t_len = self.seq_t.length();
        
        // Initialize with zeros - using Vec for 2D table as Mt sequences lack method-call API
        let mut table = vec![vec![0usize; t_len + 1]; s_len + 1];
        
        // Set base cases: empty string transformations
        for i in 0..=s_len {
            table[i][0] = i;
        }
        for j in 0..=t_len {
            table[0][j] = j;
        }
        
        table
    }

    /// Compute one diagonal of the DP table in parallel
    /// Claude Work: O(min(|S|,|T|)) - diagonal length
    /// Claude Span: O(1) with parallel threads for diagonal elements
    fn compute_diagonal_parallel(
        &self, 
        table: Arc<Mutex<Vec<Vec<usize>>>>, 
        k: usize
    ) {
        let s_len = self.seq_s.length();
        let t_len = self.seq_t.length();
        
        let start = max(1, k.saturating_sub(t_len));
        let end = min(k, s_len);
        
        // Collect diagonal positions
        let positions: Vec<(usize, usize)> = (start..=end)
            .filter_map(|i| {
                let j = k - i;
                if j > 0 && j <= t_len {
                    Some((i, j))
                } else {
                    None
                }
            })
            .collect();
        
        // Process diagonal elements in parallel
        let handles: Vec<_> = positions
            .into_iter()
            .map(|(i, j)| {
                let table_clone = Arc::clone(&table);
                let seq_s_clone = self.seq_s.clone();
                let seq_t_clone = self.seq_t.clone();
                
                thread::spawn(move || {
                    let new_value = Self::compute_cell_value_static(
                        &seq_s_clone, &seq_t_clone, &table_clone, i, j
                    );
                    (i, j, new_value)
                })
            })
            .collect();
        
        // Collect results from all threads FIRST (without holding lock)
        let results: Vec<(usize, usize, usize)> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();
        
        // Then acquire lock once and write all results
        let mut table_guard = table.lock().unwrap();
        for (i, j, new_value) in results {
            table_guard[i][j] = new_value;
        }
    }

    /// Static method to compute value for a single DP table cell
    /// Claude Work: O(1) - constant time per cell
    /// Claude Span: O(1) - constant time per cell
    fn compute_cell_value_static(
        seq_s: &ArraySeqMtPerS<char>,
        seq_t: &ArraySeqMtPerS<char>,
        table: &Arc<Mutex<Vec<Vec<usize>>>>,
        i: usize,
        j: usize,
    ) -> usize {
        let s_char = *seq_s.nth(i - 1);
        let t_char = *seq_t.nth(j - 1);
        
        let table_guard = table.lock().unwrap();
        
        if s_char == t_char {
            // Characters match: take diagonal value
            table_guard[i - 1][j - 1]
        } else {
            // Characters don't match: take minimum of insert/delete + 1
            let delete_cost = table_guard[i - 1][j];
            let insert_cost = table_guard[i][j - 1];
            1 + min(delete_cost, insert_cost)
        }
    }

    /// Get the length of sequence S
    /// Claude Work: O(1) - constant time access
    /// Claude Span: O(1) - constant time access
    pub fn s_length(&self) -> usize {
        self.seq_s.length()
    }

    /// Get the length of sequence T
    /// Claude Work: O(1) - constant time access
    /// Claude Span: O(1) - constant time access
    pub fn t_length(&self) -> usize {
        self.seq_t.length()
    }

    /// Check if sequences are empty
    /// Claude Work: O(1) - constant time check
    /// Claude Span: O(1) - constant time check
    pub fn is_empty(&self) -> bool {
        self.seq_s.length() == 0usize && self.seq_t.length() == 0usize
    }
}

impl Default for BottomUpDPMtPerS {
    fn default() -> Self {
        let empty_s = ArraySeqMtPerS::new(0, ' ');
        let empty_t = ArraySeqMtPerS::new(0, ' ');
        Self::new(empty_s, empty_t)
    }
}

impl Display for BottomUpDPMtPerS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BottomUpDPMtPer(s_len={}, t_len={})", 
               self.s_length(), self.t_length())
    }
}

} // end mod BottomUpDPMtPer

