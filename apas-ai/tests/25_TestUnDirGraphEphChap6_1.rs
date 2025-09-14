pub mod TestUnDirGraphEphChap6_1 {
use apas_ai::Types::Types::*;
use apas_ai::SetEphChap5_1::SetEphChap5_1::*;
use apas_ai::UnDirGraphEphChap6_1::UnDirGraphEphChap6_1::*;
use apas_ai::SetLit;

#[test]
fn test_undigraph_vertices_and_edges() {
    let v: Set<N> = SetLit![0,1,2,3];
    let e: Set<(N,N)> = SetLit![(0,1),(1,2),(2,3),(3,3)]; // allow self-loop representation
    let g = UnDirGraphEph::FromSets(v.clone(), e.clone());
    assert_eq!(g.sizeV(), v.size());
    assert_eq!(g.sizeE(), e.size());
    assert_eq!(g.vertices(), &v);
    assert_eq!(g.edges(), &e);
}
}


