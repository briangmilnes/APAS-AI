//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Edge Set Graph representation (persistent, single-threaded).
//! G = (V, E) where V is a set of vertices and E ⊆ V × V is a set of edges.

pub mod EdgeSetGraphStPer {

    use crate::Types::Types::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct EdgeSetGraphStPer<V: StT + Ord> {
        vertices: AVLTreeSetStPer<V>,
        edges: AVLTreeSetStPer<Pair<V, V>>,
    }

    pub trait EdgeSetGraphStPerTrait<V: StT + Ord> {
        fn empty() -> Self;
        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> Self;
        fn num_vertices(&self) -> N;
        fn num_edges(&self) -> N;
        fn vertices(&self) -> &AVLTreeSetStPer<V>;
        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>>;
        fn has_edge(&self, u: &V, v: &V) -> B;
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStPer<V>;
        fn out_degree(&self, u: &V) -> N;
        fn insert_vertex(&self, v: V) -> Self;
        fn delete_vertex(&self, v: &V) -> Self;
        fn insert_edge(&self, u: V, v: V) -> Self;
        fn delete_edge(&self, u: &V, v: &V) -> Self;
    }

    impl<V: StT + Ord> EdgeSetGraphStPerTrait<V> for EdgeSetGraphStPer<V> {
        fn empty() -> Self {
            EdgeSetGraphStPer {
                vertices: AVLTreeSetStPer::empty(),
                edges: AVLTreeSetStPer::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> Self {
            EdgeSetGraphStPer { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetStPer<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>> { &self.edges }

        // Work: Θ(log |E|), Span: Θ(log |E|)
        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        // Work: Θ(|E|), Span: Θ(log |E|) - filter over all edges
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStPer<V> {
            let u_clone = u.clone();
            let filtered = self.edges.filter(|edge| &edge.0 == &u_clone);
            let mut neighbors = AVLTreeSetStPer::empty();
            let seq = filtered.to_seq();
            for i in 0..seq.length() {
                let Pair(_, v) = seq.nth(i);
                neighbors = neighbors.insert(v.clone());
            }
            neighbors
        }

        // Work: Θ(|E|), Span: Θ(log |E|)
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        // Work: Θ(log |V|), Span: Θ(log |V|)
        fn insert_vertex(&self, v: V) -> Self {
            EdgeSetGraphStPer {
                vertices: self.vertices.insert(v),
                edges: self.edges.clone(),
            }
        }

        // Work: Θ(|E| log |E|), Span: Θ(log² |E|) - must remove all incident edges
        fn delete_vertex(&self, v: &V) -> Self {
            let v_clone = v.clone();
            let new_vertices = self.vertices.delete(&v_clone);
            let new_edges = self.edges.filter(|edge| {
                let Pair(u, w) = edge;
                u != &v_clone && w != &v_clone
            });
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        // Work: Θ(log |V| + log |E|), Span: Θ(log |V| + log |E|)
        fn insert_edge(&self, u: V, v: V) -> Self {
            let new_vertices = self.vertices.insert(u.clone()).insert(v.clone());
            let new_edges = self.edges.insert(Pair(u, v));
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        // Work: Θ(log |E|), Span: Θ(log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> Self {
            let new_edges = self.edges.delete(&Pair(u.clone(), v.clone()));
            EdgeSetGraphStPer {
                vertices: self.vertices.clone(),
                edges: new_edges,
            }
        }
    }
}
