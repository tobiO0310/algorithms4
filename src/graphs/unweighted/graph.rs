use crate::{collections::Bag, graphs::Graph};

/// An unweighted undirected graph implemented with adjencency lists.
///
/// The graph is made up of the graph vertex set *V* , containing `{0, 1, ..., V - 2, V - 1}`.
/// Parallel edges and self-loops are permitted in this graph.
///
/// This implementation uses &Theta;(*E* + *V*) extra memory,
/// where *E* is the amount of edges and *V* is the amount of vertices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndirectedGraph {
    vertices: usize,
    edges: usize,
    adjacent: Vec<Bag<usize>>,
}

impl UndirectedGraph {
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

impl Graph for UndirectedGraph {
    fn vertices(&self) -> usize {
        self.vertices
    }

    fn edges(&self) -> usize {
        self.edges
    }

    fn degree(&self, v: usize) -> usize {
        self.adjacent[v].len()
    }

    fn adjacent(&self, v: usize) -> impl Iterator<Item = usize> {
        self.adjacent[v].iter().copied()
    }

    fn add_edge(&mut self, v: usize, w: usize) {
        self.adjacent[v].insert(w);
        self.adjacent[w].insert(v);
        self.edges += 1;
    }

    fn to_dot(&self) -> String {
        let mut str = String::new();
        str.push_str(
            "graph {
            node[shape=circle, style=filled, fixedsize=true, width=0.3, fontsize=\"10pt\"]"
        );

        let mut self_loop = false;
        for v in 0..self.vertices {
            for &w in self.adjacent[v].iter() {
                if v < w {
                    str.push_str(format!("{:?} -- {:?}\n", v, w).as_str());
                } else if v == w {
                    // include only one copy of each self loop (self loops will be consecutive)
                    if !self_loop {
                        str.push_str(format!("{:?} -- {:?}\n", v, w).as_str());
                    }
                    self_loop = !self_loop;
                }
            }
        }
        str.push_str("}\na");

        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph = UndirectedGraph::new(10);

        assert_eq!(graph.edges(), 0);
        assert_eq!(graph.vertices(), 10);
        assert_eq!(graph.adjacent.len(), 10);

        graph.add_edge(0, 5);

        assert_eq!(graph.edges(), 1);
        assert_eq!(graph.vertices(), 10);
        assert_eq!(graph.adjacent.len(), 10);

        assert_eq!(vec![5], graph.adjacent(0).collect::<Vec<usize>>())
    }

    #[test]
    #[should_panic]
    fn panic_on_too_high_vertex_iter() {
        let graph = UndirectedGraph::new(1);
        let _ = graph.adjacent(1);
    }

    #[test]
    #[should_panic]
    fn panic_on_too_high_vertex_add() {
        let mut graph = UndirectedGraph::new(1);
        graph.add_edge(0, 1);
    }
}
