pub mod TestUnDirGraphStEph {
    use apas_ai::SetLit;
    use apas_ai::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
    use apas_ai::Types::Types::*;
    use apas_ai::Chap6::UnDirGraphStEph::UnDirGraphStEph::*;

    #[test]
    fn test_undigraph_vertices_and_edges() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let e: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 3));
            let _ = s.insert(Edge(3, 3));
            s
        }; // allow self-loop representation
        let g = UnDirGraphStEph::FromSets(v.clone(), e.clone());
        assert_eq!(g.sizeV(), v.size());
        assert_eq!(g.sizeE(), e.size());
        assert_eq!(g.vertices(), &v);
        assert_eq!(g.edges(), &e);
    }
}
