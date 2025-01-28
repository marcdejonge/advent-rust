#![feature(test)]

use std::fmt::{Debug, Formatter, Write};

use enum_map::{Enum, EnumMap};
use nom::error::ParseError;
use nom::AsBytes;
use nom::Parser;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

use crate::Score::*;
use advent_lib::day_main;
use advent_macros::FromRepr;

#[parse_from(separated_list1(line_ending, separated_pair(parse_hand, space1, u64)))]
struct Input {
    bets: Vec<([Card; 5], u64)>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Hand {
    score: Score,
    cards: [Card; 5],
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.score.fmt(f)?;

        f.write_char('\t')?;
        for c in self.cards {
            f.write_char(c.into())?
        }
        Ok(())
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Hand {
        let score = Score::from(&cards);
        Hand { cards, score }
    }
}

#[repr(u8)]
#[derive(FromRepr, Debug, Eq, PartialEq, Copy, Clone, Enum)]
enum Card {
    Joker = b'*',
    Two = b'2',
    Three = b'3',
    Four = b'4',
    Five = b'5',
    Six = b'6',
    Seven = b'7',
    Eight = b'8',
    Nine = b'9',
    Ten = b'T',
    Jack = b'J',
    Queen = b'Q',
    King = b'K',
    Ace = b'A',
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.into_usize().cmp(&other.into_usize()) }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Score {
    fn from(cards: &[Card; 5]) -> Score {
        let mut counter = EnumMap::<Card, u8>::default();
        for card in cards {
            counter[*card] += 1;
        }

        let jokers = counter[Card::Joker];

        let mut more_iter = counter.iter().filter(|(_, count)| **count > 1);
        let first_card = more_iter.next().map(|(_, &count)| count);
        let second_card = more_iter.next().map(|(_, &count)| count);

        if let Some(second_count) = second_card {
            let first_count = first_card.unwrap(); // First always exists if the second on does

            if first_count == 3 || second_count == 3 {
                match jokers {
                    2 | 3 => FiveOfAKind,
                    _ => FullHouse,
                }
            } else {
                match jokers {
                    2 => FourOfAKind,
                    1 => FullHouse,
                    _ => TwoPair,
                }
            }
        } else if let Some(first_count) = first_card {
            match first_count {
                5 => FiveOfAKind,
                4 => match jokers {
                    1 | 4 => FiveOfAKind,
                    _ => FourOfAKind,
                },
                3 => match jokers {
                    1 | 3 => FourOfAKind,
                    _ => ThreeOfAKind,
                },
                2 => match jokers {
                    1 | 2 => ThreeOfAKind,
                    _ => OnePair,
                },
                _ => panic!("Other values are not possible"),
            }
        } else if jokers == 1 {
            OnePair
        } else {
            HighCard
        }
    }
}

fn parse_hand<I, E>(input: I) -> nom::IResult<I, [Card; 5], E>
where
    E: ParseError<I>,
    I: nom::Input + AsBytes,
{
    nom::combinator::map(
        nom::multi::many_m_n(5, 5, nom_parse_trait::ParseFrom::parse),
        |list| list.try_into().unwrap(),
    )
    .parse(input)
}

fn jokers(cards: [Card; 5]) -> [Card; 5] {
    cards.map(|c| if c == Card::Jack { Card::Joker } else { c })
}

fn end_score(mut bets: Vec<(Hand, u64)>) -> u64 {
    bets.sort();
    bets.iter().enumerate().map(|(ix, (_, bet))| (ix as u64 + 1) * bet).sum()
}

fn calculate_part1(input: &Input) -> u64 {
    end_score(input.bets.par_iter().map(|&(c, bet)| (Hand::new(c), bet)).collect())
}

fn calculate_part2(input: &Input) -> u64 {
    end_score(input.bets.par_iter().map(|&(c, bet)| (Hand::new(jokers(c)), bet)).collect())
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 7, example => 6440, 5905 );
    day_test!( 7 => 250602641, 251037509 );

    mod parsing {
        use crate::*;
        use nom::error::Error;

        fn score_from(hand: &[u8]) -> Score {
            Score::from(&parse_hand::<_, Error<_>>(hand).unwrap().1)
        }

        #[test]
        fn test_scores() {
            assert_eq!(HighCard, score_from(b"23456"));
            assert_eq!(OnePair, score_from(b"22456"));
            assert_eq!(TwoPair, score_from(b"22446"));
            assert_eq!(ThreeOfAKind, score_from(b"22246"));
            assert_eq!(FullHouse, score_from(b"22244"));
            assert_eq!(FourOfAKind, score_from(b"22224"));
            assert_eq!(FiveOfAKind, score_from(b"22222"));
        }

        fn score_from_jokers(hand: &[u8]) -> Score {
            Score::from(&jokers(parse_hand::<_, Error<_>>(hand).unwrap().1))
        }

        #[test]
        fn test_scores_with_jokers() {
            assert_eq!(OnePair, score_from_jokers(b"2345J"));

            assert_eq!(ThreeOfAKind, score_from_jokers(b"2245J"));
            assert_eq!(ThreeOfAKind, score_from_jokers(b"2J45J"));

            assert_eq!(FullHouse, score_from_jokers(b"2244J"));

            assert_eq!(FourOfAKind, score_from_jokers(b"2JJ4J"));
            assert_eq!(FourOfAKind, score_from_jokers(b"22J4J"));
            assert_eq!(FourOfAKind, score_from_jokers(b"2224J"));

            assert_eq!(FiveOfAKind, score_from_jokers(b"JJJJJ"));
            assert_eq!(FiveOfAKind, score_from_jokers(b"2JJJJ"));
            assert_eq!(FiveOfAKind, score_from_jokers(b"22JJJ"));
            assert_eq!(FiveOfAKind, score_from_jokers(b"222JJ"));
            assert_eq!(FiveOfAKind, score_from_jokers(b"2222J"));
        }
    }
}
