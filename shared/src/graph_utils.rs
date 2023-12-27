use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BinaryHeap;
use std::hash::Hash;

use fxhash::FxHashMap;
use petgraph::algo::Measure;
use petgraph::prelude::*;
use petgraph::visit::{IntoEdges, NodeCount, VisitMap, Visitable};

pub struct PathsResult<N, K> {
    pub scores: FxHashMap<N, K>,
    pub predecessor: FxHashMap<N, N>,
}

impl<N, K> PathsResult<N, K> {
    fn with_capacity(size: usize) -> Self {
        PathsResult {
            scores: FxHashMap::with_capacity_and_hasher(size, Default::default()),
            predecessor: FxHashMap::with_capacity_and_hasher(size, Default::default()),
        }
    }
}

pub fn dijkstra_explore<G, F, K>(
    graph: G,
    start: G::NodeId,
    mut edge_cost: F,
) -> PathsResult<G::NodeId, K>
where
    G: IntoEdges + NodeCount + Visitable,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
{
    let mut visited = graph.visit_map();
    let mut result = PathsResult::<G::NodeId, K>::with_capacity(graph.node_count());
    let mut visit_next = BinaryHeap::new();
    let zero_score = K::default();
    result.scores.insert(start, zero_score);
    visit_next.push(MinScored(zero_score, start));
    while let Some(MinScored(node_score, node)) = visit_next.pop() {
        if visited.is_visited(&node) {
            continue;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            if visited.is_visited(&next) {
                continue;
            }
            let next_score = node_score + edge_cost(edge);
            match result.scores.entry(next) {
                Occupied(ent) => {
                    if next_score < *ent.get() {
                        *ent.into_mut() = next_score;
                        visit_next.push(MinScored(next_score, next));
                        result.predecessor.insert(next, node);
                    }
                }
                Vacant(ent) => {
                    ent.insert(next_score);
                    visit_next.push(MinScored(next_score, next));
                    result.predecessor.insert(next, node);
                }
            }
        }
        visited.visit(node);
    }
    result
}

#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool { self.cmp(other) == Ordering::Equal }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
