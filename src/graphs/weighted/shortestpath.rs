use std::iter::successors;

use crate::{
    IndexPriorityQueue,
    collections::{Queue, Stack},
    graphs::{
        EdgeWeightedGraph, WeightedDiGraph, WeightedDirectedEdge,
        weighted::MinFloatTotalOrder,
    },
};

/// Holds different Shortest Path algorithms
pub struct ShortestPath;

#[inline(always)]
fn dijkstra_relax(
    g: &WeightedDiGraph,
    v: usize,
    edge_to: &mut [Option<WeightedDirectedEdge>],
    dist_to: &mut [f64],
    pq: &mut IndexPriorityQueue<MinFloatTotalOrder>,
) -> Option<()> {
    for e in g.adjacent(v) {
        if e.weight() < 0. {
            return None;
        }
        let w = e.to();
        if dist_to[w] > dist_to[v] + e.weight() {
            dist_to[w] = dist_to[v] + e.weight();
            edge_to[w] = Some(e);
            if pq.contains(w) {
                pq.change_key(w, MinFloatTotalOrder(dist_to[w])).unwrap();
            } else {
                pq.insert(w, MinFloatTotalOrder(dist_to[w])).unwrap();
            }
        }
    }
    Some(())
}

#[inline(always)]
fn find_negative_cycle(
    g: &WeightedDiGraph,
    edge_to: &mut [Option<WeightedDirectedEdge>],
) -> Option<Stack<WeightedDirectedEdge>> {
    let mut spt = WeightedDiGraph::new(g.vertices());
    for e in edge_to.iter().flatten() {
        spt.add_edge(*e);
    }

    let cycle = spt.get_cycle();
    if cycle.is_empty() { None } else { Some(cycle) }
}

#[inline(always)]
fn bellman_ford_relax(
    g: &WeightedDiGraph,
    v: usize,
    edge_to: &mut [Option<WeightedDirectedEdge>],
    dist_to: &mut [f64],
    on_q: &mut [bool],
    queue: &mut Queue<usize>,
    cost: &mut usize,
) -> Result<(), Stack<WeightedDirectedEdge>> {
    for e in g.adjacent(v) {
        let w = e.to();
        if dist_to[w] > dist_to[v] + e.weight() {
            dist_to[w] = dist_to[v] + e.weight();
            edge_to[w] = Some(e);
            if !on_q[w] {
                queue.enqueue(w);
                on_q[w] = true;
            }
        }
        *cost += 1;
        if cost.is_multiple_of(g.vertices())
            && let Some(cycle) = find_negative_cycle(g, edge_to)
        {
            return Err(cycle);
        }
    }
    Ok(())
}

/// Actual cost | Total Estimated Cost
#[derive(Debug, Clone, Copy)]
struct AStarNode(f64);

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }
}
impl Eq for AStarNode {}
impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0).reverse() // make smaller numbers look bigger
    }
}

#[inline(always)]
fn a_star_relax<F>(
    g: &WeightedDiGraph,
    v: usize,
    edge_to: &mut [Option<WeightedDirectedEdge>],
    dist_to: &mut [f64],
    pq: &mut IndexPriorityQueue<AStarNode>,
    heuristic: &F,
) -> Option<()>
where
    F: Fn(usize) -> f64,
{
    for e in g.adjacent(v) {
        if e.weight() < 0. {
            return None;
        }
        let w = e.to();
        if dist_to[w] > dist_to[v] + e.weight() {
            dist_to[w] = dist_to[v] + e.weight();
            edge_to[w] = Some(e);
            if pq.contains(w) {
                pq.change_key(w, AStarNode(dist_to[w] + heuristic(w)))
                    .unwrap();
            } else {
                pq.insert(w, AStarNode(dist_to[w] + heuristic(w))).unwrap();
            }
        }
    }
    Some(())
}

/// The result from running [ShortestPath::dijkstra]
#[must_use = "The result from Dijkstra takes compute, and is useless on its own."]
pub struct DijkstraResult(Vec<Option<WeightedDirectedEdge>>, Vec<f64>);

impl DijkstraResult {
    /// Returns the distance to the given node.
    /// It may be [f64::INFINITY] if there is no path to *v*.
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to vertex amount.
    pub fn dist_to(&self, v: usize) -> f64 {
        self.1[v]
    }

    /// Indicates whether there is a path to *v* from *start*.
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to vertex amount.
    pub fn has_path_to(&self, v: usize) -> bool {
        self.0[v].is_some()
    }

    /// Returns the path to *v* from *start*.
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to vertex amount.
    pub fn path_to(
        &self,
        v: usize,
    ) -> impl Iterator<Item = WeightedDirectedEdge> {
        successors(self.0[v], |e| self.0[e.from()])
    }
}

/// The result from running [ShortestPath::bellman_ford]
#[must_use = "The result from Bellman Ford takes compute, and is useless on its own."]
pub struct BellmanFordResult(
    Vec<Option<WeightedDirectedEdge>>,
    Vec<f64>,
    Option<Stack<WeightedDirectedEdge>>,
);

impl BellmanFordResult {
    /// Returns the distance to the given node.
    /// It may be [f64::INFINITY] if there is no path to *v*.
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to vertex amount.
    pub fn dist_to(&self, v: usize) -> f64 {
        self.1[v]
    }

    /// Indicates whether there is a path to *v* from *start*.
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to vertex amount.
    pub fn has_path_to(&self, v: usize) -> bool {
        self.0[v].is_some()
    }

    /// Indicates whether a negative cycle exists in the graph.
    pub fn has_negative_cycle(&self) -> bool {
        self.2.is_some()
    }

    /// Returns an iterator over a negative cycle in the graph, if any exist.
    pub fn get_negative_cycle(
        &self,
    ) -> Option<impl Iterator<Item = WeightedDirectedEdge>> {
        self.2.as_ref().map(|f| f.iter().copied())
    }

    /// Returns the path to *v* from *start*.
    ///
    /// # Panics
    ///
    /// Panics if *v* is bigger than or equal to vertex amount.
    pub fn path_to(
        &self,
        v: usize,
    ) -> impl Iterator<Item = WeightedDirectedEdge> {
        assert!(
            !self.has_negative_cycle(),
            "This has a negative cycle, invalidating all paths"
        );
        successors(self.0[v], |e| self.0[e.from()])
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }
}

/// The result from running [ShortestPath::a_star]
///
/// This is an iterator, please use it as such :)
#[must_use = "The result from A* takes compute, and is useless on its own."]
pub struct AStarResult(std::vec::IntoIter<WeightedDirectedEdge>);

impl Iterator for AStarResult {
    type Item = WeightedDirectedEdge;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl ExactSizeIterator for AStarResult {}
impl DoubleEndedIterator for AStarResult {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl ShortestPath {
    /// Runs Dijkstra's Algorithm for finding shortest paths on a weighted directed graph.
    ///
    /// The space complexity is O(*V*) and time complexity is O(*E* log *V*) in the worst case,
    /// where *E* is the amount of edges and *V* is the amount of vertices in the graph.
    ///
    /// This function will always return [DijkstraResult],
    /// *if* all weights are nonnegative, else it returns [None].
    ///
    ///  # Panics
    ///
    /// Panics if start is bigger than or equal to `g.vertices()`
    pub fn dijkstra(
        graph: &WeightedDiGraph,
        start: usize,
    ) -> Option<DijkstraResult> {
        let mut edge_to = vec![None; graph.vertices()];
        let mut dist_to = vec![f64::INFINITY; graph.vertices()];
        let mut pq = IndexPriorityQueue::new(graph.vertices());

        dist_to[start] = 0.;

        pq.insert(start, MinFloatTotalOrder(0.)).unwrap();

        while let Some(v) = pq.pop() {
            dijkstra_relax(graph, v, &mut edge_to, &mut dist_to, &mut pq)?;
        }

        Some(DijkstraResult(edge_to, dist_to))
    }

    /// Runs Bellman-Ford Shortest Path Algorithm for finding shortest paths on a weighted directed graph.
    ///
    /// The space complexity is O(*V*) and time complexity is O(*EV*) in the worst case,
    /// where *E* is the amount of edges and *V* is the amount of vertices in the graph.
    ///
    ///  # Panics
    ///
    /// Panics if start is bigger than or equal to `g.vertices()`
    pub fn bellman_ford(
        graph: &WeightedDiGraph,
        start: usize,
    ) -> BellmanFordResult {
        let mut edge_to = vec![None; graph.vertices()];
        let mut dist_to = vec![f64::INFINITY; graph.vertices()];
        let mut on_q = vec![false; graph.vertices()];
        let mut queue = Queue::new();
        let mut cost = 0;
        let mut cycle = None;

        dist_to[start] = 0.;

        queue.enqueue(start);
        on_q[start] = true;

        while let Some(v) = queue.dequeue()
            && cycle.is_none()
        {
            on_q[v] = false;
            if let Err(x) = bellman_ford_relax(
                graph,
                v,
                &mut edge_to,
                &mut dist_to,
                &mut on_q,
                &mut queue,
                &mut cost,
            ) {
                cycle = Some(x);
            }
        }

        BellmanFordResult(edge_to, dist_to, cycle)
    }

    /// Runs A* Algorithm for finding shortest path from *start* to *end* on a weighted directed graph.
    ///
    /// The space complexity is O(*V*) and time complexity is O(*E* log *V*) in the worst case,
    /// where *E* is the amount of edges and *V* is the amount of vertices in the graph.
    ///
    /// This function will always return [AStarResult],
    /// *if* all weights are nonnegative and a path exists, else it returns [None].
    ///
    /// To gurantee A* returns the correct optimal path,
    /// the heuristic function `h(x)` must obey the following inequality:
    /// `h(x) <= w(x, y) + h(y)` for all edges x->y, where `w(x, y)` is the edge x->y's weight.
    ///
    ///  # Panics
    ///
    /// Panics if start is bigger than or equal to `g.vertices()`
    pub fn a_star<F>(
        graph: &WeightedDiGraph,
        start: usize,
        end: usize,
        heuristic: &F,
    ) -> Option<AStarResult>
    where
        F: Fn(usize) -> f64,
    {
        let mut edge_to = vec![None; graph.vertices()];
        let mut dist_to = vec![f64::INFINITY; graph.vertices()];
        let mut pq = IndexPriorityQueue::new(graph.vertices());

        dist_to[start] = 0.;

        pq.insert(start, AStarNode(heuristic(start))).unwrap();

        while let Some(v) = pq.pop() {
            let e: WeightedDirectedEdge = edge_to[v].unwrap();
            debug_assert!(
                dist_to[e.from()] + e.weight() >= dist_to[v],
                "Heuristic function is NOT admissible."
            );
            if v == end {
                break;
            }
            a_star_relax(
                graph,
                v,
                &mut edge_to,
                &mut dist_to,
                &mut pq,
                heuristic,
            )?;
        }

        Some(AStarResult(
            successors(edge_to[end], |e| edge_to[e.from()])
                .collect::<Vec<_>>()
                .into_iter(),
        ))
    }
}
