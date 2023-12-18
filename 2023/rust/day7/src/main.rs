#![feature(slice_partition_dedup)]

use aoc;
use eyre::{bail, eyre};
use jane_eyre::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::{env, mem};
use std::fs::read_to_string;
use std::ops::Index;
use itertools::Itertools;
use mem::replace;

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
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

// a Hand is scored based on its type. These are the types arranged from highest scoring to lowest scoring.
#[derive(PartialEq, Debug)]
enum Kind {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    High(Card),
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score() != other.score() {
            // Compare based on the score of the hand type
            self.score().partial_cmp(&other.score())
        } else {
            // If the scores are equal, then we need to compare the cards
            use Kind::*;
            match (self, other) {
                (FiveOfAKind(a), FiveOfAKind(b))
                | (FourOfAKind(a), FourOfAKind(b))
                | (ThreeOfAKind(a), ThreeOfAKind(b))
                | (OnePair(a), OnePair(b))
                | (High(a), High(b)) => {
                    
                    let score1 = CARDS.iter().position(|c| c == a).unwrap();
                    let score2 = CARDS.iter().position(|c| c == b).unwrap();
                    
                    return if score1 < score2 {
                        Some(Ordering::Greater)
                    } else if score1 > score2 {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Equal)
                    }
                },
                (FullHouse(a1, a2), FullHouse(b1, b2))
                | (TwoPair(a1, a2), TwoPair(b1, b2)) => {
                    let a1_pos = CARDS.iter().position(|&c| c == *a1);
                    let a2_pos = CARDS.iter().position(|&c| c == *a2);
                    let b1_pos = CARDS.iter().position(|&c| c == *b1);
                    let b2_pos = CARDS.iter().position(|&c| c == *b2);
                    // Compare the primary card first
                    match a1_pos.partial_cmp(&b1_pos).unwrap().reverse() {
                        Ordering::Equal => Some(a2_pos.partial_cmp(&b2_pos).unwrap().reverse()), // If equal, compare the secondary card
                        other => Some(other),
                    }
                }
                // If we cannot compare the kinds (which should not happen as all cases are covered), return None
                _ => None,
            }
        }
    }
}

impl Kind {
    fn score(&self) -> u64 {
        match self {
            Kind::FiveOfAKind(_) => 7, // Highest score
            Kind::FourOfAKind(_) => 6,
            Kind::FullHouse(_, _) => 5,
            Kind::ThreeOfAKind(_) => 4,
            Kind::TwoPair(_, _) => 3,
            Kind::OnePair(_) => 2,
            Kind::High(_) => 1, // Lowest score
        }
    }
}

#[derive(Default, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    rank: u64,
}

impl Hand {
    pub fn joke(&self) -> Kind {
        // generate all possible new hashsets...? Don't use hashsets at all?
        
        // generate only hashsets using only the chars available in the hand?
        // generate only hashsets using only chars w/ frequency > 1?
        // really, instead of figuring out what the kind *is*,
        // we're trying to figure out the best hand it *could* be.
        
        let mut cards_clone = self.cards.clone();
        // pull out all the J cards, then decide what to do with them?
        let Js: Vec<char> = cards_clone.into_iter().filter(|c| *c == 'J').collect();
        let rest: Vec<char> = cards_clone.into_iter().filter(|c| *c != 'J').collect();
        
        // find highest repeated char
        let card_counts = rest.into_iter().counts();
        let (highest_freq_card, freq) = card_counts.iter()
            .max_by(|(c1, freq1), (c2, freq2)| freq1.cmp(freq2))
            .unwrap();
        
        let new_hand: Vec<&Card> = cards_clone.iter()
            .map(|c| {
                if *c == 'J' { return highest_freq_card }
                c
            }).collect();
        
        dbg!();
        // now call the normal fn kind()?
        
        cards_clone.sort();
        let hand_partition = cards_clone.partition_dedup();
        let (rest, pair_cards) = hand_partition;
        
        // example
        Kind::High('A')
    }
    
    pub fn kind(&self) -> Kind {
        // fixme: so, we might need to generate more hashsets to make this work.
            // generate all possible new hashsets...? Don't use hashsets at all?
            // pull out all the J cards, then decide what to do with them?
            // generate only hashsets using only the chars available in the hand?
            // generate only hashsets using only chars w/ frequency > 1?
            
        // go through your code and find the latest possible place to account for jokers
        
        let card_set = HashSet::from(self.cards);
        match card_set.len() {
            5 => {
                // [A, K, Q, J, T]
                // todo: recursively call kind?
                if self.cards.contains(&'J') {
                    // todo:
                    
                }
                
                let highest_card = self.cards.iter().max_by(|card1, card2| {
                    let score1 = CARDS.iter().position(|c| c == *card1).unwrap();
                    let score2 = CARDS.iter().position(|c| c == *card2).unwrap();
                    
                    if score1 < score2  {
                        return Ordering::Greater;
                    } else if score1 > score2 {
                        return Ordering::Less
                    }
                    Ordering::Equal
                }).unwrap().clone();
                
                return Kind::High(highest_card);
            },
            4 => {
                // [A, A, K, Q, J]
                let mut hand_clone = self.cards.clone();
                hand_clone.sort();
                let hand_partition = hand_clone.partition_dedup();
                let (rest, pair_cards) = hand_partition;
                return Kind::OnePair(*pair_cards.iter().next().unwrap());
            },
            3 => {
                // either two pair or three of a kind
                let mut hand_clone = self.cards.clone();
                hand_clone.sort();
                let hand_partition = hand_clone.partition_dedup();
                let (rest, pair_cards) = hand_partition;
                
                // check if the pair_cards are identical
                if pair_cards[0] == pair_cards[1] {
                // [A, A, A, K, Q]
                    return Kind::ThreeOfAKind(*pair_cards.iter().next().unwrap());
                } else {
                // [A, A, K, K, Q]
                    pair_cards.sort();
                    return Kind::TwoPair(pair_cards[0], pair_cards[1])
                }
                
            },
            2 => {
                // full house (one pair and one trio) OR four_of_a_kind
                // [A, A, K, K, K]
                let mut hand_clone = self.cards.clone();
                hand_clone.sort();
                let hand_partition = hand_clone.partition_dedup();
                let (rest, tuple_cards) = hand_partition;
                
                if tuple_cards.iter().all(|&c| c == tuple_cards[0]) {
                    // [A, A, A, A, K]
                    return Kind::FourOfAKind(*tuple_cards.iter().next().unwrap());
                } else {
                    // [A, A, K, K, Q]
                    tuple_cards.sort();
                    let mut rest = rest.iter();
                    let card1 = rest.next().unwrap();
                    let card2 = rest.next().unwrap();
                    return Kind::FullHouse(*card1, *card2)
                }
            }
            1 => return Kind::FiveOfAKind(*card_set.iter().next().unwrap()),
            0 => unreachable!(),
            _ => unreachable!(),
        }

        // by inference, card_set.len() is between 2-4 (pair, full house,

        return Kind::High('0');
    }

    pub fn score(&mut self) -> u64 {
        self.joke().score()
    }

    pub fn winnings(&self) -> u64 {
        self.bid * self.rank
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.joke().score().eq(&other.joke().score()) {
            return break_tie(self, &other) == Ordering::Equal
        } else {
            false
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.joke().score().eq(&other.joke().score()) {
            break_tie(self, &other)
        } else {
            self.joke().score().cmp(&other.joke().score())
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.joke().score().eq(&other.joke().score()) {
            // todo: return the higher value within each kind, otherwise go to break_tie fn
            // return if self.kind().partial_cmp(&other.kind()).unwrap() == Ordering::Equal {
                Some(break_tie(self, &other))
            // } else {
            //     self.kind().partial_cmp(&other.kind())
            // }
        } else {
            Some(self.joke().score().cmp(&other.joke().score()))
        }
    }
}

fn break_tie(h1: &Hand, h2: &Hand) -> Ordering {
    for i in 0..5 {
        // remember, scores are inverted by priority (higher scores are lower number)
        let card1 = h1.cards[i];
        let score1 = CARDS.iter().position(|c| c == &card1).unwrap();
        
        let card2 = h2.cards[i];
        let score2 = CARDS.iter().position(|c| c == &card2).unwrap();
        
        
        if score1 < score2  {
            return Ordering::Greater;
        } else if score1 > score2 {
            return Ordering::Less
        }
    }
    
    Ordering::Equal
}

fn part1() -> Result<()> {
    let input: String = aoc::read_input();
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        dbg!();
        let (cards, bid) = line.split_once(" ").unwrap();
        
        hands.push(Hand {
            cards: cards.chars().collect::<Vec<char>>().try_into().unwrap(),
            bid: bid.parse::<u64>().unwrap(),
            rank: 0,
        })
    }
    
    hands.sort();
    for hand in hands.iter() {
        println!("{:?} : {:?} : {:?}", &hand.cards, &hand.bid, &hand.kind());
    }
    
    // the rank is the index
    let sum = hands.into_iter()
        .enumerate()
        .fold(0, |acc, (i, h)| {
            return acc + (h.bid * (i + 1) as u64)
        });
    
    // sum the winnings
    println!("{sum}");
    Ok(())
}

fn part2() -> Result<()> {
    let input: String = aoc::read_input();
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        dbg!();
        let (cards, bid) = line.split_once(" ").unwrap();
        
        hands.push(Hand {
            cards: cards.chars().collect::<Vec<char>>().try_into().unwrap(),
            bid: bid.parse::<u64>().unwrap(),
            rank: 0,
        })
    }
    
    hands.sort();
    for hand in hands.iter() {
        println!("{:?} : {:?} : {:?}", &hand.cards, &hand.bid, &hand.joke());
    }
    
    // the rank is the index
    let sum = hands.into_iter()
        .enumerate()
        .fold(0, |acc, (i, h)| {
            return acc + (h.bid * (i + 1) as u64)
        });
    
    // sum the winnings
    println!("{sum}");
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
    fn it_draws_a_full_house() {
        let mut hand = Hand {
            // cards: ['J', 'J', 'Q', 'Q', 'K'],
            cards: ['J', 'J', 'Q', 'Q', 'Q'],
            bid: 0,
            rank: 0,
        };
        
        assert_eq!(hand.kind(), Kind::FullHouse('J', 'Q'));
    }
    
    #[test]
    fn it_draws_three_of_a_kind() {
        let mut hand = Hand {
            cards: ['J', 'J', 'J', 'Q', 'K'],
            bid: 0,
            rank: 0,
        };
        
        assert_eq!(hand.kind(), Kind::ThreeOfAKind('J'));
    }
    
    #[test]
    fn it_draws_two_pair() {
        let mut hand = Hand {
            cards: ['J', 'J', 'Q', 'Q', 'K'],
            bid: 0,
            rank: 0,
        };
        
        // these are sorted alphabetically for now
        assert_eq!(hand.kind(), Kind::TwoPair('J', 'Q'));
    }
    
    #[test]
    fn it_breaks_tie() {
        let mut hand1 = Hand {
            cards: ['A', 'A', 'Q', 'Q', 'K'],
            bid: 0,
            rank: 0,
        };
        
        let mut hand2 = Hand {
            cards: ['J', 'J', 'Q', 'Q', 'K'],
            bid: 0,
            rank: 0,
        };
        
        let mut hand3 = Hand {
            cards: ['A', 'A', 'Q', 'Q', 'K'],
            bid: 0,
            rank: 0,
        };
        
        let mut ace_high = Hand {
            cards: ['2', '3', '4', '5', 'A'],
            bid: 0,
            rank: 0,
        };
        let mut king_high = Hand {
            cards: ['2', '3', '4', '5', 'K'],
            bid: 0,
            rank: 0,
        };
        
        assert_eq!(break_tie(&hand1, &hand2), Ordering::Greater);
        assert_ne!(break_tie(&hand1, &hand2), Ordering::Less);
        assert_eq!(break_tie(&hand1, &hand3), Ordering::Equal);
        assert_eq!(break_tie(&ace_high, &king_high), Ordering::Greater);
    }
    
    #[test]
    fn it_compares_two_of_a_kind() {
        let ace_high = Kind::High('A');
        let king_high = Kind::High('K');
        assert_ne!(ace_high, king_high);
        assert_eq!(ace_high.partial_cmp(&king_high), Some(Ordering::Greater));
        
        let three_aces = Kind::ThreeOfAKind('A');
        let three_kings = Kind::ThreeOfAKind('K');
        let three_more_aces = Kind::ThreeOfAKind('A');
        assert_eq!(three_aces.partial_cmp(&three_kings), Some(Ordering::Greater));
        assert_eq!(three_aces.partial_cmp(&three_more_aces), Some(Ordering::Equal));
    }
}
