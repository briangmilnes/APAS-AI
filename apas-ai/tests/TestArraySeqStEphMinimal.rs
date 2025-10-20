//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Test demonstrating: Trait default implementations mean MINIMAL impl blocks!

use apas_ai::Chap18::ArraySeqStEphMinimal::*;
use apas_ai::Types::Types::*;

#[test]
fn test_all_methods_work_from_defaults() {
    // All these methods come from trait defaults!
    let seq: ArraySeqStEphMinimalS<i32> = ArraySeqStEphMinimalS::empty();
    assert_eq!(seq.length(), 0);

    let seq = ArraySeqStEphMinimalS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.nth(0), &42);

    let mut seq = ArraySeqStEphMinimalS::new(5, 10);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth(3), &10);

    let _ = seq.set(2, 99);
    assert_eq!(seq.nth(2), &99);

    let _ = seq.update(Pair(4, 88));
    assert_eq!(seq.nth(4), &88);

    let sum: i32 = seq.iter().sum();
    assert_eq!(sum, 10 + 10 + 99 + 10 + 88);
}

#[test]
fn test_from_vec_works() {
    let seq = ArraySeqStEphMinimalS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth(0), &1);
    assert_eq!(seq.nth(4), &5);
}

#[test]
fn test_can_override_defaults_if_needed() {
    // You can create a new type and override specific methods for optimization
    // while keeping all the other defaults!

    #[derive(Clone)]
    struct FastArraySeq<T: StT> {
        data: Box<[T]>,
        cached_len: usize, // Could cache length for O(1) access
    }

    impl<T: StT> ArraySeqStEphMinimalTrait<T> for FastArraySeq<T> {
        fn from_vec(elts: Vec<T>) -> Self {
            let len = elts.len();
            Self {
                data: elts.into_boxed_slice(),
                cached_len: len,
            }
        }

        fn data(&self) -> &[T] { &self.data }
        fn data_mut(&mut self) -> &mut [T] { &mut self.data }

        // OVERRIDE: Use cached length instead of trait default
        fn length(&self) -> N { self.cached_len }

        // All other methods (empty, singleton, new, nth, set, etc.)
        // still come from trait defaults!
    }

    let seq = FastArraySeq::singleton(100);
    assert_eq!(seq.length(), 1); // Uses our override
    assert_eq!(seq.nth(0), &100); // Uses trait default
}
