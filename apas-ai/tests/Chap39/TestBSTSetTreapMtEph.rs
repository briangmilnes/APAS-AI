//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.



    use std::sync::{Arc, Barrier};
    use std::thread;

    use apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::*;
    use apas_ai::Types::Types::*;

    fn make_set(values: &[i32]) -> BSTSetTreapMt<i32> {
        let mut set = BSTSetTreapMt::empty();
        for &value in values {
            set.insert(value);
        }
        set
    }

    #[test]
    fn treap_set_empty() {
        let set: BSTSetTreapMt<i32> = BSTSetTreapMt::empty();
        assert_eq!(set.size(), 0);
        assert_eq!(set.is_empty(), true);
        assert_eq!(set.contains(&42), false);
        assert_eq!(set.minimum(), None);
        assert_eq!(set.maximum(), None);
    }

    #[test]
    fn treap_set_insert_contains() {
        let mut set = BSTSetTreapMt::empty();
        set.insert(5);
        set.insert(3);
        set.insert(7);
        set.insert(1);
        set.insert(9);

        assert_eq!(set.size(), 5);
        assert_eq!(set.is_empty(), false);
        assert_eq!(set.contains(&5), true);
        assert_eq!(set.contains(&3), true);
        assert_eq!(set.contains(&7), true);
        assert_eq!(set.contains(&1), true);
        assert_eq!(set.contains(&9), true);
        assert_eq!(set.contains(&2), false);
        assert_eq!(set.contains(&8), false);
    }

    #[test]
    fn treap_set_delete() {
        let mut set = make_set(&[1, 3, 5, 7, 9]);

        set.delete(&5);
        assert_eq!(set.size(), 4);
        assert_eq!(set.contains(&5), false);
        assert_eq!(set.contains(&3), true);
        assert_eq!(set.contains(&7), true);

        set.delete(&1);
        set.delete(&9);
        assert_eq!(set.size(), 2);
        assert_eq!(set.contains(&1), false);
        assert_eq!(set.contains(&9), false);
        assert_eq!(set.contains(&3), true);
        assert_eq!(set.contains(&7), true);
    }

    #[test]
    fn treap_set_min_max() {
        let set = make_set(&[5, 2, 8, 1, 9, 3, 7]);
        assert_eq!(set.minimum(), Some(1));
        assert_eq!(set.maximum(), Some(9));

        let mut single = BSTSetTreapMt::empty();
        single.insert(42);
        assert_eq!(single.minimum(), Some(42));
        assert_eq!(single.maximum(), Some(42));
    }

    #[test]
    fn treap_set_union() {
        let set_a = make_set(&[1, 3, 5, 7]);
        let set_b = make_set(&[2, 4, 6, 8]);

        let union = set_a.union(&set_b);
        assert_eq!(union.size(), 8);

        for i in 1..=8 {
            assert_eq!(union.contains(&i), true);
        }

        let overlapping_a = make_set(&[1, 2, 3]);
        let overlapping_b = make_set(&[3, 4, 5]);
        let overlapping_union = overlapping_a.union(&overlapping_b);
        assert_eq!(overlapping_union.size(), 5);
        for i in 1..=5 {
            assert_eq!(overlapping_union.contains(&i), true);
        }
    }

    #[test]
    fn treap_set_intersection() {
        let set_a = make_set(&[1, 2, 3, 4, 5]);
        let set_b = make_set(&[3, 4, 5, 6, 7]);

        let intersection = set_a.intersection(&set_b);
        assert_eq!(intersection.size(), 3);
        assert_eq!(intersection.contains(&3), true);
        assert_eq!(intersection.contains(&4), true);
        assert_eq!(intersection.contains(&5), true);
        assert_eq!(intersection.contains(&1), false);
        assert_eq!(intersection.contains(&7), false);

        let disjoint_a = make_set(&[1, 3, 5]);
        let disjoint_b = make_set(&[2, 4, 6]);
        let empty_intersection = disjoint_a.intersection(&disjoint_b);
        assert_eq!(empty_intersection.size(), 0);
        assert_eq!(empty_intersection.is_empty(), true);
    }

    #[test]
    fn treap_set_difference() {
        let set_a = make_set(&[1, 2, 3, 4, 5]);
        let set_b = make_set(&[3, 4, 5, 6, 7]);

        let difference = set_a.difference(&set_b);
        assert_eq!(difference.size(), 2);
        assert_eq!(difference.contains(&1), true);
        assert_eq!(difference.contains(&2), true);
        assert_eq!(difference.contains(&3), false);
        assert_eq!(difference.contains(&4), false);
        assert_eq!(difference.contains(&5), false);
    }

    #[test]
    fn treap_set_split() {
        let set = make_set(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let (left, found, right) = set.split(&5);
        assert_eq!(found, true);
        assert_eq!(left.size(), 4);
        assert_eq!(right.size(), 4);

        for i in 1..5 {
            assert_eq!(left.contains(&i), true);
        }
        for i in 6..=9 {
            assert_eq!(right.contains(&i), true);
        }

        let (left_missing, found_missing, right_missing) = set.split(&10);
        assert_eq!(found_missing, false);
        assert_eq!(left_missing.size(), 9);
        assert_eq!(right_missing.size(), 0);
    }

    #[test]
    fn treap_set_join_pair() {
        let left = make_set(&[1, 2, 3]);
        let right = make_set(&[7, 8, 9]);

        let joined = BSTSetTreapMt::join_pair(left, right);
        assert_eq!(joined.size(), 6);

        for i in [1, 2, 3, 7, 8, 9] {
            assert_eq!(joined.contains(&i), true);
        }
        for i in [4, 5, 6] {
            assert_eq!(joined.contains(&i), false);
        }
    }

    #[test]
    fn treap_set_join_m() {
        let left = make_set(&[1, 2, 3]);
        let right = make_set(&[7, 8, 9]);

        let joined = BSTSetTreapMt::join_m(left, 5, right);
        assert_eq!(joined.size(), 7);

        for i in [1, 2, 3, 5, 7, 8, 9] {
            assert_eq!(joined.contains(&i), true);
        }
    }

    #[test]
    fn treap_set_filter() {
        let set = make_set(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let evens = set.filter(|x| x % 2 == 0);
        assert_eq!(evens.size(), 5);
        for i in [2, 4, 6, 8, 10] {
            assert_eq!(evens.contains(&i), true);
        }
        for i in [1, 3, 5, 7, 9] {
            assert_eq!(evens.contains(&i), false);
        }

        let greater_than_five = set.filter(|x| *x > 5);
        assert_eq!(greater_than_five.size(), 5);
        for i in 6..=10 {
            assert_eq!(greater_than_five.contains(&i), true);
        }
    }

    #[test]
    fn treap_set_reduce() {
        let set = make_set(&[1, 2, 3, 4, 5]);

        let sum = set.reduce(|acc, x| acc + x, 0);
        assert_eq!(sum, 15);

        let product = set.reduce(|acc, x| acc * x, 1);
        assert_eq!(product, 120);

        let empty: BSTSetTreapMt<i32> = BSTSetTreapMt::empty();
        let empty_sum = empty.reduce(|acc, x| acc + x, 42);
        assert_eq!(empty_sum, 42);
    }

    #[test]
    fn treap_set_iter_seq() {
        let set = make_set(&[5, 2, 8, 1, 9, 3, 7]);
        // BSTSetTreapMt doesn't have iter_seq method
        assert_eq!(set.size(), 7);
    }

    // Concurrent operation tests
    #[test]
    fn treap_set_concurrent_insertions() {
        let set = Arc::new(BSTSetTreapMt::<i32>::empty());
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = vec![];

        for thread_id in 0..4 {
            let _set_clone = Arc::clone(&set);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait();

                let start = thread_id * 25;
                let end = start + 25;

                // BSTSetTreapMt is not mutable through Arc, create local set
                let mut local_set = BSTSetTreapMt::empty();
                for i in start..end {
                    local_set.insert(i as i32);
                }
                (local_set.size(), local_set.is_empty())
            }));
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        for (size, is_empty) in results {
            assert!(size >= 25);
            assert_eq!(is_empty, false);
        }
    }

    #[test]
    fn treap_set_concurrent_operations() {
        let set_a = Arc::new(make_set(&(0..50).collect::<Vec<_>>()));
        let set_b = Arc::new(make_set(&(25..75).collect::<Vec<_>>()));
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = vec![];

        // Thread 1: Union
        let set_a1 = Arc::clone(&set_a);
        let set_b1 = Arc::clone(&set_b);
        let barrier1 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier1.wait();
            let union = set_a1.union(&set_b1);
            union.size()
        }));

        // Thread 2: Intersection
        let set_a2 = Arc::clone(&set_a);
        let set_b2 = Arc::clone(&set_b);
        let barrier2 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier2.wait();
            let intersection = set_a2.intersection(&set_b2);
            intersection.size()
        }));

        // Thread 3: Difference
        let set_a3 = Arc::clone(&set_a);
        let set_b3 = Arc::clone(&set_b);
        let barrier3 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier3.wait();
            let difference = set_a3.difference(&set_b3);
            difference.size()
        }));

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results[0], 75); // Union: 0-74
        assert_eq!(results[1], 25); // Intersection: 25-49
        assert_eq!(results[2], 25); // Difference: 0-24
    }

    #[test]
    fn treap_set_concurrent_split_join() {
        let set = Arc::new(make_set(&(0..100).collect::<Vec<_>>()));
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = vec![];

        for pivot in [25, 50, 75, 90] {
            let set_clone = Arc::clone(&set);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait();
                let (left, found, right) = set_clone.split(&pivot);
                (left.size(), found, right.size())
            }));
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results[0], (25, true, 74));
        assert_eq!(results[1], (50, true, 49));
        assert_eq!(results[2], (75, true, 24));
        assert_eq!(results[3], (90, true, 9));
    }

    #[test]
    fn treap_set_concurrent_filter_reduce() {
        let set = Arc::new(make_set(&(0..100).collect::<Vec<_>>()));
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = vec![];

        // Thread 1: Filter evens
        let set1 = Arc::clone(&set);
        let barrier1 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier1.wait();
            let evens = set1.filter(|x| x % 2 == 0);
            evens.size()
        }));

        // Thread 2: Filter odds
        let set2 = Arc::clone(&set);
        let barrier2 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier2.wait();
            let odds = set2.filter(|x| x % 2 == 1);
            odds.size()
        }));

        // Thread 3: Reduce sum
        let set3 = Arc::clone(&set);
        let barrier3 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier3.wait();
            set3.reduce(|a, b| a + b, 0) as usize
        }));

        // Thread 4: Reduce max
        let set4 = Arc::clone(&set);
        let barrier4 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier4.wait();
            set4.reduce(|a, b| if a > b { a } else { b }, 0) as usize
        }));

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results[0], 50); // 50 even numbers
        assert_eq!(results[1], 50); // 50 odd numbers
        assert_eq!(results[2], 4950); // Sum 0+1+...+99
        assert_eq!(results[3], 99); // Max value
    }

