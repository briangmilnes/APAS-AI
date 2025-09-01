//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
// There is no functionality here at the moment, this just tests imports of data structures
// traits and implementations.

use apas_ai::Sequences::*;

fn main() {
    let a = apas_ai::seq![1, 2, 3];
    let len: N = a.length();
    let empty: B = a.isEmpty();
    println!("length (N) = {}", len);
    println!("isEmpty (B) = {:?}", empty);
    println!("isSingleton (B) = {:?}", a.isSingleton());

    // Construct a singleton via the trait's associated function (UFCS)
    let s1 = <S<&str> as Sequence<&str>>::singleton("hi");
    println!("s1 isSingleton = {:?}", s1.isSingleton());

    // Chainable in-place updates and indexed access
    let mut b = apas_ai::seq![10, 20, 30];
    let _ = b.update((1, 99)).update((0, 5));
    println!("b[1] = {}", b.nth(1));

    // Demonstrate O (Ordering) from re-export
    let ord: O = 2.cmp(&3);
    println!("cmp(2,3) -> O = {:?}", ord);
}
