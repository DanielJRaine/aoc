use std::{env};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Deref;
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

fn part1() -> Result<()> {
    let input: String = aoc::read_input();
    let mut acc: u32 = 0;
    
    for line in input.lines() {
        // take_until: ? while is_numeric
        let (header, num_str) = line.split_once(':').unwrap();
        let (winning_strs, strs_you_have) = num_str.split_once('|').unwrap();
        
        // parse into i32
        let winning_nums: Vec<i32> = winning_strs
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        
        let mut nums_you_have: Vec<i32> = strs_you_have
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
            // .chars();
        
        nums_you_have.sort();
        
        let mut score = 1u32;
        for num in nums_you_have {
            if winning_nums.contains(&num) {
                score <<= 1;
            }
        }
        
        // account for initial value
        println!("score: {score}");
        score >>= 1;
        acc += score;
        // "Card   1: 33 34 29 52 91  7 31 42  2  6 | 53 52  6 96 42 91  2 23  7 38 90 28 31 51  1 26 33 22 95 34 29 77 32 86  3"
    }
    
    println!("sum: {acc}");
    Ok(())
}

fn check_for_winning_nums(winning_nums: &Vec<i32>, nums_you_have: &Vec<i32>) -> usize {
    let mut score = 1;
    
    for num in nums_you_have {
        if winning_nums.contains(&num) {
            score += 1;
        }
    }
    
    score
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_nums: Vec<i32>,
    nums_you_have: Vec<i32>,
}

fn part2() -> Result<()> {
    let input: String = aoc::read_input();
    let mut cards: Vec<Card> = vec![];
    let mut won_cards: Vec<Card> = vec![];
    let mut acc: u32 = 0;
    
    for (id, line) in input.lines().enumerate() {
        // take_until: ? while is_numeric
        
        let (header, num_str) = line.split_once(':').unwrap();
        
        let (winning_strs, strs_you_have) = num_str.split_once('|').unwrap();
        
        // parse into i32
        let winning_nums: Vec<i32> = winning_strs
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        
        let nums_you_have: Vec<i32> = strs_you_have
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        
        cards.push(Card {
            id,
            winning_nums,
            nums_you_have,
        })
    }
    
    collect_winning_cards(cards, &mut won_cards);
    
    dbg!(&won_cards);
    
    let sum = won_cards.len();
    println!("sum: {sum}");
    Ok(())
}

fn collect_winning_cards<'a>(cards: Vec<Card>, won_cards: &mut Vec<Card>) {
    for card in &cards {
        let card_score = check_for_winning_nums(&card.winning_nums, &card.nums_you_have);
        
        // this should start on the *next* card
        for c in cards.iter().skip(card.id + 1).take(card_score).collect::<Vec<&Card>>() {
            // if c.id == cards.len() { break }
            won_cards.push(c.clone());
            
            // if card.id == cards.len() { return }
            // collect_winning_cards(card, cards, won_cards);
        }
    }
    
    // now, won_cards is populated by the nth round of winning cards. Do it again...
    // todo: break condition?
    // todo: make recursive
    collect_winning_cards(won_cards.clone(), won_cards);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_eq() {
        assert_eq!(1, 1)
    }
}
