/*
--- Day 7: Camel Cards ---

Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least
it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island
Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a
large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's
looking for; you're here to figure out why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

After riding a bit across the sands of Desert Island, you can see what look like very large rocks
covering half of the horizon. The Elf explains that the rocks are all along the part of Desert
Island that is directly above Island Island, making it hard to even get there. Normally, they use
big machines to move the rocks and filter the sand, but the machines have broken down because Desert
Island recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts stopped when she asks if you
can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel
Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of
each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start by comparing the first
card in each hand. If these cards are different, the hand with the stronger first card is considered
stronger. If the first card in each hand have the same label, however, then move on to considering
the second card in each hand. If they differ, the hand with the higher second card wins; otherwise,
continue with the third card in each hand, then the fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is
stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third
card is stronger (and both hands have the same first and second card).

Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank.

Find the rank of every hand in your set. What are the total winnings?

--- Part Two ---

To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are
jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards
stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of determining hand type;
for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties
between two hands of the same type, J is always treated as J, not the card it's pretending to be:
JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
*/

#![feature(test)]

use fxhash::{FxBuildHasher, FxHashMap};
use std::fmt::{Debug, Formatter, Write};

use advent_lib::day::{execute_day, ExecutableDay};

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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
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
        let mut counter =
            FxHashMap::<Card, usize>::with_capacity_and_hasher(8, FxBuildHasher::default());
        for card in cards {
            counter.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }

        let jokers = *counter.get(&Card::Joker).unwrap_or(&0);

        match counter.len() {
            1 => Score::FiveOfAKind,
            2 => {
                if jokers > 0 {
                    Score::FiveOfAKind
                } else if counter.iter().any(|c| *c.1 == 4) {
                    Score::FourOfAKind
                } else {
                    Score::FullHouse
                }
            }
            3 => {
                if counter.iter().any(|c| *c.1 == 3) {
                    match jokers {
                        1 | 3 => Score::FourOfAKind,
                        _ => Score::ThreeOfAKind,
                    }
                } else {
                    match jokers {
                        1 => Score::FullHouse,
                        2 => Score::FourOfAKind,
                        _ => Score::TwoPair,
                    }
                }
            }
            4 => match jokers {
                1 | 2 => Score::ThreeOfAKind,
                _ => Score::OnePair,
            },
            5 => match jokers {
                1 => Score::OnePair,
                _ => Score::HighCard,
            },
            _ => panic!("Impossible to reach"),
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

fn replace_joker(cards: [Card; 5]) -> [Card; 5] {
    cards.map(|c| if c == Card::Jack { Card::Joker } else { c })
}

fn end_score(mut bets: Vec<(Hand, u64)>) -> u64 {
    bets.sort();
    bets.iter().for_each(|c| {
        dbg!(c.0);
    });
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
        end_score(self.bets.iter().map(|(c, b)| (Hand::new(*c), *b)).collect())
    }

    fn calculate_part2(&self) -> Self::Output {
        end_score(
            self.bets
                .iter()
                .map(|(c, bet)| (Hand::new(replace_joker(*c)), *bet))
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
            assert_eq!(Score::HighCard, Score::from(&parse_hand("23456")));
            assert_eq!(Score::OnePair, Score::from(&parse_hand("22456")));
            assert_eq!(Score::TwoPair, Score::from(&parse_hand("22446")));
            assert_eq!(Score::ThreeOfAKind, Score::from(&parse_hand("22246")));
            assert_eq!(Score::FullHouse, Score::from(&parse_hand("22244")));
            assert_eq!(Score::FourOfAKind, Score::from(&parse_hand("22224")));
            assert_eq!(Score::FiveOfAKind, Score::from(&parse_hand("22222")));
        }

        #[test]
        fn test_scores_with_jokers() {
            assert_eq!(
                Score::OnePair,
                Score::from(&replace_joker(parse_hand("2345J")))
            );
            assert_eq!(
                Score::ThreeOfAKind,
                Score::from(&replace_joker(parse_hand("2245J")))
            );
            assert_eq!(
                Score::ThreeOfAKind,
                Score::from(&replace_joker(parse_hand("2J45J")))
            );
            assert_eq!(
                Score::FullHouse,
                Score::from(&replace_joker(parse_hand("2244J")))
            );
            assert_eq!(
                Score::FourOfAKind,
                Score::from(&replace_joker(parse_hand("2JJ4J")))
            );
            assert_eq!(
                Score::FourOfAKind,
                Score::from(&replace_joker(parse_hand("2224J")))
            );
            assert_eq!(
                Score::FiveOfAKind,
                Score::from(&replace_joker(parse_hand("222JJ")))
            );
            assert_eq!(
                Score::FiveOfAKind,
                Score::from(&replace_joker(parse_hand("2222J")))
            );
        }
    }
}
