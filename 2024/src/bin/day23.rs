#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Name([u8; 2]);

type Nodes = FxHashSet<Name>;
type Graph = FxHashMap<Name, Nodes>;

impl Name {
    fn is_chief(&self) -> bool { self.0[0] == b't' }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
    }
}

struct Day {
    graph: Graph,
}

fn bron_kerbosh(graph: &Graph, r: Nodes, mut p: Nodes, mut x: Nodes, collector: &mut Vec<Nodes>) {
    if p.is_empty() && x.is_empty() {
        collector.push(r.clone());
        return;
    }

    // Find pivot node that has many neighbours
    let u = p.union(&x).max_by_key(|name| graph[name].len()).unwrap();

    let vs = p.difference(&graph[u]).copied().collect::<Vec<_>>();

    for v in vs {
        let mut next_r = r.clone();
        next_r.insert(v);

        bron_kerbosh(
            graph,
            next_r,
            p.intersection(&graph[&v]).copied().collect(),
            x.intersection(&graph[&v]).copied().collect(),
            collector,
        );
        p.remove(&v);
        x.insert(v);
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(
                line_ending,
                separated_pair(
                    take_while_m_n(2, 2, |b: u8| b.is_ascii_alphabetic()),
                    tag(b"-"),
                    take_while_m_n(2, 2, |b: u8| b.is_ascii_alphabetic()),
                ),
            ),
            |pairs: Vec<(&[u8], &[u8])>| {
                let mut graph = Graph::default();
                for (from, to) in pairs {
                    let from = Name(from.try_into().unwrap());
                    let to = Name(to.try_into().unwrap());
                    graph.entry(from).or_insert_with(FxHashSet::default).insert(to);
                    graph.entry(to).or_insert_with(FxHashSet::default).insert(from);
                }
                Day { graph }
            },
        )
    }
    fn calculate_part1(&self) -> Self::Output {
        self.graph
            .iter()
            .filter(|(name, _)| name.is_chief())
            .map(|(name, computer_connections)| {
                IteratorUtils::combinations(computer_connections.iter())
                    .filter(|[snd, third]| {
                        (!snd.is_chief() || snd > &name)
                            && (!third.is_chief() || third > &name)
                            && self.graph.get(snd).unwrap().contains(third)
                    })
                    .count()
            })
            .sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut collector = Vec::new();
        bron_kerbosh(
            &self.graph,
            Nodes::default(),
            self.graph.keys().copied().collect(),
            Nodes::default(),
            &mut collector,
        );

        collector.sort_by_key(|a| a.len());
        let max_set = collector.last().unwrap();
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
