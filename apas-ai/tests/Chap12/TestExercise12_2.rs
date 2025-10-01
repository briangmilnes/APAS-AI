//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod TestExercise12_2 {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    use apas_ai::Chap12::Exercise12_2::Exercise12_2::{fetch_add_cas, FetchAddCasTrait};

    #[test]
    fn fetch_add_cas_returns_previous_value() {
        let value = AtomicUsize::new(10);
        assert_eq!(fetch_add_cas(&value, 5), 10);
        assert_eq!(value.load(Ordering::Relaxed), 15);
    }

    #[test]
    fn trait_impl_matches_free_function() {
        let value = AtomicUsize::new(3);
        let via_trait = value.fetch_add_cas(4);
        let via_free = fetch_add_cas(&value, 2);
        assert_eq!(via_trait, 3);
        assert_eq!(via_free, 7);
        assert_eq!(value.load(Ordering::Relaxed), 9);
    }

    #[test]
    fn fetch_add_cas_is_thread_safe() {
        let value = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for _ in 0..8 {
            let shared = Arc::clone(&value);
            handles.push(thread::spawn(move || {
                for _ in 0..1_000 {
                    shared.fetch_add_cas(1);
                }
            }));
        }

        for handle in handles {
            handle.join().expect("fetch_add_cas_is_thread_safe: worker panicked");
        }

        assert_eq!(value.load(Ordering::Relaxed), 8 * 1_000);
    }
}
