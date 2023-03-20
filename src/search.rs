use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;

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

#[derive(Clone)]
struct SearchNode<'a, G: SearchGraph<'a>> {
    node: &'a G::Node,
    came_from: Option<Rc<RefCell<SearchNode<'a, G>>>>,
    g_score: G::Score,
    f_score: G::Score,
}

fn reconstruct_path<'a, G: SearchGraph<'a>>(
    ref_node: Rc<RefCell<SearchNode<'a, G>>>,
) -> Vec<&'a G::Node> {
    let mut path = vec![ref_node.borrow().node];
    let mut curr = ref_node;
    loop {
        let next_ref = curr.borrow().came_from.clone();
        if let Some(next) = next_ref {
            path.push(next.borrow().node);
            curr = next
        } else {
            break;
        }
    }

    path.reverse();
    path
}

impl<'a, G: SearchGraph<'a>> PartialEq for SearchNode<'a, G> {
    fn eq(&self, other: &Self) -> bool {
        self.node.eq(other.node)
    }
}

impl<'a, G: SearchGraph<'a>> Eq for SearchNode<'a, G> {}

impl<'a, G: SearchGraph<'a>> PartialOrd for SearchNode<'a, G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl<'a, G: SearchGraph<'a>> Ord for SearchNode<'a, G> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

pub fn a_star_search<'a, G: SearchGraph<'a>>(graph: &'a G) -> Option<Vec<&'a G::Node>> {
    let start_node = graph.start_node();

    let mut all_nodes = HashMap::new();
    all_nodes.insert(
        start_node,
        Rc::new(RefCell::new(SearchNode::<'a, G> {
            node: start_node,
            came_from: None,
            g_score: Default::default(),
            f_score: graph.heuristic(start_node),
        })),
    );
    let mut open_set = BinaryHeap::new();
    open_set.push(all_nodes.get(start_node).cloned().unwrap());

    while let Some(ref_node) = open_set.pop() {
        let node = ref_node.try_borrow().expect("Node was already borrowed");
        if graph.is_goal(node.node) {
            let result = reconstruct_path(ref_node.clone());
            return Some(result);
        }

        for (neighbour, distance) in graph.neighbours(node.node) {
            let curr_search_node = all_nodes.get(neighbour);
            let new_g_score: G::Score = node.g_score + distance;
            if curr_search_node == None || new_g_score < curr_search_node.unwrap().borrow().g_score
            {
                let search_node = Rc::new(RefCell::new(SearchNode {
                    node: neighbour,
                    came_from: Some(ref_node.clone()),
                    g_score: new_g_score,
                    f_score: new_g_score + graph.heuristic(neighbour),
                }));
                all_nodes.insert(neighbour, search_node.clone());
                open_set.push(search_node);
            }
        }
    }

    return None;
}
