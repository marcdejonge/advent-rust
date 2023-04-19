use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};
use std::hash::{BuildHasher, Hash};
use std::ops::Add;

use fxhash::FxBuildHasher;
use priority_queue::PriorityQueue;

pub trait SearchGraph {
    type Node: Copy + PartialEq + Eq + Hash;
    type Score: Copy + Default + Eq + Add<Self::Score, Output = Self::Score> + Ord;

    fn neighbours(&self, node: Self::Node) -> Vec<(Self::Node, Self::Score)>;
}

pub trait SearchGraphWithGoal: SearchGraph {
    fn is_goal(&self, node: Self::Node) -> bool;

    fn heuristic(&self, _node: Self::Node) -> Self::Score { Default::default() }
}

fn reconstruct_path<N, H: BuildHasher, S>(
    end_node: N,
    current_status: HashMap<N, (S, Option<N>), H>,
) -> Vec<N>
where
    N: Copy + Eq + Hash,
{
    let mut path = vec![end_node];
    let mut curr = end_node;
    loop {
        if let &(_, Some(next)) = current_status.get(&curr).unwrap() {
            path.push(next);
            curr = next;
        } else {
            break;
        }
    }

    path
}

pub fn a_star_search<G: SearchGraphWithGoal>(
    graph: &G,
    start_node: G::Node,
) -> Option<Vec<G::Node>> {
    let mut current_states = HashMap::with_capacity_and_hasher(1000, FxBuildHasher::default());
    current_states.insert(start_node, (G::Score::default(), None)); // the value is the score + where the node came from
    let mut open_set = PriorityQueue::with_capacity_and_hasher(1000, FxBuildHasher::default());
    open_set.push(start_node, Reverse(G::Score::default())); // open_set uses the f_score as priority

    while let Some((node, _)) = open_set.pop() {
        if graph.is_goal(node) {
            return Some(reconstruct_path(node, current_states));
        }

        let &(node_g_score, _) = current_states.get(&node).unwrap();
        for (neighbour, distance) in graph.neighbours(node) {
            let new_g_score: G::Score = node_g_score + distance;
            let current_state = current_states.get(&neighbour);
            if current_state.is_none() || new_g_score < current_state.unwrap().0 {
                current_states.insert(neighbour, (new_g_score, Some(node)));
                open_set.push(neighbour, Reverse(new_g_score + graph.heuristic(neighbour)));
            }
        }
    }

    return None;
}

pub fn depth_first_search<S, I, FN, FV>(start_state: S, neighbours: FN, mut visit: FV)
where
    S: Clone,
    I: IntoIterator<Item = S>,
    FN: Fn(S) -> I,
    FV: FnMut(S) -> bool,
{
    let mut stack = Vec::new();
    stack.push(start_state);

    while !stack.is_empty() {
        let from_state = stack.pop().unwrap();
        for next_state in neighbours(from_state) {
            if visit(next_state.clone()) {
                stack.push(next_state);
            }
        }
    }
}

pub fn breadth_first_search<S, I, FN, FV>(start_state: S, neighbours: FN, mut visit: FV)
where
    S: Clone,
    I: IntoIterator<Item = S>,
    FN: Fn(S) -> I,
    FV: FnMut(S) -> bool,
{
    let mut queue = VecDeque::new();
    queue.push_back(start_state);

    while !queue.is_empty() {
        let from_state = queue.pop_front().unwrap();
        for next_state in neighbours(from_state) {
            if visit(next_state.clone()) {
                queue.push_back(next_state);
            }
        }
    }
}

pub fn find_max_nonoverlapping_combination<T, S>(
    input: impl Iterator<Item = (u64, T)>,
    get_score: fn(&T) -> S,
    bits_split: u32,
) -> (T, T)
where
    T: Clone,
    S: Copy + Ord + Default + Add<Output = S>,
{
    if bits_split == 0 {
        panic!("bits_split needs to be a positive number")
    }
    let split_mask = (1usize << bits_split) - 1;
    let mut buckets: Vec<Vec<&(u64, T)>> = Vec::with_capacity(1 << bits_split);
    for _ in 0..(1 << bits_split) {
        buckets.push(Vec::with_capacity(32));
    }
    let mut input = input.collect::<Vec<_>>();
    input.sort_by(|l, r| get_score(&r.1).cmp(&get_score(&l.1)));
    input.iter().for_each(|item| buckets[item.0 as usize & split_mask].push(item));

    let mut max: S = Default::default();
    let mut max_items = None;

    for first_ix in 0..buckets.len() {
        for first_item in buckets[first_ix].iter().cloned() {
            for second_ix in (first_ix + 1)..buckets.len() {
                if (first_item.0 as usize & second_ix) != 0 {
                    continue; // Skip any group that won't match anyway
                }

                for second_item in buckets[second_ix].iter().cloned() {
                    let score = get_score(&first_item.1) + get_score(&second_item.1);
                    if score < max {
                        break;
                    }
                    if first_item.0 & second_item.0 == 0 {
                        max = score;
                        max_items = Some((first_item.1.clone(), second_item.1.clone()));
                    }
                }
            }
        }
    }

    max_items.expect("Could not find any combination!")
}
