#![feature(test)]
use advent_lib::day::{execute_day, ExecutableDay};
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

struct Day {
    packets: Vec<Packet>,
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            packets: lines
                .filter(|line| line.len() > 0)
                .map(|line| line.parse().unwrap())
                .collect(),
        }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.packets
            .as_slice()
            .chunks(2)
            .enumerate()
            .filter_map(|(ix, packets)| {
                let first = packets.get(0).unwrap();
                let second = packets.get(1).unwrap();
                if first < second {
                    Some(ix + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut packets = self.packets.clone();
        packets.sort();
        let start_ix = packets.binary_search(&"[[2]]".parse().unwrap()).unwrap_err() + 1;
        let end_ix = packets.binary_search(&"[[6]]".parse().unwrap()).unwrap_err() + 2;
        start_ix * end_ix
    }
}

#[derive(Debug, Clone, Eq)]
enum Packet {
    List { items: Vec<Packet> },
    Single { value: u32 },
}

impl Packet {
    fn from_str_peekable(line: &mut Peekable<Chars>) -> Result<Self, String> {
        if line.peek() == Some(&'[') {
            line.next(); // Drop the '['
            let mut items: Vec<Packet> = Vec::new();
            loop {
                if line.peek() != Some(&']') {
                    items.push(Packet::from_str_peekable(line)?);
                }
                let next = line.next();
                match next {
                    Some(',') => {}
                    Some(']') => return Ok(Packet::List { items }),
                    _ => return Err(format!("Expected ',' or ']', but found {:?}", next)),
                }
            }
        } else {
            let mut number_string = String::new();
            while let Some(c) = line.peek() {
                if ('0'..='9').contains(c) {
                    number_string.push(*c);
                    line.next();
                } else {
                    break;
                }
            }
            match number_string.parse::<u32>() {
                Ok(value) => Ok(Packet::Single { value }),
                Err(_) => Err(format!("Expected a number, but found {} ", number_string)),
            }
        }
    }
}

impl From<u32> for Packet {
    fn from(value: u32) -> Self { Packet::Single { value } }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match Packet::from_str_peekable(&mut line.chars().peekable()) {
            Ok(packet) => Ok(packet),
            Err(msg) => Err(format!("{} for {}", msg, line)),
        }
    }
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
            Packet::Single { value: self_value } => match other {
                Packet::Single { value: other_value } => self_value.cmp(other_value),
                Packet::List { items: other_items } => match other_items.first() {
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
            Packet::List { items: self_items } => match other {
                Packet::Single { value: _ } => other.cmp(self).reverse(),
                Packet::List { items: other_items } => {
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

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 13, example => 13, 140 );
    day_test!( 13 => 5843, 26289 );
}
