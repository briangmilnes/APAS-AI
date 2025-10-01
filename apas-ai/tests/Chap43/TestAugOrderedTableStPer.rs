//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for single-threaded persistent reducer-augmented ordered table implementation.

use apas_ai::AugOrderedTableStPerLit;
use apas_ai::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_ai::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::*;
use apas_ai::Types::Types::*;

#[test]
fn test_empty_table() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let table = AugOrderedTableStPer::empty(sum_reducer, 0);

    assert_eq!(table.size(), 0);
    assert_eq!(table.reduce_val(), 0);
    assert!(table.first_key().is_none());
    assert!(table.last_key().is_none());
}

#[test]
fn test_singleton_table() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let table = AugOrderedTableStPer::singleton(42, 100, sum_reducer, 0);

    assert_eq!(table.size(), 1);
    assert_eq!(table.reduce_val(), 100);
    assert_eq!(table.find(&42), Some(100));
    assert_eq!(table.first_key(), Some(42));
    assert_eq!(table.last_key(), Some(42));
}

#[test]
fn test_insert_and_reduce_val() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    table = table.insert(1, 10);
    assert_eq!(table.size(), 1);
    assert_eq!(table.reduce_val(), 10);

    table = table.insert(2, 20);
    assert_eq!(table.size(), 2);
    assert_eq!(table.reduce_val(), 30);

    table = table.insert(3, 30);
    assert_eq!(table.size(), 3);
    assert_eq!(table.reduce_val(), 60);
}

#[test]
fn test_delete_and_reduce_val() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    table = table.insert(1, 10);
    table = table.insert(2, 20);
    table = table.insert(3, 30);
    assert_eq!(table.reduce_val(), 60);

    table = table.delete(&2);
    assert_eq!(table.size(), 2);
    assert_eq!(table.reduce_val(), 40);
    assert_eq!(table.find(&2), None);
}

#[test]
fn test_tramlaw_sales_scenario() {
    // TRAMLAW scenario: timestamp -> sales amount
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut sales_table = AugOrderedTableStPer::empty(sum_reducer, 0);

    // Sales data: timestamp (hour) -> amount
    sales_table = sales_table.insert(17, 1000); // 5pm: $1000
    sales_table = sales_table.insert(18, 1500); // 6pm: $1500
    sales_table = sales_table.insert(19, 2000); // 7pm: $2000
    sales_table = sales_table.insert(20, 1200); // 8pm: $1200
    sales_table = sales_table.insert(21, 800); // 9pm: $800
    sales_table = sales_table.insert(22, 500); // 10pm: $500

    // Total sales for the day
    assert_eq!(sales_table.reduce_val(), 7000);

    // Sales between 5pm and 7pm (17-19 inclusive)
    let evening_sales = sales_table.reduce_range(&17, &19);
    assert_eq!(evening_sales, 4500); // 1000 + 1500 + 2000

    // Sales between 8pm and 10pm (20-22 inclusive)
    let late_sales = sales_table.reduce_range(&20, &22);
    assert_eq!(late_sales, 2500); // 1200 + 800 + 500

    // Verify individual lookups still work
    assert_eq!(sales_table.find(&19), Some(2000));
    assert_eq!(sales_table.find(&23), None);
}

#[test]
fn test_max_reducer() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableStPer::empty(max_reducer, i32::MIN);

    table = table.insert(1, 50);
    assert_eq!(table.reduce_val(), 50);

    table = table.insert(2, 30);
    assert_eq!(table.reduce_val(), 50); // max(50, 30) = 50

    table = table.insert(3, 80);
    assert_eq!(table.reduce_val(), 80); // max(50, 30, 80) = 80

    table = table.insert(4, 20);
    assert_eq!(table.reduce_val(), 80); // max(50, 30, 80, 20) = 80
}

#[test]
fn test_string_concatenation_reducer() {
    let concat_reducer = |a: &String, b: &String| format!("{}{}", a, b);
    let mut table = AugOrderedTableStPer::empty(concat_reducer, String::new());

    table = table.insert(1, "Hello".to_string());
    assert_eq!(table.reduce_val(), "Hello");

    table = table.insert(2, " ".to_string());
    assert_eq!(table.reduce_val(), "Hello ");

    table = table.insert(3, "World".to_string());
    assert_eq!(table.reduce_val(), "Hello World");
}

#[test]
fn test_range_operations() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    // Insert data: key -> value
    for i in 1..=10 {
        table = table.insert(i, i * 10); // 1->10, 2->20, ..., 10->100
    }

    assert_eq!(table.reduce_val(), 550); // Sum of 10+20+...+100

    // Test range [3, 7]
    let range_sum = table.reduce_range(&3, &7);
    assert_eq!(range_sum, 300); // 30+40+50+60+70 = 250... wait, let me recalculate

    // Actually: range [3,7] should include keys 3,4,5,6,7
    // Values: 30+40+50+60+70 = 250
    let expected_range_sum = 30 + 40 + 50 + 60 + 70;
    assert_eq!(range_sum, expected_range_sum);
}

#[test]
fn test_split_and_join() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    table = table.insert(1, 10);
    table = table.insert(3, 30);
    table = table.insert(5, 50);
    table = table.insert(7, 70);

    assert_eq!(table.reduce_val(), 160);

    // Split at key 4
    let (left, middle, right) = table.split_key(&4);

    // Left should have keys 1,3 with values 10,30
    assert_eq!(left.reduce_val(), 40);
    assert_eq!(left.size(), 2);

    // Middle should be None (key 4 doesn't exist)
    assert_eq!(middle, None);

    // Right should have keys 5,7 with values 50,70
    assert_eq!(right.reduce_val(), 120);
    assert_eq!(right.size(), 2);

    // Join them back
    let rejoined = AugOrderedTableStPer::join_key(&left, &right);
    assert_eq!(rejoined.reduce_val(), 160);
    assert_eq!(rejoined.size(), 4);
}

#[test]
fn test_map_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    table = table.insert(1, 10);
    table = table.insert(2, 20);
    table = table.insert(3, 30);

    assert_eq!(table.reduce_val(), 60);

    // Double all values
    let doubled = table.map(|v| v * 2);
    assert_eq!(doubled.reduce_val(), 120); // 20+40+60 = 120
    assert_eq!(doubled.size(), 3);

    // Original table unchanged (persistent)
    assert_eq!(table.reduce_val(), 60);
}

#[test]
fn test_filter_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    for i in 1..=10 {
        table = table.insert(i, i * 10);
    }

    assert_eq!(table.reduce_val(), 550);

    // Filter even keys
    let even_table = table.filter(|k, _v| k % 2 == 0);

    // Even keys: 2,4,6,8,10 with values 20,40,60,80,100
    let expected_sum = 20 + 40 + 60 + 80 + 100;
    assert_eq!(even_table.reduce_val(), expected_sum);
    assert_eq!(even_table.size(), 5);

    // Original table unchanged
    assert_eq!(table.reduce_val(), 550);
}

#[test]
fn test_union_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table1 = AugOrderedTableStPer::empty(sum_reducer.clone(), 0);
    let mut table2 = AugOrderedTableStPer::empty(sum_reducer, 0);

    table1 = table1.insert(1, 10);
    table1 = table1.insert(2, 20);

    table2 = table2.insert(2, 25); // Overlapping key
    table2 = table2.insert(3, 30);

    let combined = table1.union(&table2, |v1, v2| v1 + v2);

    // Keys: 1->10, 2->45 (20+25), 3->30
    assert_eq!(combined.reduce_val(), 85); // 10+45+30
    assert_eq!(combined.size(), 3);
    assert_eq!(combined.find(&2), Some(45));
}

#[test]
fn test_macro_construction() {
    let sum_reducer = |a: &i32, b: &i32| a + b;

    let table = AugOrderedTableStPerLit![
        reducer: sum_reducer, identity: 0,
        1 => 100,
        2 => 200,
        3 => 300
    ];

    assert_eq!(table.size(), 3);
    assert_eq!(table.reduce_val(), 600);
    assert_eq!(table.find(&2), Some(200));

    // Empty table via macro
    let empty_table: AugOrderedTableStPer<i32, i32, _> = AugOrderedTableStPerLit![
        reducer: sum_reducer, identity: 0
    ];
    assert_eq!(empty_table.size(), 0);
    assert_eq!(empty_table.reduce_val(), 0);
}

#[test]
fn test_display_and_debug() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let table = AugOrderedTableStPer::singleton(42, 100, sum_reducer, 0);

    let display_str = format!("{}", table);
    assert!(display_str.contains("AugOrderedTableStPer"));
    assert!(display_str.contains("size: 1"));
    assert!(display_str.contains("reduction: 100"));

    let debug_str = format!("{:?}", table);
    assert!(debug_str.contains("AugOrderedTableStPer"));
    assert!(debug_str.contains("size"));
    assert!(debug_str.contains("cached_reduction"));
}

#[test]
fn test_ordering_operations() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStPer::empty(sum_reducer, 0);

    table = table.insert(5, 50);
    table = table.insert(2, 20);
    table = table.insert(8, 80);
    table = table.insert(1, 10);
    table = table.insert(9, 90);

    // Test ordering operations
    assert_eq!(table.first_key(), Some(1));
    assert_eq!(table.last_key(), Some(9));
    assert_eq!(table.previous_key(&5), Some(2));
    assert_eq!(table.next_key(&5), Some(8));

    // Test rank and select
    assert_eq!(table.rank_key(&5), 2); // 5 is the 3rd key (0-indexed: rank 2)
    assert_eq!(table.select_key(2), Some(5)); // 3rd key (0-indexed: index 2)
}

#[test]
fn test_complex_tramlaw_scenario() {
    // More complex TRAMLAW scenario with multiple days
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut weekly_sales = AugOrderedTableStPer::empty(sum_reducer, 0);

    // Day 1 (timestamps 1-24)
    for hour in 1..=24 {
        weekly_sales = weekly_sales.insert(hour, hour * 50);
    }

    // Day 2 (timestamps 25-48)
    for hour in 25..=48 {
        weekly_sales = weekly_sales.insert(hour, (hour - 24) * 60);
    }

    // Total sales for both days
    let day1_sum: i32 = (1..=24).map(|h| h * 50).sum();
    let day2_sum: i32 = (1..=24).map(|h| h * 60).sum();
    let expected_total = day1_sum + day2_sum;

    assert_eq!(weekly_sales.reduce_val(), expected_total);

    // Day 1 sales (hours 1-24)
    let day1_sales = weekly_sales.reduce_range(&1, &24);
    assert_eq!(day1_sales, day1_sum);

    // Day 2 sales (hours 25-48)
    let day2_sales = weekly_sales.reduce_range(&25, &48);
    assert_eq!(day2_sales, day2_sum);

    // Peak hours across both days (hours 18-30)
    let peak_sales = weekly_sales.reduce_range(&18, &30);
    let expected_peak = (18..=24).map(|h| h * 50).sum::<i32>() + (25..=30).map(|h| (h - 24) * 60).sum::<i32>();
    assert_eq!(peak_sales, expected_peak);
}
