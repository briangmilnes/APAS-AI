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
        assert_eq!(ng_2.size(), 1); // vertex 2 has incoming neighbor 1
        assert_eq!(ng_2.mem(&1), B::True);
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
        assert_eq!(ng_subset.size(), 3); // NG(0)={1} ∪ NG(1)={0,2} = {0,1,2}
        assert_eq!(ng_subset.mem(&0), B::True);
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
        assert_eq!(g.Incident(&Edge(0, 1), &0), B::True);  // edge (0,1) is incident to vertex 0
        assert_eq!(g.Incident(&Edge(0, 1), &1), B::True);  // edge (0,1) is incident to vertex 1
        assert_eq!(g.Incident(&Edge(0, 1), &2), B::False); // edge (0,1) is not incident to vertex 2
        assert_eq!(g.Incident(&Edge(1, 2), &1), B::True);  // edge (1,2) is incident to vertex 1
        assert_eq!(g.Incident(&Edge(1, 2), &2), B::True);  // edge (1,2) is incident to vertex 2
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
        
        assert_eq!(g.Degree(&0), 2); // one in + one out = 2
        assert_eq!(g.Degree(&1), 2); // one in + one out = 2
        assert_eq!(g.Degree(&2), 2); // one in + one out = 2
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
                
                // Verify basic properties (degrees are usize, always >= 0)
                // Just verify they're valid values by using them
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

    #[test]
    fn test_race_condition_verification_concurrent_graph_reads() {
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::sync::atomic::{AtomicBool, Ordering};
        
        // Create a test graph
        let vertices: Set<i32> = SetLit![0, 1, 2, 3, 4, 5];
        let edges: Set<Edge<i32>> = {
            let mut s: Set<Edge<i32>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            let _ = s.insert(Edge(2, 3));
            let _ = s.insert(Edge(3, 4));
            let _ = s.insert(Edge(4, 5));
            let _ = s.insert(Edge(5, 0)); // Cycle
            s
        };
        let graph = Arc::new(DirGraphMtEph::FromSets(vertices, edges));
        
        let barrier = Arc::new(Barrier::new(8));
        let race_detected = Arc::new(AtomicBool::new(false));
        let mut handles = vec![];
        
        // Spawn multiple threads that read graph properties simultaneously
        for thread_id in 0..8 {
            let graph_clone = Arc::clone(&graph);
            let barrier_clone = Arc::clone(&barrier);
            let race_detected_clone = Arc::clone(&race_detected);
            
            let handle = thread::spawn(move || {
                barrier_clone.wait();
                
                let mut read_count = 0;
                for _ in 0..100 {
                    // Read various graph properties
                    let size_v = graph_clone.sizeV();
                    let size_a = graph_clone.sizeA();
                    
                    // Verify basic consistency
                    if size_v != 6 || size_a != 6 {
                        race_detected_clone.store(true, Ordering::SeqCst);
                    }
                    
                    // Test neighbor relationships
                    let neighbor_01 = graph_clone.Neighbor(&0, &1);
                    let neighbor_10 = graph_clone.Neighbor(&1, &0);
                    
                    if neighbor_01 != B::True || neighbor_10 != B::False {
                        race_detected_clone.store(true, Ordering::SeqCst);
                    }
                    
                    // Test degree calculations
                    let degree_0 = graph_clone.Degree(&0);
                    let in_degree_0 = graph_clone.InDegree(&0);
                    let out_degree_0 = graph_clone.OutDegree(&0);
                    
                    // For vertex 0: outgoing edge to 1, incoming edge from 5
                    // So degree = 2, in_degree = 1, out_degree = 1
                    if degree_0 != 2 || in_degree_0 != 1 || out_degree_0 != 1 {
                        race_detected_clone.store(true, Ordering::SeqCst);
                    }
                    
                    // Test neighbor sets
                    let ng_0 = graph_clone.NG(&0);
                    let nplus_0 = graph_clone.NPlus(&0);
                    let nminus_0 = graph_clone.NMinus(&0);
                    
                    if ng_0.size() != 2 || nplus_0.size() != 1 || nminus_0.size() != 1 {
                        race_detected_clone.store(true, Ordering::SeqCst);
                    }
                    
                    read_count += 1;
                }
                
                (thread_id, read_count)
            });
            handles.push(handle);
        }
        
        // Collect results
        let mut results = vec![];
        for handle in handles {
            results.push(handle.join().unwrap());
        }
        
        // Verify no race conditions detected
        assert!(!race_detected.load(Ordering::SeqCst), "Race condition detected in concurrent graph reads");
        
        // Verify all threads completed their reads
        for (thread_id, read_count) in results {
            assert_eq!(read_count, 100, "Thread {} didn't complete all reads", thread_id);
        }
    }

    #[test]
    fn test_race_condition_verification_mixed_graph_operations() {
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        
        // Create multiple graphs for concurrent access
        let graphs = Arc::new(std::sync::RwLock::new(vec![
            DirGraphMtEph::empty(),
            DirGraphMtEph::empty(),
            DirGraphMtEph::empty(),
        ]));
        
        let barrier = Arc::new(Barrier::new(9));
        let inconsistency_detected = Arc::new(AtomicBool::new(false));
        let operation_counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        // Spawn reader threads
        for thread_id in 0..3 {
            let graphs_clone = Arc::clone(&graphs);
            let barrier_clone = Arc::clone(&barrier);
            let inconsistency_clone = Arc::clone(&inconsistency_detected);
            
            let handle = thread::spawn(move || {
                barrier_clone.wait();
                
                let mut read_count = 0;
                for _ in 0..50 {
                    if let Ok(graph_vec) = graphs_clone.read() {
                        for (_idx, graph) in graph_vec.iter().enumerate() {
                            let size_v = graph.sizeV();
                            let size_a = graph.sizeA();
                            
                            // Verify basic consistency
                            if size_a > size_v * size_v {
                                inconsistency_clone.store(true, Ordering::SeqCst);
                            }
                            
                            // Test some operations if graph is non-empty
                            if size_v > 0 {
                                let degree_0 = graph.Degree(&0);
                                if degree_0 > size_v {
                                    inconsistency_clone.store(true, Ordering::SeqCst);
                                }
                            }
                            
                            read_count += 1;
                        }
                    }
                }
                
                (thread_id, read_count)
            });
            handles.push(handle);
        }
        
        // Spawn writer threads that create new graphs
        for thread_id in 3..9 {
            let graphs_clone = Arc::clone(&graphs);
            let barrier_clone = Arc::clone(&barrier);
            let operation_counter_clone = Arc::clone(&operation_counter);
            
            let handle = thread::spawn(move || {
                barrier_clone.wait();
                
                let mut write_count = 0;
                for i in 0..10 {
                    let graph_index = (thread_id % 3) as usize;
                    
                    // Create a small graph
                    let vertices: Set<i32> = SetLit![thread_id * 10 + i, thread_id * 10 + i + 1];
                    let edges: Set<Edge<i32>> = {
                        let mut s: Set<Edge<i32>> = Set::empty();
                        let _ = s.insert(Edge(thread_id * 10 + i, thread_id * 10 + i + 1));
                        s
                    };
                    let new_graph = DirGraphMtEph::FromSets(vertices, edges);
                    
                    if let Ok(mut graph_vec) = graphs_clone.write() {
                        graph_vec[graph_index] = new_graph;
                        write_count += 1;
                        operation_counter_clone.fetch_add(1, Ordering::SeqCst);
                    }
                }
                
                (thread_id, write_count)
            });
            handles.push(handle);
        }
        
        // Collect results
        let mut results = vec![];
        for handle in handles {
            results.push(handle.join().unwrap());
        }
        
        // Verify no inconsistencies detected
        assert!(!inconsistency_detected.load(Ordering::SeqCst), 
            "Data inconsistency detected during concurrent graph operations");
        
        // Verify all operations completed
        assert_eq!(operation_counter.load(Ordering::SeqCst), 60, "Not all write operations completed");
        
        // Verify thread results
        for (thread_id, count) in results {
            if thread_id < 3 {
                // Reader thread
                assert!(count > 0, "Reader thread {} performed no reads", thread_id);
            } else {
                // Writer thread
                assert_eq!(count, 10, "Writer thread {} didn't complete all writes", thread_id);
            }
        }
        
        // Final consistency check
        if let Ok(final_graphs) = graphs.read() {
            for (idx, graph) in final_graphs.iter().enumerate() {
                let size_v = graph.sizeV();
                let size_a = graph.sizeA();
                
                // Basic sanity checks
                assert!(size_a <= size_v * size_v, 
                    "Graph {} has {} edges but only {} vertices", idx, size_a, size_v);
                
                if size_v > 0 {
                    // Test that degree calculations don't crash
                    let _ = graph.Degree(&0);
                    let _ = graph.InDegree(&0);
                    let _ = graph.OutDegree(&0);
                }
            }
        }
    }

    #[test]
    fn test_deadlock_prevention_concurrent_graph_operations() {
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};
    
    // Create multiple graphs with different locking orders to test deadlock prevention
    let graph_a = Arc::new(Mutex::new({
        let vertices: Set<i32> = SetLit![0, 1, 2];
        let edges: Set<Edge<i32>> = {
            let mut s: Set<Edge<i32>> = Set::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        DirGraphMtEph::FromSets(vertices, edges)
    }));
    
    let graph_b = Arc::new(Mutex::new({
        let vertices: Set<i32> = SetLit![3, 4, 5];
        let edges: Set<Edge<i32>> = {
            let mut s: Set<Edge<i32>> = Set::empty();
            let _ = s.insert(Edge(3, 4));
            let _ = s.insert(Edge(4, 5));
            s
        };
        DirGraphMtEph::FromSets(vertices, edges)
    }));
    
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];
    
    // Thread 1: A -> B operations
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_a_clone = Arc::clone(&graph_a);
        let graph_b_clone = Arc::clone(&graph_b);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let start_time = Instant::now();
            let mut operations = 0;
            
            while start_time.elapsed() < Duration::from_millis(50) {
                if let (Ok(a), Ok(b)) = (
                    graph_a_clone.try_lock(),
                    graph_b_clone.try_lock()
                ) {
                    // Perform read operations on both graphs
                    let _ = a.sizeV();
                    let _ = a.Degree(&0);
                    let _ = b.sizeA();
                    let _ = b.Neighbor(&3, &4);
                    operations += 1;
                }
                thread::yield_now();
            }
            
            operations
        });
        handles.push(handle);
    }
    
    // Thread 2: B -> A operations (reverse order)
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_a_clone = Arc::clone(&graph_a);
        let graph_b_clone = Arc::clone(&graph_b);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let start_time = Instant::now();
            let mut operations = 0;
            
            while start_time.elapsed() < Duration::from_millis(50) {
                if let (Ok(b), Ok(a)) = (
                    graph_b_clone.try_lock(),
                    graph_a_clone.try_lock()
                ) {
                    // Perform read operations on both graphs
                    let _ = b.InDegree(&4);
                    let _ = b.OutDegree(&3);
                    let _ = a.NG(&1);
                    let _ = a.NPlus(&0);
                    operations += 1;
                }
                thread::yield_now();
            }
            
            operations
        });
        handles.push(handle);
    }
    
    // Thread 3: Mixed operations on A
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_a_clone = Arc::clone(&graph_a);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let start_time = Instant::now();
            let mut operations = 0;
            
            while start_time.elapsed() < Duration::from_millis(50) {
                if let Ok(a) = graph_a_clone.try_lock() {
                    // Perform various operations on graph A
                    let _ = a.NMinus(&2);
                    let _ = a.Incident(&Edge(0, 1), &0);
                    let _ = a.NGOfVertices(&SetLit![0, 1]);
                    operations += 1;
                }
                thread::yield_now();
            }
            
            operations
        });
        handles.push(handle);
    }
    
    // Thread 4: Mixed operations on B
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_b_clone = Arc::clone(&graph_b);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let start_time = Instant::now();
            let mut operations = 0;
            
            while start_time.elapsed() < Duration::from_millis(50) {
                if let Ok(b) = graph_b_clone.try_lock() {
                    // Perform various operations on graph B
                    let _ = b.NPlusOfVertices(&SetLit![3, 4]);
                    let _ = b.NMinusOfVertices(&SetLit![4, 5]);
                    let _ = b.Neighbor(&4, &5);
                    operations += 1;
                }
                thread::yield_now();
            }
            
            operations
        });
        handles.push(handle);
    }
    
    // Collect results - if there's a deadlock, this will hang
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify all threads completed some operations (no deadlock occurred)
    for (thread_id, op_count) in results.iter().enumerate() {
        assert!(*op_count > 0, "Thread {} completed no operations - possible deadlock", thread_id);
    }
    
    // Verify final state is consistent
    let final_a = graph_a.lock().unwrap();
    let final_b = graph_b.lock().unwrap();
    
    assert_eq!(final_a.sizeV(), 3);
    assert_eq!(final_a.sizeA(), 2);
    assert_eq!(final_b.sizeV(), 3);
    assert_eq!(final_b.sizeA(), 2);
    }
}
