mod edge;
mod graphs;
mod mst;
mod shortestpath;
use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

pub use edge::{WeightedDirectedEdge, WeightedEdge};
pub use graphs::{WeightedDiGraph, WeightedGraph};
pub use mst::MST;
pub use shortestpath::{
    AStarResult, BellmanFordResult, DijkstraResult, ShortestPath,
};

/// A graph with weighted edges, may be directed.
pub trait EdgeWeightedGraph<T> {
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

    /// Returns the edges adjacent with vertex *v*
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to `vertices`
    #[must_use]
    fn adjacent(&self, v: usize) -> impl Iterator<Item = T>;

    /// Adds an edge between *v* and *w*.
    ///
    /// # Panics
    ///
    /// Panics if *v* or *w* is bigger than or equal to `vertices`
    fn add_edge(&mut self, edge: T);

    /// Returns an iterator over all edges
    fn all_edges(&self) -> impl Iterator<Item = T>;

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

/// Smaller numbers are given as [Ordering::Greater] (used to find smallest edge weights)
#[derive(Debug, Clone, Copy)]
struct MinFloatTotalOrder(f64);

impl PartialEq for MinFloatTotalOrder {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }
}
impl Eq for MinFloatTotalOrder {}
impl PartialOrd for MinFloatTotalOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MinFloatTotalOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0).reverse() // make it small values appear larger
    }
}
impl Deref for MinFloatTotalOrder {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MinFloatTotalOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
