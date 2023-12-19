#![feature(test)]

use std::fmt::{Debug, Formatter, Write};

use enum_map::{Enum, EnumMap};
use rayon::prelude::*;

use advent_lib::day::*;

use crate::Score::*;

struct Day {
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
            f.write_char(match c {
                Card::Joker => '_',
                Card::Two => '2',
                Card::Three => '3',
                Card::Four => '4',
                Card::Five => '5',
                Card::Six => '6',
                Card::Seven => '7',
                Card::Eight => '8',
                Card::Nine => '9',
                Card::Ten => 'T',
                Card::Jack => 'J',
                Card::Queen => 'Q',
                Card::King => 'K',
                Card::Ace => 'A',
            })?
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Enum)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
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

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown card {value}"),
        }
    }
}

fn parse_hand(s: &str) -> [Card; 5] {
    s.chars().take(5).map(Card::from).collect::<Vec<_>>().try_into().unwrap()
}

fn jokers(cards: [Card; 5]) -> [Card; 5] {
    cards.map(|c| if c == Card::Jack { Card::Joker } else { c })
}

fn end_score(mut bets: Vec<(Hand, u64)>) -> u64 {
    bets.sort();
    bets.iter().enumerate().map(|(ix, (_, bet))| (ix as u64 + 1) * bet).sum()
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            bets: lines.map(|line| (parse_hand(&line), line[6..].parse().unwrap())).collect(),
        }
    }

    fn calculate_part1(&self) -> Self::Output {
        end_score(self.bets.par_iter().map(|(c, b)| (Hand::new(*c), *b)).collect())
    }

    fn calculate_part2(&self) -> Self::Output {
        end_score(
            self.bets
                .par_iter()
                .map(|(c, bet)| (Hand::new(jokers(*c)), *bet))
                .collect::<Vec<_>>(),
        )
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 7, example => 6440, 5905 );
    day_test!( 7 => 250602641, 251037509 );

    mod parsing {
        use crate::*;

        #[test]
        fn test_scores() {
            assert_eq!(HighCard, Score::from(&parse_hand("23456")));
            assert_eq!(OnePair, Score::from(&parse_hand("22456")));
            assert_eq!(TwoPair, Score::from(&parse_hand("22446")));
            assert_eq!(ThreeOfAKind, Score::from(&parse_hand("22246")));
            assert_eq!(FullHouse, Score::from(&parse_hand("22244")));
            assert_eq!(FourOfAKind, Score::from(&parse_hand("22224")));
            assert_eq!(FiveOfAKind, Score::from(&parse_hand("22222")));
        }

        #[test]
        fn test_scores_with_jokers() {
            assert_eq!(OnePair, Score::from(&jokers(parse_hand("2345J"))));

            assert_eq!(ThreeOfAKind, Score::from(&jokers(parse_hand("2245J"))));
            assert_eq!(ThreeOfAKind, Score::from(&jokers(parse_hand("2J45J"))));

            assert_eq!(FullHouse, Score::from(&jokers(parse_hand("2244J"))));

            assert_eq!(FourOfAKind, Score::from(&jokers(parse_hand("2JJ4J"))));
            assert_eq!(FourOfAKind, Score::from(&jokers(parse_hand("22J4J"))));
            assert_eq!(FourOfAKind, Score::from(&jokers(parse_hand("2224J"))));

            assert_eq!(FiveOfAKind, Score::from(&jokers(parse_hand("JJJJJ"))));
            assert_eq!(FiveOfAKind, Score::from(&jokers(parse_hand("2JJJJ"))));
            assert_eq!(FiveOfAKind, Score::from(&jokers(parse_hand("22JJJ"))));
            assert_eq!(FiveOfAKind, Score::from(&jokers(parse_hand("222JJ"))));
            assert_eq!(FiveOfAKind, Score::from(&jokers(parse_hand("2222J"))));
        }
    }
}
