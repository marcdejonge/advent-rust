#![feature(test)]

use fxhash::FxHashMap;
use num::integer::sqrt;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use prse_derive::parse;

use advent_lib::day::*;
use advent_lib::graph_utils::dijkstra_explore;
use advent_lib::iter_utils::IteratorUtils;
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
        let mut edge_count: FxHashMap<EdgeIndex, usize> =
            self.graph.edge_indices().map(|ix| (ix, 0)).collect();

        // The minimum amount of steps to make the edge count representable is the sqrt of the number of nodes
        let min_steps = sqrt(self.graph.node_count());

        self.graph
            .node_indices()
            .enumerate()
            .filter_map(move |(count, from)| {
                // Explore all nodes from the current starting node, and count the amount some edge is being used for the shortest path
                let predecessors = dijkstra_explore(&self.graph, from, |_| 1).predecessor;
                for to in self.graph.node_indices() {
                    if let Some(&from) = predecessors.get(&to) {
                        let connecting = self.graph.edges_connecting(from, to);
                        connecting.for_each(|edge| {
                            *edge_count.get_mut(&edge.id()).unwrap() += 1;
                        })
                    }
                }

                if count < min_steps {
                    return None; // Just let it explore more edges before determining the most used edges
                }

                // Find the top 3 most used edges and create a limited graph without those edges
                let removed_edges: [_; 3] =
                    edge_count.iter().top(|(_, a), (_, b)| a.cmp(b)).unwrap().map(|(ix, _)| *ix);
                let (left, right) = self.graph.edge_endpoints(removed_edges[0]).unwrap();
                let limited_graph = self.graph.filter_map(
                    |_, key| Some(*key),
                    |ix, _| if removed_edges.contains(&ix) { None } else { Some(()) },
                );

                let left_graph_size = dijkstra(&limited_graph, left, None, |_| 1).len();
                if left_graph_size == self.graph.node_count() {
                    return None; // If we can still found the whole graph, we did not partition it well
                }

                let right_graph_size = dijkstra(&limited_graph, right, None, |_| 1).len();
                assert_eq!(self.graph.node_count(), left_graph_size + right_graph_size); // The two halves should make up the whole
                Some(left_graph_size * right_graph_size)
            })
            .next()
            .expect("A result could not be found")
    }

    fn calculate_part2(&self) -> Self::Output { 0 } // No part 2!
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 25, example => 54 );
    day_test!( 25 => 571753 );
}
