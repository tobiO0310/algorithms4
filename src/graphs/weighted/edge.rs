use std::{cmp::Ordering, fmt::Display};

use crate::graphs::WeightedEdge;

/// A weighted undirected edge, which can be compared to other edges on their weights.
#[must_use = "A weighted undirected edge means nothing on its own"]
#[derive(Debug, Clone, Copy)]
pub struct UndirectedEdge(usize, usize, f64);

impl PartialEq for UndirectedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2.total_cmp(&other.2).is_eq()
    }
}
impl Eq for UndirectedEdge {}
impl PartialOrd for UndirectedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for UndirectedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.total_cmp(&other.2)
    }
}

impl Display for UndirectedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{:?} {:.2?}", self.0, self.1, self.2)
    }
}

impl UndirectedEdge {
    /// Create a new undirected weighted edge
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        Self(v, w, weight)
    }
}

impl WeightedEdge for UndirectedEdge {
    fn either(&self) -> usize {
        self.0
    }

    fn other(&self, vertex: usize) -> usize {
        match vertex {
            v if v == self.0 => self.1,
            w if w == self.1 => self.0,
            _ => panic!("Unknown edge"),
        }
    }

    fn weight(&self) -> f64 {
        self.2
    }
}
