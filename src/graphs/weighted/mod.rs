mod edge;
mod graphs;
pub use edge::UndirectedEdge;
pub use graphs::WeightedGraph;

/// A weighted edge.
pub trait WeightedEdge {
    /// Returns any end/vertex of this edge
    #[must_use]
    fn either(&self) -> usize;

    /// Returns the other end/vertex of this edge
    ///
    /// # Panics
    ///
    /// Panics if the input is neither of the two vertexes in this edge.
    #[must_use]
    fn other(&self, vertex: usize) -> usize;

    /// Get the weight of this edge
    #[must_use]
    fn weight(&self) -> f64;
}

/// A graph with weighted edges, may be directed.
pub trait EdgeWeightedGraph<T: WeightedEdge> {
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
}
