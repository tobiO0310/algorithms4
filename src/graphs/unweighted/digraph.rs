use crate::{
    collections::{Bag, Queue, Stack},
    graphs::Graph,
};

/// An unweighted directed graph implemented with adjencency lists.
///
/// The graph is made up of the graph vertex set *V* , containing `{0, 1, ..., V - 2, V - 1}`.
/// Parallel edges and self-loops are permitted in this graph.
///
/// This implementation uses &Theta;(*E* + *V*) extra memory,
/// where *E* is the amount of edges and *V* is the amount of vertices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectedGraph {
    vertices: usize,
    edges: usize,
    adjacent: Vec<Bag<usize>>,
}

impl DirectedGraph {
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

    /// Returns the inverse of this graph.
    #[must_use]
    pub fn inverse(&self) -> DirectedGraph {
        let mut inv = DirectedGraph::new(self.vertices);
        for v in 0..self.vertices {
            for w in self.adjacent(v) {
                inv.add_edge(w, v);
            }
        }
        inv
    }
}

impl Graph for DirectedGraph {
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
        self.edges += 1;
    }

    fn to_dot(&self) -> String {
        let mut str = String::new();
        str.push_str(
            "graph {
            node[shape=circle, style=filled, fixedsize=true, width=0.3, fontsize=\"10pt\"]"
        );

        for v in 0..self.vertices {
            for &w in self.adjacent[v].iter() {
                if v <= w {
                    str.push_str(format!("{:?} -> {:?}\n", v, w).as_str());
                }
            }
        }
        str.push_str("}\n");

        str
    }
}

/// The result from running [dfo].
#[must_use = "Runnning Depth-First-Order is meaningless if result is ignored"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DFOResult(Queue<usize>, Queue<usize>, Stack<usize>, Vec<bool>);

impl DFOResult {
    /// Returns the pre order from the [DFOResult].
    pub fn pre_order(&self) -> impl Iterator<Item = usize> {
        self.0.iter().copied()
    }

    /// Returns the post order from the [DFOResult].
    pub fn post_order(&self) -> impl Iterator<Item = usize> {
        self.1.iter().copied()
    }

    /// Returns the reverse post order from the [DFOResult].
    pub fn reverse_post_order(&self) -> impl Iterator<Item = usize> {
        self.2.iter().copied()
    }

    /// Returns the topological order from the [DFOResult].
    ///
    /// If the input graph is not a DAG, this is an unusable value.
    pub fn topological_sort(&self) -> impl Iterator<Item = usize> {
        self.reverse_post_order()
    }

    /// Returns the marked array
    pub fn marked(&self) -> &Vec<bool> {
        &self.3
    }
}

/// Runs DFS on the entire graph to get pre, post and reverse post order.
///
/// This is useful for topological sort.
///
/// # Panics
///
/// Panics if there are no vertices or edges
pub fn dfo(graph: &DirectedGraph) -> DFOResult {
    if graph.vertices() == 0 {
        panic!("graph")
    }
    let mut pre = Queue::new();
    let mut post = Queue::new();
    let mut reverse_post = Stack::new();
    let mut marked = vec![false; graph.vertices()];

    let mut stack = Stack::new();

    for v in 0..graph.vertices() {
        if !marked[v] {
            stack.push((Some(v), None));
            while let Some((v, finished)) = stack.pop() {
                if let Some(finished) = finished {
                    // if a node has been visited, add to post order
                    post.enqueue(finished);
                    reverse_post.push(finished);
                } else if let Some(v) = v {
                    pre.enqueue(v);
                    marked[v] = true;
                    stack.push((None, Some(v)));
                    // ^ make sure, that after it's children, v is added to post-order

                    for w in graph.adjacent(v) {
                        if !marked[w] {
                            stack.push((Some(w), None));
                        }
                    }
                }
            }
        }
    }

    DFOResult(pre, post, reverse_post, marked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph = DirectedGraph::new(10);

        assert_eq!(graph.edges(), 0);
        assert_eq!(graph.vertices(), 10);
        assert_eq!(graph.adjacent.len(), 10);

        graph.add_edge(0, 5);

        assert_eq!(graph.edges(), 1);
        assert_eq!(graph.vertices(), 10);
        assert_eq!(graph.adjacent.len(), 10);

        assert_eq!(vec![5], graph.adjacent(0).collect::<Vec<usize>>());
        assert_ne!(vec![0], graph.adjacent(5).collect::<Vec<usize>>());
    }

    #[test]
    #[should_panic]
    fn panic_on_too_high_vertex_iter() {
        let graph = DirectedGraph::new(1);
        let _ = graph.adjacent(1);
    }

    #[test]
    #[should_panic]
    fn panic_on_too_high_vertex_add() {
        let mut graph = DirectedGraph::new(1);
        graph.add_edge(1, 0);
    }
}
