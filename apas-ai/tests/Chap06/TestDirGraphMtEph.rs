//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestDirGraphMtEph {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::DirGraphMtEph::DirGraphMtEph::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use std::thread;
    use std::sync::Arc;

    #[test]
    fn test_dirgraphmteph_empty() {
        let empty_graph: DirGraphMtEph<i32> = DirGraphMtEph::empty();
        assert_eq!(empty_graph.sizeV(), 0);
        assert_eq!(empty_graph.sizeA(), 0);
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.arcs().size(), 0);
    }

    #[test]
    fn test_dirgraphmteph_basic_operations() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 3));
            let _ = s.insert(Edge(3, 3));
            s
        }; // includes self-loop (3,3)
        let g = DirGraphMtEph::FromSets(v.clone(), a.clone());
        assert_eq!(g.sizeV(), v.size());
        assert_eq!(g.sizeA(), a.size());
        assert_eq!(g.vertices(), &v);
        assert_eq!(g.arcs(), &a);
    }

    #[test]
    fn test_dirgraphmteph_neighbor() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(0, 2));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        // Test Neighbor method - checks if edge exists between two vertices
        assert_eq!(g.Neighbor(&0, &1), B::True);  // edge 0->1 exists
        assert_eq!(g.Neighbor(&0, &2), B::True);  // edge 0->2 exists
        assert_eq!(g.Neighbor(&1, &2), B::True);  // edge 1->2 exists
        assert_eq!(g.Neighbor(&1, &0), B::False); // edge 1->0 does not exist
        assert_eq!(g.Neighbor(&2, &0), B::False); // edge 2->0 does not exist
        assert_eq!(g.Neighbor(&2, &1), B::False); // edge 2->1 does not exist
    }

    #[test]
    fn test_dirgraphmteph_ng() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        let ng_0 = g.NG(&0);
        assert_eq!(ng_0.size(), 1);
        assert_eq!(ng_0.mem(&1), B::True);
        
        let ng_2 = g.NG(&2);
        assert_eq!(ng_2.size(), 0);
    }

    #[test]
    fn test_dirgraphmteph_ngofvertices() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        let vertices_subset = SetLit![0, 1];
        let ng_subset = g.NGOfVertices(&vertices_subset);
        assert_eq!(ng_subset.size(), 2);
        assert_eq!(ng_subset.mem(&1), B::True);
        assert_eq!(ng_subset.mem(&2), B::True);
    }

    #[test]
    fn test_dirgraphmteph_nplus() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        let nplus_0 = g.NPlus(&0);
        assert_eq!(nplus_0.size(), 1);
        assert_eq!(nplus_0.mem(&1), B::True);
        
        let nplus_2 = g.NPlus(&2);
        assert_eq!(nplus_2.size(), 0);
    }

    #[test]
    fn test_dirgraphmteph_nminus() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        let nminus_1 = g.NMinus(&1);
        assert_eq!(nminus_1.size(), 1);
        assert_eq!(nminus_1.mem(&0), B::True);
        
        let nminus_0 = g.NMinus(&0);
        assert_eq!(nminus_0.size(), 0);
    }

    #[test]
    fn test_dirgraphmteph_nplusofvertices() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        let vertices_subset = SetLit![0, 1];
        let nplus_subset = g.NPlusOfVertices(&vertices_subset);
        assert_eq!(nplus_subset.size(), 2);
        assert_eq!(nplus_subset.mem(&1), B::True);
        assert_eq!(nplus_subset.mem(&2), B::True);
    }

    #[test]
    fn test_dirgraphmteph_nminusofvertices() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        let vertices_subset = SetLit![1, 2];
        let nminus_subset = g.NMinusOfVertices(&vertices_subset);
        assert_eq!(nminus_subset.size(), 2);
        assert_eq!(nminus_subset.mem(&0), B::True);
        assert_eq!(nminus_subset.mem(&1), B::True);
    }

    #[test]
    fn test_dirgraphmteph_incident() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 0));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        // Test Incident method - checks if edge is incident to vertex
        assert_eq!(g.Incident(&Pair(0, 1), &0), B::True);  // edge (0,1) is incident to vertex 0
        assert_eq!(g.Incident(&Pair(0, 1), &1), B::True);  // edge (0,1) is incident to vertex 1
        assert_eq!(g.Incident(&Pair(0, 1), &2), B::False); // edge (0,1) is not incident to vertex 2
        assert_eq!(g.Incident(&Pair(1, 2), &1), B::True);  // edge (1,2) is incident to vertex 1
        assert_eq!(g.Incident(&Pair(1, 2), &2), B::True);  // edge (1,2) is incident to vertex 2
    }

    #[test]
    fn test_dirgraphmteph_degree() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 0));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        assert_eq!(g.Degree(&0), 1); // one in OR one out (not both)
        assert_eq!(g.Degree(&1), 1); // one in OR one out (not both)
        assert_eq!(g.Degree(&2), 1); // one in OR one out (not both)
    }

    #[test]
    fn test_dirgraphmteph_indegree() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 0));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        assert_eq!(g.InDegree(&0), 1); // edge from 2
        assert_eq!(g.InDegree(&1), 1); // edge from 0
        assert_eq!(g.InDegree(&2), 1); // edge from 1
    }

    #[test]
    fn test_dirgraphmteph_outdegree() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 0));
            s
        };
        let g = DirGraphMtEph::FromSets(v, a);
        
        assert_eq!(g.OutDegree(&0), 1); // edge to 1
        assert_eq!(g.OutDegree(&1), 1); // edge to 2
        assert_eq!(g.OutDegree(&2), 1); // edge to 0
    }

    #[test]
    fn test_dirgraphmteph_concurrent_access() {
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 3));
            let _ = s.insert(Edge(3, 4));
            s
        };
        let g = Arc::new(DirGraphMtEph::FromSets(v, a));
        
        let mut handles = vec![];
        
        // Spawn multiple threads to test concurrent access
        for i in 0..4 {
            let g_clone = Arc::clone(&g);
            let handle = thread::spawn(move || {
                // Each thread performs different graph operations
                let ng = g_clone.NG(&i);
                let degree = g_clone.Degree(&i);
                let in_degree = g_clone.InDegree(&i);
                let out_degree = g_clone.OutDegree(&i);
                
                // Verify basic properties
                assert!(degree >= 0);
                assert!(in_degree >= 0);
                assert!(out_degree >= 0);
                assert_eq!(g_clone.sizeV(), 5);
                assert_eq!(g_clone.sizeA(), 4);
                
                (ng.size(), degree, in_degree, out_degree)
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            let _ = handle.join().expect("Thread should complete successfully");
        }
    }

    #[test]
    fn test_dirgraphmteph_thread_safety() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<Edge<N>> = {
            let mut s: Set<Edge<N>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        let g = Arc::new(DirGraphMtEph::FromSets(v, a));
        
        let mut handles = vec![];
        
        // Test that multiple threads can safely read from the graph simultaneously
        for _ in 0..10 {
            let g_clone = Arc::clone(&g);
            let handle = thread::spawn(move || {
                // Perform various read operations
                assert_eq!(g_clone.Neighbor(&0, &1), B::True);
                assert_eq!(g_clone.Neighbor(&1, &0), B::False);
                assert_eq!(g_clone.sizeV(), 3);
                assert_eq!(g_clone.sizeA(), 2);
                
                let ng_0 = g_clone.NG(&0);
                assert_eq!(ng_0.size(), 1);
                assert_eq!(ng_0.mem(&1), B::True);
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }
    }
}
