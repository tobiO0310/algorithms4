use crate::{
    collections::{Bag, Stack},
    graphs::{EdgeWeightedGraph, WeightedDirectedEdge, WeightedEdge},
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
    adjacent: Vec<Bag<WeightedEdge>>,
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

impl EdgeWeightedGraph<WeightedEdge> for WeightedGraph {
    fn vertices(&self) -> usize {
        self.vertices
    }

    fn edges(&self) -> usize {
        self.edges
    }

    fn degree(&self, v: usize) -> usize {
        self.adjacent[v].len()
    }

    fn adjacent(&self, v: usize) -> impl Iterator<Item = WeightedEdge> {
        self.adjacent[v].iter().copied()
    }

    fn add_edge(&mut self, edge: WeightedEdge) {
        let v = edge.either();
        let w = edge.other(v);
        self.adjacent[v].insert(edge);
        self.adjacent[w].insert(edge);
        self.edges += 1;
    }

    fn all_edges(&self) -> impl Iterator<Item = WeightedEdge> {
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

    fn to_dot(&self) -> String {
        let mut str = String::new();
        str.push_str(
            "digraph {
            node[shape=circle, style=filled, fixedsize=true, width=0.3, fontsize=\"10pt\"]"
        );

        let mut self_loop = false;
        for v in 0..self.vertices {
            for &e in self.adjacent[v].iter() {
                let w = e.other(v);
                if v < w {
                    str.push_str(
                        format!(
                            "{:?} -- {:?} [weight={:?}]\n",
                            v,
                            w,
                            e.weight()
                        )
                        .as_str(),
                    );
                } else if v == w {
                    // include only one copy of each self loop (self loops will be consecutive)
                    if !self_loop {
                        str.push_str(
                            format!(
                                "{:?} -- {:?} [weight={:?}]\n",
                                v,
                                w,
                                e.weight()
                            )
                            .as_str(),
                        );
                    }
                    self_loop = !self_loop;
                }
            }
        }
        str.push_str("}\n");

        str
    }
}

/// A weighted directed graph implemented with adjencency lists.
///
/// The graph is made up of the graph vertex set *V* , containing `{0, 1, ..., V - 2, V - 1}`.
/// Parallel edges and self-loops are permitted in this graph.
///
/// This implementation uses &Theta;(*E* + *V*) extra memory,
/// where *E* is the amount of edges and *V* is the amount of vertices.
pub struct WeightedDiGraph {
    vertices: usize,
    edges: usize,
    adjacent: Vec<Bag<WeightedDirectedEdge>>,
}

impl WeightedDiGraph {
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

    /// Finds a directed cycle and returns it, if this graph contains one.
    ///
    /// If there is none, the stack will be empty.
    pub fn get_cycle(&self) -> Stack<WeightedDirectedEdge> {
        let mut marked = vec![false; self.vertices];
        let mut on_stack = vec![false; self.vertices];
        let mut edge_to = vec![None; self.vertices];
        let mut cycle = Stack::new();
        for v in 0..self.vertices {
            if !marked[v] {
                let mut stack = Stack::new();
                stack.push((Some(v), None));
                while let Some((elem, finished)) = stack.pop() {
                    if let Some(finished) = finished {
                        on_stack[finished] = false;
                    } else if let Some(v) = elem {
                        on_stack[v] = true;
                        stack.push((None, Some(v)));
                        marked[v] = true;
                        for e in self.adjacent(v) {
                            let w = e.to();

                            if !cycle.is_empty() {
                                break;
                            } else if !marked[w] {
                                edge_to[w] = Some(e);
                                stack.push((Some(w), None));
                            } else if on_stack[w] {
                                let mut f = e;
                                while f.from() != w {
                                    cycle.push(f);
                                    f = edge_to[f.from()].unwrap();
                                }
                                cycle.push(f);

                                break;
                            }
                        }
                    }
                }
            }
        }

        cycle
    }
}

impl EdgeWeightedGraph<WeightedDirectedEdge> for WeightedDiGraph {
    fn vertices(&self) -> usize {
        self.vertices
    }

    fn edges(&self) -> usize {
        self.edges
    }

    fn degree(&self, v: usize) -> usize {
        self.adjacent[v].len()
    }

    fn adjacent(&self, v: usize) -> impl Iterator<Item = WeightedDirectedEdge> {
        self.adjacent[v].iter().copied()
    }

    fn add_edge(&mut self, edge: WeightedDirectedEdge) {
        self.adjacent[edge.from()].insert(edge);
        self.edges += 1;
    }

    fn all_edges(&self) -> impl Iterator<Item = WeightedDirectedEdge> {
        let mut bag = Bag::new();
        for v in 0..self.vertices {
            for &e in self.adjacent[v].iter() {
                bag.insert(e)
            }
        }
        bag.into_iter()
    }

    fn to_dot(&self) -> String {
        let mut str = String::new();
        str.push_str(
            "graph {
            node[shape=circle, style=filled, fixedsize=true, width=0.3, fontsize=\"10pt\"]"
        );

        for v in 0..self.vertices {
            for &e in self.adjacent[v].iter() {
                let w = e.to();
                if v <= w {
                    str.push_str(
                        format!(
                            "{:?} -> {:?} [weight={:}]\n",
                            v,
                            w,
                            e.weight()
                        )
                        .as_str(),
                    );
                }
            }
        }
        str.push_str("}\n");

        str
    }
}
