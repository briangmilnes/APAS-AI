//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 single-threaded persistent table implementation using ArraySeq as backing store.

pub mod TableStPer {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use std::cmp::Ordering;

    /// Single-threaded persistent table backed by ArraySeq
    #[derive(Debug, Clone, PartialEq)]
    pub struct TableStPer<K: StT + Ord, V: StT> {
        entries: ArraySeqStPerS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStPer<K, V>;

    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableStPerTrait<K: StT + Ord, V: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> Self;

        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(key: K, value: V) -> Self;

        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        fn domain(&self) -> ArraySetStEph<K>;

        /// APAS: Work Θ(|s| * W(f)), Span Θ(lg |s| + S(f))
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self;

        /// APAS: Work Θ(|a| * W(f)), Span Θ(lg |a| + S(f))
        fn map<F: Fn(&V) -> V>(&self, f: F) -> Self;

        /// APAS: Work Θ(|a| * W(f)), Span Θ(lg |a| + S(f))
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self;

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> Self;

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> Self;

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn difference(&self, other: &Self) -> Self;

        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn find(&self, key: &K) -> Option<V>;

        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn delete(&self, key: &K) -> Self;

        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn insert<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> Self;

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn restrict(&self, keys: &ArraySetStEph<K>) -> Self;

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn subtract(&self, keys: &ArraySetStEph<K>) -> Self;

        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        fn collect(&self) -> ArraySeqStPerS<Pair<K, V>>;
    }

    impl<K: StT + Ord, V: StT> TableStPerTrait<K, V> for TableStPer<K, V> {
        fn size(&self) -> N { self.entries.length() }

        fn empty() -> Self {
            TableStPer {
                entries: ArraySeqStPerS::empty(),
            }
        }

        fn singleton(key: K, value: V) -> Self {
            TableStPer {
                entries: ArraySeqStPerS::singleton(Pair(key, value)),
            }
        }

        fn domain(&self) -> ArraySetStEph<K> {
            let keys: Vec<K> = (0..self.entries.length())
                .map(|i| self.entries.nth(i).0.clone())
                .collect();
            ArraySetStEph::from_seq(crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(
                keys,
            ))
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self {
            let key_seq = keys.to_seq();
            let entries = ArraySeqStPerS::tabulate(
                &|i| {
                    let key = key_seq.nth(i);
                    let value = f(key);
                    Pair(key.clone(), value)
                },
                key_seq.length(),
            );

            TableStPer { entries }
        }

        fn map<F: Fn(&V) -> V>(&self, f: F) -> Self {
            let entries = ArraySeqStPerS::tabulate(
                &|i| {
                    let Pair(key, value) = self.entries.nth(i);
                    let new_value = f(value);
                    Pair(key.clone(), new_value)
                },
                self.entries.length(),
            );

            TableStPer { entries }
        }

        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self {
            let filtered = ArraySeqStPerS::filter(&self.entries, &|pair| f(&pair.0, &pair.1));
            TableStPer { entries: filtered }
        }

        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> Self {
            let mut result_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let Pair(key1, val1) = self.entries.nth(i);
                let Pair(key2, val2) = other.entries.nth(j);

                match key1.cmp(key2) {
                    | Ordering::Less => i += 1,
                    | Ordering::Greater => j += 1,
                    | Ordering::Equal => {
                        let combined_value = combine(val1, val2);
                        result_entries.push(Pair(key1.clone(), combined_value));
                        i += 1;
                        j += 1;
                    }
                }
            }

            TableStPer {
                entries: ArraySeqStPerS::from_vec(result_entries),
            }
        }

        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> Self {
            let intersection = self.intersection(other, &combine);
            let left_diff = self.difference(other);
            let right_diff = other.difference(self);

            // Combine all three parts
            let mut all_entries = Vec::new();

            // Add intersection entries
            for i in 0..intersection.entries.length() {
                all_entries.push(intersection.entries.nth(i).clone());
            }

            // Add left difference entries
            for i in 0..left_diff.entries.length() {
                all_entries.push(left_diff.entries.nth(i).clone());
            }

            // Add right difference entries
            for i in 0..right_diff.entries.length() {
                all_entries.push(right_diff.entries.nth(i).clone());
            }

            // Sort by key to maintain order
            all_entries.sort_by(|a, b| a.0.cmp(&b.0));

            TableStPer {
                entries: ArraySeqStPerS::from_vec(all_entries),
            }
        }

        fn difference(&self, other: &Self) -> Self {
            let mut result_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let Pair(key1, val1) = self.entries.nth(i);
                let Pair(key2, _) = other.entries.nth(j);

                match key1.cmp(key2) {
                    | Ordering::Less => {
                        result_entries.push(Pair(key1.clone(), val1.clone()));
                        i += 1;
                    }
                    | Ordering::Greater => j += 1,
                    | Ordering::Equal => {
                        i += 1;
                        j += 1;
                    }
                }
            }

            // Add remaining entries from self
            while i < self.entries.length() {
                let Pair(key, val) = self.entries.nth(i);
                result_entries.push(Pair(key.clone(), val.clone()));
                i += 1;
            }

            TableStPer {
                entries: ArraySeqStPerS::from_vec(result_entries),
            }
        }

        fn find(&self, key: &K) -> Option<V> {
            // Binary search on sorted entries
            let mut left = 0;
            let mut right = self.entries.length();

            while left < right {
                let mid = left + (right - left) / 2;
                let Pair(mid_key, mid_val) = self.entries.nth(mid);

                match key.cmp(mid_key) {
                    | Ordering::Less => right = mid,
                    | Ordering::Greater => left = mid + 1,
                    | Ordering::Equal => return Some(mid_val.clone()),
                }
            }

            None
        }

        fn delete(&self, key: &K) -> Self {
            let filtered = ArraySeqStPerS::filter(&self.entries, &|pair| &pair.0 != key);
            TableStPer { entries: filtered }
        }

        fn insert<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> Self {
            if let Some(existing_value) = self.find(&key) {
                // Key exists, combine values
                let combined_value = combine(&existing_value, &value);
                let updated = self.delete(&key);
                // Find insertion point
                let mut insert_pos = updated.entries.length();
                for i in 0..updated.entries.length() {
                    let Pair(k, _) = updated.entries.nth(i);
                    if key < *k {
                        insert_pos = i;
                        break;
                    }
                }

                let new_pair = Pair(key, combined_value);
                let entries = ArraySeqStPerS::tabulate(
                    &|i| {
                        if i < insert_pos {
                            updated.entries.nth(i).clone()
                        } else if i == insert_pos {
                            new_pair.clone()
                        } else {
                            updated.entries.nth(i - 1).clone()
                        }
                    },
                    updated.entries.length() + 1,
                );

                TableStPer { entries }
            } else {
                // Key doesn't exist, insert new entry
                // Find insertion point
                let mut insert_pos = self.entries.length();
                for i in 0..self.entries.length() {
                    let Pair(k, _) = self.entries.nth(i);
                    if key < *k {
                        insert_pos = i;
                        break;
                    }
                }

                let new_pair = Pair(key, value);
                let entries = ArraySeqStPerS::tabulate(
                    &|i| {
                        if i < insert_pos {
                            self.entries.nth(i).clone()
                        } else if i == insert_pos {
                            new_pair.clone()
                        } else {
                            self.entries.nth(i - 1).clone()
                        }
                    },
                    self.entries.length() + 1,
                );

                TableStPer { entries }
            }
        }

        fn restrict(&self, keys: &ArraySetStEph<K>) -> Self {
            let filtered = ArraySeqStPerS::filter(&self.entries, &|pair| keys.find(&pair.0));
            TableStPer { entries: filtered }
        }

        fn subtract(&self, keys: &ArraySetStEph<K>) -> Self {
            let filtered = ArraySeqStPerS::filter(&self.entries, &|pair| !keys.find(&pair.0));
            TableStPer { entries: filtered }
        }

        fn collect(&self) -> ArraySeqStPerS<Pair<K, V>> { self.entries.clone() }
    }

    /// Helper function for creating tables from sorted entries
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(entries: Vec<Pair<K, V>>) -> TableStPer<K, V> {
        TableStPer {
            entries: ArraySeqStPerS::from_vec(entries),
        }
    }

    /// Macro for creating table literals
    #[macro_export]
    macro_rules! TableStPerLit {
        () => {
            $crate::Chap42::TableStPer::TableStPer::TableStPer::empty()
        };
        ($($key:expr => $value:expr),+ $(,)?) => {{
            let mut entries = vec![$($crate::Types::Types::Pair($key, $value)),+];
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            $crate::Chap42::TableStPer::TableStPer::from_sorted_entries(entries)
        }};
    }

    pub use TableStPerLit;
}
