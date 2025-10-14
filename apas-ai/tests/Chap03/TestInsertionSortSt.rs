//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap03::InsertionSortSt::InsertionSortSt::InsertionSortStTrait;

fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {
    0i32.insSort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn insertion_sort_handles_empty() {
    let mut data: Vec<i32> = Vec::new();
    0i32.insSort(&mut data);
    assert!(data.is_empty());
}

#[test]
fn insertion_sort_single_element() { sort_and_assert(vec![42], &[42]); }

#[test]
fn insertion_sort_already_sorted() { sort_and_assert(vec![1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]); }

#[test]
fn insertion_sort_reverse_order() { sort_and_assert(vec![5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]); }

#[test]
fn insertion_sort_with_duplicates() { sort_and_assert(vec![3, 1, 2, 3, 1], &[1, 1, 2, 3, 3]); }

#[test]
fn insertion_sort_random_slice() {
    let mut data = vec![10, -1, 7, 3, 3, 9, 0, -5];
    let mut expected = data.clone();
    expected.sort();
    0i32.insSort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn insertion_sort_large_input_stress_test() {
    // Generate a large vector with 10,000+ elements
    let mut data: Vec<i32> = (0..10_000).rev().collect(); // Reverse sorted - worst case
    let mut expected = data.clone();
    expected.sort();

    0i32.insSort(&mut data);
    assert_eq!(data, expected);
    assert_eq!(data.len(), 10_000);

    // Verify it's actually sorted
    for i in 1..data.len() {
        assert!(data[i - 1] <= data[i], "Array not properly sorted at index {i}");
    }
}
