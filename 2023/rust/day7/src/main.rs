use std::{env};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use eyre::{bail, eyre};
use jane_eyre::Result;
use regex::{Regex};
use aoc;

fn main() -> Result<()> {
    jane_eyre::install()?;
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        color_eyre::eyre::bail!("Must provide part #. Allowed values: {:?}", vec![1,2]);
    }
    
    let part_number = &args[1];
    match part_number.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => Err(color_eyre::eyre::eyre!("Select part 1 or 2")),
    }
}

type Card = char;

const CARDS: [Card; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

// let DECK: HashSet<Card> = HashSet::from(['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']);

// type
enum Type {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    TwoPair(Card, Card),
    OnePair(Card),
    High(Card), // how to specify this as a CARD. Make CARD into enum as well?
}

struct Hand {
    cards: [Card; 5],
    bid: u32,
    rank: u32,
}

impl Hand {
    pub fn kind() -> Type {
        Type::High('A')
    }
    
    pub fn secondary_ord() -> Option<Ordering> {
        todo!();
    }
    
    pub fn secondary_eq() -> bool {
        todo!();
    }
    
    pub fn winnings(&self) -> u32 {
        self.bid * self.rank
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.rank.cmp(&other.rank))
    }
}

fn part1() -> Result<()> {
    let input: String = aoc::read_input();
    for line in input.lines() {
    
    }
    
    let hands: Vec<Hand> = vec![];
    let mut sum: u32 = 0;
    // sum the winnings
    let sum: u32 = hands.iter()
        .fold(0, |acc, hand| sum + hand.winnings());
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
    fn it_eq() {
        assert_eq!(1, 1);
    }
}
