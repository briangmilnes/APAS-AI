//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.


    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use apas_ai::UnDirGraphStEphLit;

    #[test]
    fn test_undirgraphstephlit_macro_functionality() {
        // Test empty graph creation
        let empty: UnDirGraphStEph<i32> = UnDirGraphStEphLit!();
        assert_eq!(empty.vertices().size(), 0);
        assert_eq!(empty.edges().size(), 0);
        
        // Test graph creation with vertices and edges
        let with_data: UnDirGraphStEph<i32> = UnDirGraphStEphLit!(
            V: [1, 2, 3],
            E: [(1, 2), (2, 3)]
        );
        assert_eq!(with_data.vertices().size(), 3);
        assert_eq!(with_data.edges().size(), 2);
    }

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

