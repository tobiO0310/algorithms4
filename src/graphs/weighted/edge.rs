use std::{cmp::Ordering, fmt::Display};

/// A weighted undirected edge, which can be compared to other edges on their weights.
#[must_use = "A weighted undirected edge means nothing on its own"]
#[derive(Debug, Clone, Copy)]
pub struct WeightedEdge(usize, usize, f64);

impl PartialEq for WeightedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.2.total_cmp(&other.2).is_eq()
    }
}
impl Eq for WeightedEdge {}
impl PartialOrd for WeightedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for WeightedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.total_cmp(&other.2)
    }
}

impl Display for WeightedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{:?} {:.2?}", self.0, self.1, self.2)
    }
}

impl WeightedEdge {
    /// Create a new undirected weighted edge
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        Self(v, w, weight)
    }

    /// Returns any end/vertex of this edge
    pub fn either(&self) -> usize {
        self.0
    }

    /// Returns the other end/vertex of this edge
    ///
    /// # Panics
    ///
    /// Panics if the input is neither of the two vertexes in this edge.
    pub fn other(&self, vertex: usize) -> usize {
        match vertex {
            v if v == self.0 => self.1,
            w if w == self.1 => self.0,
            _ => panic!("Unknown edge"),
        }
    }

    /// Get the weight of this edge
    pub fn weight(&self) -> f64 {
        self.2
    }
}

/// A weighted directed edge, which can be compared to other edges on their weights.
#[must_use = "A weighted directed edge means nothing on its own"]
#[derive(Debug, Clone, Copy)]
pub struct WeightedDirectedEdge(usize, usize, f64);

impl PartialEq for WeightedDirectedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.2.total_cmp(&other.2).is_eq()
    }
}
impl Eq for WeightedDirectedEdge {}
impl PartialOrd for WeightedDirectedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for WeightedDirectedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.total_cmp(&other.2)
    }
}

impl Display for WeightedDirectedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}->{:?} {:.2?}", self.0, self.1, self.2)
    }
}

impl WeightedDirectedEdge {
    /// Create a new undirected weighted edge
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        Self(v, w, weight)
    }

    /// Returns the source of this edge
    pub fn from(&self) -> usize {
        self.0
    }

    /// Returns the end of this edge
    pub fn to(&self) -> usize {
        self.1
    }

    /// Get the weight of this edge
    pub fn weight(&self) -> f64 {
        self.2
    }
}
