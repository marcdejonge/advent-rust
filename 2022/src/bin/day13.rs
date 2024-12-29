#![feature(test)]

use advent_lib::day_main;
use advent_lib::parsing::single;
use advent_macros::parsable;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::IResult;
use nom::Parser;
use std::cmp::Ordering;

fn calculate_part1(input: &Input) -> usize {
    input
        .packets
        .as_slice()
        .chunks(2)
        .enumerate()
        .filter_map(
            |(ix, packets)| {
                if packets[0] < packets[1] {
                    Some(ix + 1)
                } else {
                    None
                }
            },
        )
        .sum()
}

fn calculate_part2(input: &Input) -> usize {
    let mut packets = input.packets.clone();
    packets.sort();
    let two = Packet::List(vec![Packet::List(vec![Packet::Single(2)])]);
    let start_ix = packets.binary_search(&two).unwrap_err() + 1;
    let six = Packet::List(vec![Packet::List(vec![Packet::Single(6)])]);
    let end_ix = packets.binary_search(&six).unwrap_err() + 2;
    start_ix * end_ix
}

#[parsable(separated_list1(many1(line_ending), parse_packet))]
struct Input {
    packets: Vec<Packet>,
}

#[derive(Debug, Clone, Eq)]
enum Packet {
    List(Vec<Packet>),
    Single(u32),
}

fn parse_packet(input: &[u8]) -> IResult<&[u8], Packet> {
    if let Ok((rest, _)) = single(b'[').parse(input) {
        let (rest, packets) = separated_list0(single(b','), parse_packet).parse(rest)?;
        let (rest, _) = single(b']').parse(rest)?;
        Ok((rest, Packet::List(packets)))
    } else {
        map(nom::character::complete::u32, Packet::Single).parse(input)
    }
}

impl From<u32> for Packet {
    fn from(value: u32) -> Self { Packet::Single(value) }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Single(self_value) => match other {
                Packet::Single(other_value) => self_value.cmp(other_value),
                Packet::List(other_items) => match other_items.first() {
                    None => Ordering::Greater,
                    Some(other_value) => {
                        let sub_order = self.cmp(other_value);
                        if sub_order != Ordering::Equal {
                            sub_order
                        } else if other_items.len() > 1 {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    }
                },
            },
            Packet::List(self_items) => match other {
                Packet::Single(_) => other.cmp(self).reverse(),
                Packet::List(other_items) => {
                    let mut self_iter = self_items.iter();
                    let mut other_iter = other_items.iter();
                    loop {
                        if let Some(self_packet) = self_iter.next() {
                            if let Some(other_packet) = other_iter.next() {
                                let sub_order = self_packet.cmp(other_packet);
                                if sub_order != Ordering::Equal {
                                    return sub_order;
                                }
                            } else {
                                return Ordering::Greater;
                            }
                        } else {
                            return if other_iter.next().is_some() {
                                Ordering::Less
                            } else {
                                Ordering::Equal
                            };
                        }
                    }
                }
            },
        }
    }
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 13, example => 13, 140 );
    day_test!( 13 => 5843, 26289 );
}
