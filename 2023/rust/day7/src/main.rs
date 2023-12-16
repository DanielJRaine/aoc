#![feature(slice_partition_dedup)]

use aoc;
use eyre::{bail, eyre};
use jane_eyre::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

fn main() -> Result<()> {
    jane_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        color_eyre::eyre::bail!("Must provide part #. Allowed values: {:?}", vec![1, 2]);
    }

    let part_number = &args[1];
    match part_number.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => Err(color_eyre::eyre::eyre!("Select part 1 or 2")),
    }
}

type Card = char;

const CARDS: [Card; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

// let DECK: HashSet<Card> = HashSet::from(['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']);

// a Hand is scored based on its type. These are the types arranged from highest scoring to lowest scoring.
#[derive(PartialEq, Debug)]
enum Kind {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    TwoPair(Card, Card),
    OnePair(Card),
    High(Card),
}

impl Kind {
    fn score(&self) -> u32 {
        match self {
            Kind::FiveOfAKind(_) => 7, // Highest score
            Kind::FourOfAKind(_) => 6,
            Kind::ThreeOfAKind(_) => 5,
            Kind::FullHouse(_, _) => 4,
            Kind::TwoPair(_, _) => 3,
            Kind::OnePair(_) => 2,
            Kind::High(_) => 1, // Lowest score
        }
    }
}

#[derive(Default)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    rank: u32,
}

impl Hand {
    pub fn kind(&mut self) -> Kind {
        let card_set = HashSet::from(self.cards);
        match card_set.len() {
            5 => {
                // [A, K, Q, J, 10]
                let highest_card = self.cards.iter().max().unwrap().clone();
                return Kind::High(highest_card);
            },
            4 => {
                // [A, A, K, Q, J]
                let mut hand_clone = self.cards.clone();
                let hand_partition = hand_clone.partition_dedup();
                let (rest, pair_cards) = hand_partition;
                return Kind::OnePair(*pair_cards.iter().next().unwrap());
            },
            3 => {
                // either two pair or three of a kind
                // [A, A, A, K, Q]
                let mut hand_clone = self.cards.clone();
                let hand_partition = hand_clone.partition_dedup();
                let (rest, pair_cards) = hand_partition;
                return Kind::ThreeOfAKind(*pair_cards.iter().next().unwrap());
            },
            2 => {
                // full house (one pair and one trio)
                // [A, A, K, K, K]
                let mut hand_clone = self.cards.clone();
                let hand_partition = hand_clone.partition_dedup();
                let (rest, _) = hand_partition;
                let mut rest = rest.iter();
                let card1 = rest.next().unwrap();
                let card2 = rest.next().unwrap();
                return Kind::FullHouse(*card1, *card2)
            }
            1 => return Kind::FiveOfAKind(*card_set.iter().next().unwrap()),
            0 => unreachable!(),
            _ => unreachable!(),
        }

        // by inference, card_set.len() is between 2-4 (pair, full house,

        return Kind::High('0');
    }

    pub fn score(&mut self) -> u32 {
        self.kind().score()
    }

    pub fn winnings(&self) -> u32 {
        self.bid * self.rank
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.rank.cmp(&other.rank));
    }
}

fn part1() -> Result<()> {
    let input: String = aoc::read_input();
    for line in input.lines() {}

    let hands: Vec<Hand> = vec![];
    let mut sum: u32 = 0;
    // sum the winnings
    let sum: u32 = hands.iter().fold(0, |acc, hand| sum + hand.winnings());
    println!("{sum}");

    Ok(())
}

fn part2() -> Result<()> {
    todo!();

    dbg!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_draws_five_of_a_kind() {
        let mut hand = Hand {
            cards: ['A', 'A', 'A', 'A', 'A'],
            bid: 0,
            rank: 0,
        };

        assert_eq!(hand.kind(), Kind::FiveOfAKind('A'));
        assert_ne!(hand.kind(), Kind::FiveOfAKind('K'));
        assert_ne!(hand.kind(), Kind::FourOfAKind('K'));
    }

    #[test]
    fn it_draws_ace_high() {
        let mut hand = Hand {
            cards: ['2', '3', 'A', '4', '5'],
            bid: 0,
            rank: 0,
        };

        assert_eq!(hand.kind(), Kind::High('A'));
        assert_ne!(hand.kind(), Kind::High('K'));
    }

    #[test]
    fn it_draws_one_pair() {
        let mut hand = Hand {
            cards: ['J', 'J', 'A', '4', '5'],
            bid: 0,
            rank: 0,
        };

        assert_eq!(hand.kind(), Kind::OnePair('J'));
        assert_ne!(hand.kind(), Kind::OnePair('4'));
        assert_ne!(hand.kind(), Kind::TwoPair('J', 'A'));
    }

    #[test]
    fn it_draws_three_of_a_kind() {
        let mut hand = Hand {
            cards: ['J', 'J', 'J', '4', '5'],
            bid: 0,
            rank: 0,
        };

        assert_eq!(hand.kind(), Kind::ThreeOfAKind('J'));
    }
    
    #[test]
    fn it_draws_a_full_house() {
        let mut hand = Hand {
            // cards: ['J', 'J', 'Q', 'Q', 'K'],
            cards: ['J', 'J', 'Q', 'Q', 'Q'],
            bid: 0,
            rank: 0,
        };
        
        assert_eq!(hand.kind(), Kind::FullHouse('J', 'Q'));
    }
}
