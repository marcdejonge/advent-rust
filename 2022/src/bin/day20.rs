#![feature(test)]

extern crate core;

use advent_lib::day_main;

#[derive(Copy, Clone, Debug)]
struct NumberNode {
    number: i64,
    shift: i32,
    next: usize,
    prev: usize,
}

fn calculate_part1(numbers: &Vec<i64>) -> i64 { calculate(create_nodes(numbers, 1), 1) }

fn calculate_part2(numbers: &Vec<i64>) -> i64 { calculate(create_nodes(numbers, 811589153), 10) }

fn create_nodes(numbers: &[i64], multiply: i64) -> Vec<NumberNode> {
    let mut result = Vec::with_capacity(5000);
    for (ix, number) in numbers.iter().enumerate() {
        let max_ix = (numbers.len() - 1) as i32;
        let mut shift = ((number * multiply) % (max_ix as i64)) as i32;
        if shift > max_ix / 2 {
            shift -= max_ix;
        }
        if shift < -max_ix / 2 {
            shift += max_ix;
        }
        let prev = if ix == 0 { max_ix as usize } else { ix - 1 };
        let next = if ix == max_ix as usize { 0 } else { ix + 1 };
        result.push(NumberNode { number: number * multiply, shift, next, prev });
    }
    result
}

fn apply_shift(nodes: &mut [NumberNode], ix: usize) {
    let node = nodes[ix];
    if node.shift == 0 {
        return; // Nothing to do
    }

    // Remove from the chain
    nodes[node.prev].next = node.next;
    nodes[node.next].prev = node.prev;

    // Find the node after which this should end up
    let mut after_node_ix = ix;
    if node.shift > 0 {
        for _ in 0..node.shift {
            after_node_ix = nodes[after_node_ix].next;
        }
    } else {
        for _ in 0..(-node.shift + 1) {
            after_node_ix = nodes[after_node_ix].prev;
        }
    }

    // Insert this node after the found node
    let prev_ix = after_node_ix;
    let next_ix = nodes[after_node_ix].next;

    nodes[ix].prev = prev_ix;
    nodes[prev_ix].next = ix;

    nodes[ix].next = next_ix;
    nodes[next_ix].prev = ix;
}

struct NodesIterator<'a> {
    nodes: &'a Vec<NumberNode>,
    index: usize,
}

impl<'a> Iterator for NodesIterator<'a> {
    type Item = &'a NumberNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.nodes.get(self.index).expect("Iterations broke");
        self.index = node.next;
        Some(node)
    }
}

fn node_iterate(nodes: &Vec<NumberNode>) -> NodesIterator {
    let zero_ix = nodes
        .iter()
        .enumerate()
        .find(|(_, node)| node.number == 0)
        .map(|x| x.0)
        .expect("Could not find zero node");
    NodesIterator { nodes, index: zero_ix }
}

fn calculate(mut nodes: Vec<NumberNode>, repeat: u32) -> i64 {
    for _ in 0..repeat {
        for ix in 0..nodes.len() {
            apply_shift(&mut nodes, ix);
        }
    }
    node_iterate(&nodes).map(|n| n.number).step_by(1000).take(4).sum()
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 20, example => 3, 1623178306 );
    day_test!( 20 => 18257, 4148032160983 );
}
