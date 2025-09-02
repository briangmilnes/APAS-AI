use apas_ai::Types::{N, B, O};

#[test]
fn test_boolean_eq_neq_and_ordering() {
    assert_eq!(B::True, B::True);
    assert_ne!(B::True, B::False);
    assert_ne!(B::False, B::True);
    assert_eq!(B::False, B::False);

    // Check ordering semantics on B (derived Ord): True < False?
    // The user wants True < False verified explicitly.
    assert!(B::True < B::False);
}

#[test]
fn test_ordering_on_n_naturals() {
    let a: N = 0;
    let b: N = 1;
    assert!(matches!(a.cmp(&b), O::Less));
    assert!(matches!(b.cmp(&a), O::Greater));
    assert!(matches!(a.cmp(&a), O::Equal));
}


