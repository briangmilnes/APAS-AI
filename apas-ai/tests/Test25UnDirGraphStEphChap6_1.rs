pub mod TestUnDirGraphStEphChap6_1 {
    use apas_ai::SetLit;
    use apas_ai::SetStEphChap5_1::SetStEphChap5_1::*;
    use apas_ai::Types::Types::*;
    use apas_ai::UnDirGraphStEphChap6_1::UnDirGraphStEphChap6_1::*;

    #[test]
    fn test_undigraph_vertices_and_edges() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let e: Set<Pair<N, N>> = {
            let mut s: Set<Pair<N, N>> = Set::empty();
            let _ = s.insert(Pair(0, 1));
            let _ = s.insert(Pair(1, 2));
            let _ = s.insert(Pair(2, 3));
            let _ = s.insert(Pair(3, 3));
            s
        }; // allow self-loop representation
        let g = UnDirGraphStEph::FromSets(v.clone(), e.clone());
        assert_eq!(g.sizeV(), v.size());
        assert_eq!(g.sizeE(), e.size());
        assert_eq!(g.vertices(), &v);
        assert_eq!(g.edges(), &e);
    }
}
