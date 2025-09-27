//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestUnDirGraphStEph {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;

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
