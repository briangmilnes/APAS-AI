//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Probability.

use apas_ai::Chap50::Probability::Probability::*;

#[test]
fn test_new() {
    let p = Probability::new(0.5);
    assert_eq!(p.value(), 0.5);
}

#[test]
fn test_zero() {
    let p = Probability::zero();
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_infinity() {
    let p = Probability::infinity();
    assert!(p.value().is_infinite());
}

#[test]
fn test_add() {
    let p1 = Probability::new(0.3);
    let p2 = Probability::new(0.2);
    let sum = p1 + p2;
    assert!((sum.value() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_sub() {
    let p1 = Probability::new(0.7);
    let p2 = Probability::new(0.3);
    let diff = p1 - p2;
    assert!((diff.value() - 0.4).abs() < f64::EPSILON);
}

#[test]
fn test_mul() {
    let p1 = Probability::new(0.5);
    let p2 = Probability::new(0.4);
    let prod = p1 * p2;
    assert!((prod.value() - 0.2).abs() < f64::EPSILON);
}

#[test]
fn test_div() {
    let p1 = Probability::new(0.6);
    let p2 = Probability::new(0.3);
    let quot = p1 / p2;
    assert!((quot.value() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn test_eq() {
    let p1 = Probability::new(0.5);
    let p2 = Probability::new(0.5);
    let p3 = Probability::new(0.3);
    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
}

#[test]
fn test_ord() {
    let p1 = Probability::new(0.3);
    let p2 = Probability::new(0.5);
    assert!(p1 < p2);
    assert!(p2 > p1);
}

#[test]
fn test_clone() {
    let p1 = Probability::new(0.7);
    let p2 = p1.clone();
    assert_eq!(p1, p2);
}

#[test]
fn test_display() {
    let p = Probability::new(0.75);
    let s = format!("{}", p);
    assert!(s.contains("0.75"));
}

#[test]
fn test_debug() {
    let p = Probability::new(0.5);
    let s = format!("{:?}", p);
    assert!(s.len() > 0);
}

#[test]
fn test_zero_operations() {
    let p = Probability::zero();
    let p2 = Probability::new(0.5);
    let sum = p + p2;
    assert_eq!(sum.value(), 0.5);
}

#[test]
fn test_infinity_operations() {
    let p = Probability::infinity();
    let p2 = Probability::new(0.5);
    let sum = p + p2;
    assert!(sum.value().is_infinite());
}
