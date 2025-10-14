#![feature(test)]

use advent_lib::graph_utils::dijkstra_explore;
use advent_lib::iter_utils::IteratorUtils;
use advent_lib::key::Key;
use advent_lib::*;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;
use num::integer::sqrt;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

#[parse_from(map(
    separated_list1(
        line_ending,
        separated_pair(Key::parse, ": ", separated_list1(space1, Key::parse)),
    ),
    parse_graph
))]
struct Input {
    graph: UnGraph<Key, ()>,
}

fn parse_graph(lines: Vec<(Key, Vec<Key>)>) -> UnGraph<Key, ()> {
    let mut graph = UnGraph::<Key, ()>::new_undirected();
    let mut indices = FxHashMap::<Key, NodeIndex>::default();
    for (source, targets) in lines {
        let source_ix = *indices.entry(source).or_insert_with(|| graph.add_node(source));
        for target in targets {
            let target_ix = *indices.entry(target).or_insert_with(|| graph.add_node(target));
            graph.add_edge(source_ix, target_ix, ());
        }
    }
    graph
}

fn calculate_part1(input: &Input) -> usize {
    let graph = &input.graph;

    let mut edge_count: FxHashMap<EdgeIndex, usize> =
        graph.edge_indices().map(|ix| (ix, 0)).collect();

    // The minimum amount of steps to make the edge count representable is the sqrt of the number of nodes
    let min_steps = sqrt(graph.node_count());

    graph
        .node_indices()
        .enumerate()
        .filter_map(move |(count, from)| {
            // Explore all nodes from the current starting node, and count the amount some edge is being used for the shortest path
            let predecessors = dijkstra_explore(&graph, from, |_| 1).predecessor;
            for to in graph.node_indices() {
                if let Some(&from) = predecessors.get(&to) {
                    let connecting = graph.edges_connecting(from, to);
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
            let (left, right) = graph.edge_endpoints(removed_edges[0]).unwrap();
            let limited_graph = graph.filter_map(
                |_, key| Some(*key),
                |ix, _| if removed_edges.contains(&ix) { None } else { Some(()) },
            );

            let left_graph_size = dijkstra(&limited_graph, left, None, |_| 1).len();
            if left_graph_size == graph.node_count() {
                return None; // If we can still found the whole graph, we did not partition it well
            }

            let right_graph_size = dijkstra(&limited_graph, right, None, |_| 1).len();
            assert_eq!(graph.node_count(), left_graph_size + right_graph_size); // The two halves should make up the whole
            Some(left_graph_size * right_graph_size)
        })
        .next()
        .expect("A result could not be found")
}

day_main!(calculate_part1);
day_test!( 25, example => 54 );
day_test!( 25 => 571753 );
