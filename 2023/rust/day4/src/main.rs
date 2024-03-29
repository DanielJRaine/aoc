use std::cell::RefCell;
use std::env;
use std::ops::Deref;
use std::rc::Rc;

use aoc;
use jane_eyre::Result;

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
	let mut score = 0;
	
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
	card_score: usize,
	parent: Option<Rc<RefCell<Card>>>,
	children: Vec<Rc<RefCell<Card>>>,
}

impl Card {
	fn num_children(&self) -> usize {
		self.children.len()
	}
	
	fn total_children(&self) -> usize {
		self.children.iter()
			.fold(0, |acc, card| {
				let child_card = card.borrow();
				acc + 1 + child_card.total_children()
			})
	}
}

#[derive(Debug)]
struct Deck {
	cards: Vec<Rc<RefCell<Card>>>,
}

fn part2() -> Result<()> {
	let input: String = aoc::read_input();
	let mut deck = Deck {
		cards: vec![],
	};
	
	for (id, line) in input.lines().enumerate() {
		let (header, num_str) = line.split_once(':').unwrap();
		
		let (winning_strs, strs_you_have) = num_str.split_once('|').unwrap();
		
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
		
		let card_score = check_for_winning_nums(&winning_nums, &nums_you_have);
		
		deck.cards.push(Rc::new(RefCell::new(Card {
			id: deck.cards.len() + 1,
			winning_nums,
			nums_you_have,
			card_score,
			parent: None,
			children: vec![],
		})))
	}
	
	// We don't care about scores. We only care about the number of winning Cards yielded per Card
	// Each top-level card is a tree
	
	for card_copy in &deck.cards {
		let children: Vec<Rc<RefCell<Card>>> = deck.cards
			.iter()
			.skip(card_copy.borrow().id)
			.take(card_copy.borrow().card_score)
			.cloned()
			.collect();
		
		for child_card in children {
			card_copy.borrow_mut().children.push(child_card.clone());
		}
	}
	
	// Final debug output to check the state of each card
	for card_rc in deck.cards.iter() {
		println!(
			"id: {} \n\
			num_of_children: {}\n\
			",
			card_rc.borrow().id,
			card_rc.borrow().num_children()
		);
	}
	
	// Count the depth of every tree's branches and sum them.
	// 10484 is too low
	let mut sum = 0;
	for card in deck.cards {
		sum += 1 + card.borrow().total_children()
	}
	println!("{sum}");
	Ok(())
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn it_eq() {
		assert_eq!(1, 1)
	}
	
	#[test]
	fn it_finds_winning_numbers() {
		assert_eq!(check_for_winning_nums(&vec![41, 48, 83, 86, 17], &vec![83, 86, 6, 31, 17, 9, 48, 53]), 4)
	}
}
