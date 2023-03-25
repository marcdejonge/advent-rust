use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::ops::Add;

use fxhash::FxBuildHasher;
use priority_queue::PriorityQueue;

pub trait SearchGraph<'a> {
    type Node: PartialEq + Eq + Hash;
    type Score: Default + Copy + Eq + Add<Self::Score, Output = Self::Score> + Ord;

    fn neighbours(&self, node: &'a Self::Node) -> Vec<(&'a Self::Node, Self::Score)>;

    fn start_node(&self) -> &'a Self::Node;

    fn is_goal(&self, node: &'a Self::Node) -> bool;

    fn heuristic(&self, _: &'a Self::Node) -> Self::Score {
        Default::default()
    }
}

fn reconstruct_path<'a, N, H: BuildHasher>(
    end_node: &'a N,
    all_node: HashMap<&'a N, &'a N, H>,
) -> Vec<&'a N>
where
    N: Eq + Hash,
{
    let mut path = vec![end_node];
    let mut curr = end_node;
    loop {
        let next_ref = all_node.get(curr);
        if let Some(next) = next_ref {
            path.push(next);
            curr = next;
        } else {
            break;
        }
    }

    path
}

pub fn a_star_search<'a, G: SearchGraph<'a>>(graph: &'a G) -> Option<Vec<&'a G::Node>> {
    let start_node = graph.start_node();

    let mut g_scores = HashMap::with_hasher(FxBuildHasher::default());
    g_scores.insert(start_node, G::Score::default());
    let mut came_from = HashMap::with_hasher(FxBuildHasher::default());
    let mut open_set = PriorityQueue::with_hasher(FxBuildHasher::default());
    open_set.push(start_node, Reverse(G::Score::default()));

    while let Some((node, _)) = open_set.pop() {
        if graph.is_goal(node) {
            return Some(reconstruct_path(node, came_from));
        }

        let node_g_score: G::Score = *g_scores.get(node).unwrap();
        for (neighbour, distance) in graph.neighbours(node) {
            let new_g_score = node_g_score + distance;
            let current_g_score = g_scores.get(neighbour);
            if current_g_score.is_none() || &new_g_score < current_g_score.unwrap() {
                g_scores.insert(neighbour, new_g_score.clone());
                came_from.insert(neighbour, node);
                open_set.push(neighbour, Reverse(new_g_score + graph.heuristic(neighbour)));
            }
        }
    }

    return None;
}
