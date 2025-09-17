pub mod TestDirGraphStEphChap6_1 {
    use apas_ai::DirGraphStEphChap6_1::DirGraphStEphChap6_1::*;
    use apas_ai::SetLit;
    use apas_ai::SetStEphChap5_1::SetStEphChap5_1::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_digraph_vertices_and_arcs() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<Pair<N, N>> = {
            let mut s: Set<Pair<N, N>> = Set::empty();
            let _ = s.insert(Pair(0, 1));
            let _ = s.insert(Pair(1, 2));
            let _ = s.insert(Pair(2, 3));
            let _ = s.insert(Pair(3, 3));
            s
        }; // includes self-loop (3,3)
        let g = DirGraphStEph::FromSets(v.clone(), a.clone());
        assert_eq!(g.sizeV(), v.size());
        assert_eq!(g.sizeA(), a.size());
        assert_eq!(g.vertices(), &v);
        assert_eq!(g.arcs(), &a);
    }
}
