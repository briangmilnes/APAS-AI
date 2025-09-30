//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqMtEphSLit;
use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
use apas_ai::Chap28::MaxContigSubSumOptMtEph::MaxContigSubSumOptMtEph::MaxContigSubSumOptMtTrait;

#[test]
fn test_empty() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphSLit![];
    let result = ArraySeqMtEphS::max_contig_sub_sum_opt_mt(&a);
    assert_eq!(result, None);
}

#[test]
fn test_single_positive() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphSLit![5];
    let result = ArraySeqMtEphS::max_contig_sub_sum_opt_mt(&a);
    assert_eq!(result, Some(5));
}

#[test]
fn test_example_from_book() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphSLit![1, -2, 0, 3, -1, 0, 2, -3];
    let result = ArraySeqMtEphS::max_contig_sub_sum_opt_mt(&a);
    assert_eq!(result, Some(4));
}

#[test]
fn test_all_negative() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphSLit![-5, -2, -8, -1];
    let result = ArraySeqMtEphS::max_contig_sub_sum_opt_mt(&a);
    assert_eq!(result, Some(-1));
}

#[test]
fn test_all_positive() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphSLit![1, 2, 3, 4];
    let result = ArraySeqMtEphS::max_contig_sub_sum_opt_mt(&a);
    assert_eq!(result, Some(10));
}

#[test]
fn test_larger_example() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphSLit![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = ArraySeqMtEphS::max_contig_sub_sum_opt_mt(&a);
    assert_eq!(result, Some(6));
}
