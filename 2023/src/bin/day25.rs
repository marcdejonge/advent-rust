#![feature(test)]

use fxhash::FxHashMap;
use petgraph::prelude::*;
use prse_derive::parse;

use advent_lib::day::*;
use advent_lib::key::Key;

struct Day {
    graph: UnGraph<Key, ()>,
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut graph = UnGraph::<Key, ()>::new_undirected();
        let mut indices = FxHashMap::<Key, NodeIndex>::default();
        for line in lines {
            let (source, targets): (Key, Vec<Key>) = parse!(line, "{}: {: :}");
            let source_ix = *indices.entry(source).or_insert_with(|| graph.add_node(source));
            for target in targets {
                let target_ix = *indices.entry(target).or_insert_with(|| graph.add_node(target));
                graph.add_edge(source_ix, target_ix, ());
            }
        }

        Day { graph }
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut path_graph = self.graph.map(|_, key| *key, |_, _| 1.0);
        let mut edge_count: FxHashMap<EdgeIndex, usize> =
            path_graph.edge_indices().map(|ix| (ix, 0)).collect();
        for from in path_graph.node_indices() {
            let paths = petgraph::algo::bellman_ford(&path_graph, from).unwrap();
            for to in path_graph.node_indices() {
                if from != to {
                    let from = paths.predecessors[to.index()].unwrap();
                    let connecting = path_graph.edges_connecting(from, to);
                    connecting.for_each(|edge| {
                        *edge_count.get_mut(&edge.id()).unwrap() += 1;
                    })
                }
            }
        }

        let mut edge_count: Vec<_> = edge_count.into_iter().collect();
        edge_count.sort_by(|(_, a), (_, b)| b.cmp(a));

        let (left, right) = path_graph.edge_endpoints(edge_count[0].0).unwrap();
        edge_count.iter().take(3).for_each(|(ix, _)| {
            path_graph.remove_edge(*ix);
        });

        let left_count = petgraph::algo::dijkstra(&path_graph, left, None, |_| 1).len();
        let right_count = petgraph::algo::dijkstra(&path_graph, right, None, |_| 1).len();

        assert_eq!(self.graph.node_count(), left_count + right_count);

        left_count * right_count
    }

    fn calculate_part2(&self) -> Self::Output { 0 } // No part 2!
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 25, example => 54 );
    day_test!( 25 => 571753 ); // Slow in testing, maybe you want to disable it...
}
