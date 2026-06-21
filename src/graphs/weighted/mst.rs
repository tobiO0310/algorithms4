use std::cmp::Ordering;

use crate::{
    IndexPriorityQueue, PriorityQueue, UnionFind, WeightedQuickUnionWPC,
    collections::Queue,
    graphs::{
        EdgeWeightedGraph, WeightedEdge, WeightedGraph,
        weighted::MinFloatTotalOrder,
    },
};

/// Holds different Minimum Spanning Tree algorithms
pub struct MST;

#[inline(always)]
fn lazy_prim_visit(
    graph: &WeightedGraph,
    v: usize,
    marked: &mut [bool],
    pq: &mut PriorityQueue<
        WeightedEdge,
        fn(&WeightedEdge, &WeightedEdge) -> Ordering,
    >,
) {
    marked[v] = true;
    for e in graph.adjacent(v) {
        if !marked[e.other(v)] {
            pq.insert(e)
        }
    }
}

#[inline(always)]
fn eager_prim_visit(
    graph: &WeightedGraph,
    v: usize,
    marked: &mut [bool],
    pq: &mut IndexPriorityQueue<MinFloatTotalOrder>,
    edge_to: &mut [Option<WeightedEdge>],
    dist_to: &mut [f64],
) {
    marked[v] = true;
    for e in graph.adjacent(v) {
        let w = e.other(v);
        if marked[w] {
            continue;
        }
        if e.weight() < dist_to[w] {
            edge_to[w] = Some(e);
            dist_to[w] = e.weight();
            if pq.contains(w) {
                pq.change_key(w, MinFloatTotalOrder(dist_to[w])).unwrap();
            } else {
                pq.insert(w, MinFloatTotalOrder(dist_to[w])).unwrap();
            }
        }
    }
}

impl MST {
    /// Finds a minimum spanning tree using a lazy implementation of Prim's Algorithm.
    ///
    /// It has a space complexity of O(*E*) and time complexity of O(*E* log *E*),
    /// where *E* is the amount of edges.
    pub fn lazy_prim(
        graph: &WeightedGraph,
    ) -> (impl Iterator<Item = WeightedEdge>, f64) {
        let mut pq: PriorityQueue<_, fn(&WeightedEdge, &WeightedEdge) -> _> =
            PriorityQueue::with_comparator(|a, b| a.cmp(b).reverse());
        let mut marked = vec![false; graph.vertices()];
        let mut mst = Queue::new();

        lazy_prim_visit(graph, 0, &mut marked, &mut pq);
        while let Some(e) = pq.pop() {
            let v = e.either();
            let w = e.other(v);
            if marked[v] && marked[w] {
                continue;
            }
            mst.enqueue(e);
            if !marked[v] {
                lazy_prim_visit(graph, v, &mut marked, &mut pq);
            }
            if !marked[w] {
                lazy_prim_visit(graph, w, &mut marked, &mut pq);
            }
        }
        let weight = mst.iter().map(|a| a.weight()).sum();

        (mst.into_iter(), weight)
    }

    /// Finds a minimum spanning tree using an eager implementation of Prim's Algorithm.
    ///
    /// It has a space complexity of O(*V*) and time complexity of O(*E* log *V*),
    /// where *E* is the amount of edges.
    pub fn eager_prim(
        graph: &WeightedGraph,
    ) -> (impl Iterator<Item = WeightedEdge>, f64) {
        let mut pq = IndexPriorityQueue::new(graph.vertices());
        let mut marked = vec![false; graph.vertices()];
        let mut edge_to = vec![None; graph.vertices()];
        let mut dist_to = vec![f64::INFINITY; graph.vertices()];

        eager_prim_visit(
            graph,
            0,
            &mut marked,
            &mut pq,
            &mut edge_to,
            &mut dist_to,
        );
        while let Some(v) = pq.pop() {
            eager_prim_visit(
                graph,
                v,
                &mut marked,
                &mut pq,
                &mut edge_to,
                &mut dist_to,
            );
        }
        let mst = edge_to.iter().flatten().copied().collect::<Vec<_>>();
        let weight = mst.iter().map(|a| a.weight()).sum();

        (mst.into_iter(), weight)
    }

    /// Finds a minimum spanning tree using Kruskal's MST Algorithm.
    ///
    /// It has a space complexity of O(*E*) and time complexity of O(*E* log *E*),
    /// where *E* is the amount of edges.
    pub fn kruskal(
        graph: &WeightedGraph,
    ) -> (impl Iterator<Item = WeightedEdge>, f64) {
        let mut mst = Queue::new();
        let mut pq: PriorityQueue<_, fn(&WeightedEdge, &WeightedEdge) -> _> =
            PriorityQueue::with_comparator(|a, b| a.cmp(b).reverse());
        graph.all_edges().for_each(|e| pq.insert(e));
        let mut uf = WeightedQuickUnionWPC::new(graph.vertices());

        while let Some(e) = pq.pop()
            && mst.len() < graph.vertices() - 1
        {
            let v = e.either();
            let w = e.other(v);
            if uf.connected(v, w) {
                continue;
            }
            uf.union(v, w).unwrap();
            mst.enqueue(e);
        }
        let weight = mst.iter().map(|e| e.weight()).sum();

        (mst.into_iter(), weight)
    }
}
