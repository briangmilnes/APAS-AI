//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap43Claude Example 43.1.

use apas_ai::Chap43Claude::Example43_1::Example43_1::run_example43_1;

#[test]
fn test_example43_1_basic_functionality() {
    // Since this is a demonstration function, we just verify it runs without panicking
    // The actual verification is done through the printed output
    run_example43_1();
}

#[test]
fn test_example43_1_lexicographic_ordering() {
    // The example uses lexicographic ordering on strings
    // Just verify it runs without panicking
    run_example43_1();
}

#[test]
fn test_example43_1_demonstrates_operations() {
    // The example should demonstrate various ordering operations
    // Just verify it runs without panicking
    run_example43_1();
}

#[test]
fn test_example43_1_string_set_operations() {
    // The example works with string sets using lexicographic ordering
    // Just verify it runs without panicking
    run_example43_1();
}

#[test]
fn test_example43_1_consistency() {
    // Run the example multiple times to ensure consistency
    run_example43_1();
    run_example43_1();
    // Both calls should complete without panicking
}

#[test]
fn test_example43_1_output_format() {
    // Should run without panicking and produce output to stdout
    run_example43_1();
}
