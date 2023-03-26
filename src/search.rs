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

fn reconstruct_path<'a, N, H: BuildHasher, S>(
    end_node: &'a N,
    current_status: HashMap<&'a N, (S, Option<&'a N>), H>,
) -> Vec<&'a N>
where
    N: Eq + Hash,
{
    let mut path = vec![end_node];
    let mut curr = end_node;
    loop {
        if let (_, Some(next)) = current_status.get(curr).unwrap() {
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

    let mut current_states = HashMap::with_capacity_and_hasher(1000, FxBuildHasher::default());
    current_states.insert(start_node, (G::Score::default(), None)); // the value is the score + where the node came from
    let mut open_set = PriorityQueue::with_capacity_and_hasher(1000, FxBuildHasher::default());
    open_set.push(start_node, Reverse(G::Score::default())); // open_set uses the f_score as priority

    while let Some((node, _)) = open_set.pop() {
        if graph.is_goal(node) {
            return Some(reconstruct_path(node, current_states));
        }

        let (node_g_score, _) = *current_states.get(node).unwrap();
        for (neighbour, distance) in graph.neighbours(node) {
            let new_g_score: G::Score = node_g_score + distance;
            let current_state = current_states.get(neighbour);
            if current_state.is_none() || new_g_score < current_state.unwrap().0 {
                current_states.insert(neighbour, (new_g_score, Some(node)));
                open_set.push(neighbour, Reverse(new_g_score + graph.heuristic(neighbour)));
            }
        }
    }

    return None;
}
