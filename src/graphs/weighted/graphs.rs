use crate::{
    collections::Bag,
    graphs::{EdgeWeightedGraph, UndirectedEdge, WeightedEdge},
};

/// A weighted undirected graph implemented with adjencency lists.
///
/// The graph is made up of the graph vertex set *V* , containing `{0, 1, ..., V - 2, V - 1}`.
/// Parallel edges and self-loops are permitted in this graph.
///
/// This implementation uses &Theta;(*E* + *V*) extra memory,
/// where *E* is the amount of edges and *V* is the amount of vertices.
pub struct WeightedGraph {
    vertices: usize,
    edges: usize,
    adjacent: Vec<Bag<UndirectedEdge>>,
}

impl WeightedGraph {
    /// Creates a new graph with a given amount of vertices.
    #[must_use]
    pub fn new(vertices: usize) -> Self {
        let mut adj = Vec::with_capacity(vertices);
        adj.resize_with(vertices, Bag::new);
        Self {
            vertices,
            edges: 0,
            adjacent: adj,
        }
    }
}

impl EdgeWeightedGraph<UndirectedEdge> for WeightedGraph {
    fn vertices(&self) -> usize {
        self.vertices
    }

    fn edges(&self) -> usize {
        self.edges
    }

    fn degree(&self, v: usize) -> usize {
        self.adjacent[v].len()
    }

    fn adjacent(&self, v: usize) -> impl Iterator<Item = UndirectedEdge> {
        self.adjacent[v].iter().copied()
    }

    fn add_edge(&mut self, edge: UndirectedEdge) {
        let v = edge.either();
        let w = edge.other(v);
        self.adjacent[v].insert(edge);
        self.adjacent[w].insert(edge);
        self.edges += 1;
    }

    fn all_edges(&self) -> impl Iterator<Item = UndirectedEdge> {
        let mut bag = Bag::new();
        for v in 0..self.vertices {
            for &e in self.adjacent[v].iter() {
                if e.other(v) > v {
                    bag.insert(e)
                }
            }
        }
        bag.into_iter()
    }
}
