pub mod TestDirGraphEphChap6_1 {
use apas_ai::Types::Types::*;
use apas_ai::SetEphChap5_1::SetEphChap5_1::*;
use apas_ai::DirGraphEphChap6_1::DirGraphEphChap6_1::*;
use apas_ai::SetLit;

#[test]
fn test_digraph_vertices_and_arcs() {
    let v: Set<N> = SetLit![0,1,2,3];
    let a: Set<(N,N)> = SetLit![(0,1),(1,2),(2,3),(3,3)]; // includes self-loop (3,3)
    let g = DirGraphEph::FromSets(v.clone(), a.clone());
    assert_eq!(g.sizeV(), v.size());
    assert_eq!(g.sizeA(), a.size());
    assert_eq!(g.vertices(), &v);
    assert_eq!(g.arcs(), &a);
}
}


