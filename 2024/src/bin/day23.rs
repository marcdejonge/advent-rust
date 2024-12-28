#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use advent_lib::key::Key;
use advent_macros::parsable;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

type Nodes = FxHashSet<Key>;
type Graph = FxHashMap<Key, Nodes>;

fn is_chief(key: Key) -> bool {
    let raw: usize = key.into();
    ((raw / 36) % 36) == 20 // Second from last character is a 't'
}

#[parsable(map(separated_list1(line_ending, parsable_pair(tag(b"-"))), generate_graph))]
struct Day {
    graph: Graph,
}

fn generate_graph(pairs: Vec<(Key, Key)>) -> Graph {
    let mut graph = Graph::default();
    for (from, to) in pairs {
        graph.entry(from).or_insert_with(FxHashSet::default).insert(to);
        graph.entry(to).or_insert_with(FxHashSet::default).insert(from);
    }
    graph
}

fn max_bron_kerbosh(graph: &Graph, r: Nodes, mut p: Nodes, mut x: Nodes) -> Option<Nodes> {
    if p.is_empty() && x.is_empty() {
        return Some(r.clone());
    }

    // Find pivot node that has many neighbours
    let u = p.union(&x).max_by_key(|name| graph[name].len()).unwrap();

    let vs = p.difference(&graph[u]).copied().collect::<Vec<_>>();

    let mut max = None;
    for v in vs {
        let mut next_r = r.clone();
        next_r.insert(v);

        let next = max_bron_kerbosh(
            graph,
            next_r,
            p.intersection(&graph[&v]).copied().collect(),
            x.intersection(&graph[&v]).copied().collect(),
        );
        if let Some(next) = next {
            if next.len() > max.as_ref().map(|n: &Nodes| n.len()).unwrap_or_default() {
                max = Some(next);
            }
        }
        p.remove(&v);
        x.insert(v);
    }

    max
}

impl ExecutableDay for Day {
    type Output = usize;
    fn calculate_part1(&self) -> Self::Output {
        self.graph
            .iter()
            .filter(|(name, _)| is_chief(**name))
            .map(|(name, computer_connections)| {
                IteratorUtils::combinations(computer_connections.iter())
                    .filter(|[snd, third]| {
                        (!is_chief(**snd) || snd > &name)
                            && (!is_chief(**third) || third > &name)
                            && self.graph.get(snd).unwrap().contains(third)
                    })
                    .count()
            })
            .sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let max_set = max_bron_kerbosh(
            &self.graph,
            Nodes::default(),
            self.graph.keys().copied().collect(),
            Nodes::default(),
        )
        .unwrap();
        println!(
            " ├── Part 2 full answer: {}",
            max_set.iter().sorted().join(",")
        );
        max_set.len()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 23, example1 => 7, 4 );
    day_test!( 23 => 1230, 13 );
}
