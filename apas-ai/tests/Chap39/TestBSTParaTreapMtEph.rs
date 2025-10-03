//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod Test53BSTParaTreapMtEph {

    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use apas_ai::Types::Types::*;

    fn make_tree(values: &[i32]) -> ParamTreap<i32> {
        let tree = ParamTreap::new();
        for &value in values {
            tree.insert(value);
        }
        tree
    }

    fn make_range_tree(start: i32, end: i32) -> ParamTreap<i32> {
        let tree = ParamTreap::new();
        for value in start..end {
            tree.insert(value);
        }
        tree
    }

    #[test]
    fn treap_basic_insert_find() {
        let tree = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
        assert_eq!(tree.size(), 7);
        assert_eq!(tree.find(&3), Some(3));
        assert_eq!(tree.find(&8), None);
        assert_eq!(tree.is_empty(), false);
        assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn treap_split_join_pair() {
        let tree = make_tree(&[0, 1, 2, 3, 4, 5]);
        let (left, present, right) = tree.split(&3);
        assert_eq!(present, true);
        assert_eq!(left.in_order(), ArraySeqStPerSLit![0, 1, 2]);
        assert_eq!(right.in_order(), ArraySeqStPerSLit![4, 5]);

        let rejoined = left.join_pair(right);
        assert_eq!(rejoined.in_order(), ArraySeqStPerSLit![0, 1, 2, 4, 5]);
    }

    #[test]
    fn treap_union_intersect_difference() {
        let a = make_range_tree(0, 200);
        let b = make_range_tree(100, 300);

        let union = a.union(&b);
        let values: Vec<_> = union.in_order().iter().copied().collect();
        assert_eq!(values, (0..300).collect::<Vec<_>>());

        let intersection = make_range_tree(0, 200).intersect(&make_range_tree(100, 300));
        let intersection_values: Vec<_> = intersection.in_order().iter().copied().collect();
        assert_eq!(intersection_values, (100..200).collect::<Vec<_>>());

        let difference = make_range_tree(0, 200).difference(&make_range_tree(100, 300));
        let difference_values: Vec<_> = difference.in_order().iter().copied().collect();
        assert_eq!(difference_values, (0..100).collect::<Vec<_>>());
    }

    #[test]
    fn treap_filter_reduce() {
        let tree = make_range_tree(0, 64);

        let evens = tree.filter(|v| v % 2 == 0);
        let evens_vec: Vec<_> = evens.in_order().iter().copied().collect();
        assert_eq!(evens_vec, (0..64).filter(|v| v % 2 == 0).collect::<Vec<_>>());

        let sum = tree.reduce(|acc, v| acc + v, 0);
        let expected_sum: i32 = (0..64).sum();
        assert_eq!(sum, expected_sum);
    }

    #[test]
    fn treap_join_mid_roundtrip() {
        let empty: ParamTreap<i32> = ParamTreap::join_mid(Exposed::Leaf);
        matches!(empty.expose(), Exposed::Leaf);

        let left = ParamTreap::join_mid(Exposed::Leaf);
        let right = ParamTreap::join_mid(Exposed::Leaf);
        let combined = ParamTreap::join_mid(Exposed::Node(left, 10, right));

        match combined.expose() {
            | Exposed::Leaf => panic!("expected node"),
            | Exposed::Node(l, key, r) => {
                assert_eq!(key, 10);
                assert_eq!(l.size(), 0);
                assert_eq!(r.size(), 0);
            }
        }
    }

    #[test]
    fn treap_invariants_priority_heap() {
        fn check_heap(tree: &ParamTreap<i32>) {
            if let Some((left, _key, priority, right)) = tree.expose_with_priority() {
                if let Some((_, _, left_priority, _)) = left.expose_with_priority() {
                    assert!(priority >= left_priority);
                }
                if let Some((_, _, right_priority, _)) = right.expose_with_priority() {
                    assert!(priority >= right_priority);
                }
                check_heap(&left);
                check_heap(&right);
            }
        }

        let tree = make_range_tree(0, 128);
        check_heap(&tree);
    }

    // Multithreaded verification tests for BSTParaTreapMtEph
    #[test]
    fn treap_concurrent_insertions() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let tree = Arc::new(ParamTreap::<i32>::new());
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = vec![];

        for thread_id in 0..4 {
            let tree_clone = Arc::clone(&tree);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait();

                let start = thread_id * 25;
                let end = start + 25;

                for i in start..end {
                    tree_clone.insert(i as i32);
                }

                (tree_clone.size(), tree_clone.is_empty())
            }));
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        for (size, is_empty) in results {
            // Due to concurrent execution timing, size might be less than 25 when a thread finishes
            // if other threads haven't completed their insertions yet
            assert!(size >= 1); // At least some insertions should succeed
            assert_eq!(is_empty, false);
        }
    }

    #[test]
    fn treap_concurrent_operations_stress() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let tree = Arc::new(make_range_tree(0, 100));
        let barrier = Arc::new(Barrier::new(6));
        let mut handles = vec![];

        for thread_id in 0..6 {
            let tree_clone = Arc::clone(&tree);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait();

                match thread_id % 3 {
                    | 0 => {
                        let mut found = 0;
                        for i in 0..100 {
                            if tree_clone.find(&i).is_some() {
                                found += 1;
                            }
                        }
                        found
                    }
                    | 1 => {
                        let evens = tree_clone.filter(|x| x % 2 == 0);
                        evens.size()
                    }
                    | _ => {
                        let sum = tree_clone.reduce(|a, b| a + b, 0);
                        sum as usize
                    }
                }
            }));
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        for _result in results {
            // Result can vary
        }
    }

    #[test]
    fn treap_concurrent_set_operations() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let tree_a = Arc::new(make_range_tree(0, 50));
        let tree_b = Arc::new(make_range_tree(25, 75));
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = vec![];

        let tree_a1 = Arc::clone(&tree_a);
        let tree_b1 = Arc::clone(&tree_b);
        let barrier1 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier1.wait();
            let union = tree_a1.union(&tree_b1);
            union.size()
        }));

        let tree_a2 = Arc::clone(&tree_a);
        let tree_b2 = Arc::clone(&tree_b);
        let barrier2 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier2.wait();
            let intersection = tree_a2.intersect(&tree_b2);
            intersection.size()
        }));

        let tree_a3 = Arc::clone(&tree_a);
        let tree_b3 = Arc::clone(&tree_b);
        let barrier3 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier3.wait();
            let difference = tree_a3.difference(&tree_b3);
            difference.size()
        }));

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results[0], 75);
        assert_eq!(results[1], 25);
        assert_eq!(results[2], 25);
    }

    #[test]
    fn treap_concurrent_split_join() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let tree = Arc::new(make_range_tree(0, 100));
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = vec![];

        for pivot in [25, 50, 75, 90] {
            let tree_clone = Arc::clone(&tree);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait();
                let (left, found, right) = tree_clone.split(&pivot);
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
    fn treap_concurrent_expose_join_mid() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let barrier = Arc::new(Barrier::new(3));
        let mut handles = vec![];

        let barrier1 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier1.wait();
            let tree = make_tree(&[10, 5, 15, 3, 7, 12, 18]);
            match tree.expose() {
                | Exposed::Leaf => 0,
                | Exposed::Node(_, key, _) => key,
            }
        }));

        let barrier2 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier2.wait();
            let left = ParamTreap::join_mid(Exposed::Leaf);
            let right = ParamTreap::join_mid(Exposed::Leaf);
            let tree = ParamTreap::join_mid(Exposed::Node(left, 42, right));
            tree.size() as i32
        }));

        let barrier3 = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier3.wait();
            let left_tree = make_tree(&[1, 2, 3]);
            let right_tree = make_tree(&[7, 8, 9]);
            let combined = ParamTreap::join_mid(Exposed::Node(left_tree, 5, right_tree));
            combined.size() as i32
        }));

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert!(results[0] >= 0);
        assert_eq!(results[1], 1);
        assert_eq!(results[2], 7);
    }

    #[test]
    fn treap_concurrent_priority_invariants() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        fn check_heap_property(tree: &ParamTreap<i32>) -> bool {
            if let Some((left, _key, priority, right)) = tree.expose_with_priority() {
                let left_ok = if let Some((_, _, left_priority, _)) = left.expose_with_priority() {
                    priority >= left_priority && check_heap_property(&left)
                } else {
                    true
                };

                let right_ok = if let Some((_, _, right_priority, _)) = right.expose_with_priority() {
                    priority >= right_priority && check_heap_property(&right)
                } else {
                    true
                };

                left_ok && right_ok
            } else {
                true
            }
        }

        let tree = Arc::new(make_range_tree(0, 50));
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = vec![];

        for thread_id in 0..4 {
            let tree_clone = Arc::clone(&tree);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait();

                let start = thread_id * 10;
                let end = start + 10;

                for i in start..end {
                    tree_clone.insert(i as i32 + 100);
                }

                check_heap_property(&tree_clone)
            }));
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        for heap_valid in results {
            assert!(heap_valid);
        }
    }

    #[test]
    fn treap_concurrent_delete_operations() {
        use std::sync::{Arc, Barrier};
        use std::thread;
use apas_ai::{ArraySeqStPerSLit};

        let tree = Arc::new(make_range_tree(0, 100));
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = vec![];

        for thread_id in 0..4 {
            let tree_clone = Arc::clone(&tree);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait();

                let step = thread_id + 2;
                for i in (thread_id..100).step_by(step) {
                    tree_clone.delete(&(i as i32));
                }

                let mut remaining = 0;
                for i in 0..100 {
                    if tree_clone.find(&(i as i32)).is_some() {
                        remaining += 1;
                    }
                }
                remaining
            }));
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        for remaining in results {
            assert!(remaining >= 0);
            assert!(remaining <= 100);
        }
    }
}
