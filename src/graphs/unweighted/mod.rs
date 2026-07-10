mod digraph;
mod graph;
pub use digraph::{DFOResult, DirectedGraph, dfo};
pub use graph::UndirectedGraph;

use crate::collections::Queue;

/// A collection of functions that any unweighted graph will implement
pub trait Graph {
    /// The amount of vertices in this graph
    #[must_use]
    fn vertices(&self) -> usize;

    /// The amount of edges in this graph
    #[must_use]
    fn edges(&self) -> usize;

    /// Returns the degree of vertex *v*
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to `vertices`.
    #[must_use]
    fn degree(&self, v: usize) -> usize;

    /// Returns the vertices adjacent with vertex *v*
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to `vertices`
    #[must_use]
    fn adjacent(&self, v: usize) -> impl Iterator<Item = usize>;

    /// Adds an edge between *v* and *w*.
    ///
    /// # Panics
    ///
    /// Panics if *v* or *w* is bigger than or equal to `vertices`
    fn add_edge(&mut self, v: usize, w: usize);

    /// Returns a string representation of this graph in DOT format,
    /// which can be used to visualize it with Graphviz.
    ///
    /// To visualize the graph, install Graphviz (e.g., `brew install graphviz`).
    /// Then use one of the graph visualization tools
    ///    - dot    (hierarchical or layer drawing)
    ///    - neato  (spring model)
    ///    - fdp    (force-directed placement)
    ///    - sfdp   (scalable force-directed placement)
    ///    - twopi  (radial layout)
    ///
    /// For example, the following commands will create graph drawings in SVG
    /// and PDF formats
    ///    - `dot input.dot -Tsvg -o output.svg`
    ///    - `dot input.dot -Tpdf -o output.pdf`
    ///
    /// To change the graph attributes (e.g., vertex and edge shapes, arrows, colors)
    ///  in the DOT format, see <https://graphviz.org/doc/info/lang.html>
    #[must_use]
    fn to_dot(&self) -> String;
}

/// The result from running [dfs]
#[must_use = "Running Depth-First-Search without using the result is meaningless"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DFSResult(Vec<bool>, usize, usize);

impl DFSResult {
    /// The marked array
    #[must_use]
    pub fn marked(&self) -> &Vec<bool> {
        &self.0
    }

    /// The start of the DFS.
    #[must_use]
    pub fn start(&self) -> usize {
        self.1
    }

    /// The amount of vertices reachable from *start*.
    #[must_use]
    pub fn amount(&self) -> usize {
        self.2
    }

    /// Indicates whether *w* is reachable from *start* via traversal in the graph.
    #[must_use]
    pub fn has_path_to(&self, w: usize) -> bool {
        self.0.get(w).is_some_and(|&v| v)
    }
}

/// The result from running [dfs_mul]
#[must_use = "Running Depth-First-Search without using the result is meaningless"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DFSMultResult(Vec<bool>, Vec<usize>, usize);

impl DFSMultResult {
    /// The marked array
    #[must_use]
    pub fn marked(&self) -> &Vec<bool> {
        &self.0
    }

    /// The start of the DFS.
    #[must_use]
    pub fn starts(&self) -> &Vec<usize> {
        &self.1
    }

    /// The amount of vertices reachable from *start*.
    #[must_use]
    pub fn amount(&self) -> usize {
        self.2
    }

    /// Indicates whether *w* is reachable from *start* via traversal in the graph.
    #[must_use]
    pub fn has_path_to(&self, w: usize) -> bool {
        self.0.get(w).is_some_and(|&v| v)
    }
}

/// Runs Depth-First-Search on a [Graph], starting at *start*.
///
/// # Panics
///
/// Panics if *start* is bigger than or equal to vertex amount in graph
pub fn dfs<T: Graph>(graph: &T, start: usize) -> DFSResult {
    if graph.vertices() <= start {
        panic!("start is not in vertex set")
    }
    let mut marked = vec![false; graph.vertices()];
    let mut count = 0;
    let mut stack = Vec::new();
    stack.push(start);
    while let Some(v) = stack.pop() {
        count += 1;
        marked[v] = true;
        for w in graph.adjacent(v) {
            if !marked[w] {
                stack.push(w);
            }
        }
    }

    DFSResult(marked, start, count)
}

/// Runs Depth-First-Search on a [Graph], starting at all *starts*.
///
/// # Panics
///
/// Panics if *start* is bigger than or equal to vertex amount in graph
pub fn dfs_mul<T: Graph>(
    graph: &T,
    starts: impl Iterator<Item = usize>,
) -> DFSMultResult {
    let mut marked = vec![false; graph.vertices()];
    let mut count = 0;
    let mut start_vertices = Vec::new();
    for start in starts {
        start_vertices.push(start);
        let mut stack = Vec::new();
        stack.push(start);
        while let Some(v) = stack.pop() {
            count += 1;
            marked[v] = true;
            for w in graph.adjacent(v) {
                if !marked[w] {
                    stack.push(w);
                }
            }
        }
    }

    DFSMultResult(marked, start_vertices, count)
}

/// The result from running [bfs].
#[must_use = "Running Breadth-First-Search without using the result is meaningless"]
pub struct BFSResult(Vec<bool>, usize, usize);

impl BFSResult {
    /// The marked array
    pub fn marked(&self) -> &Vec<bool> {
        &self.0
    }

    /// The start of the DFS.
    pub fn start(&self) -> usize {
        self.1
    }

    /// The amount of vertices reachable from *start*.
    pub fn amount(&self) -> usize {
        self.2
    }

    /// Indicates whether *w* is reachable from *start* via traversal in the graph.
    pub fn has_path_to(&self, w: usize) -> bool {
        self.0.get(w).is_some_and(|&v| v)
    }
}

/// Runs Breadth-First-Search on a [Graph], starting at *start*.
///
/// # Panics
///
/// Panics if *start* is bigger than or equal to vertex amount in graph
pub fn bfs<T: Graph>(graph: &T, start: usize) -> BFSResult {
    if graph.vertices() <= start {
        panic!("start is not in vertex set")
    }

    let mut marked = vec![false; graph.vertices()];
    let mut count = 0;
    let mut queue = Queue::new();
    queue.enqueue(start);
    while let Some(v) = queue.dequeue() {
        count += 1;
        for w in graph.adjacent(v) {
            if !marked[w] {
                marked[w] = true;
                queue.enqueue(w);
            }
        }
    }

    BFSResult(marked, start, count)
}
