pub mod TestRelationEphChap5_2 {
use apas_ai::Types::Types::*;
use apas_ai::SetStEphChap5_1::SetStEphChap5_1::*;
use apas_ai::RelationStEphChap5_2::RelationStEphChap5_2::*;
use apas_ai::SetLit; // macro import

#[test]
fn test_relation_domain_range_and_mem() {
    // R subset of A x B
    let pairs = SetLit![Pair(1usize, 'a'), Pair(2usize, 'b'), Pair(1usize, 'b')];
    let r = Relation::FromSet(pairs);

    let d = r.domain();
    let e = SetLit![1usize, 2usize];
    assert_eq!(d, e);

    let rg = r.range();
    let rexp = SetLit!['a', 'b'];
    assert_eq!(rg, rexp);

    assert_eq!(r.mem(&1usize, &'a'), B::True);
    assert_eq!(r.mem(&2usize, &'a'), B::False);
}
}


