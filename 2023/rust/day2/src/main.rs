// Secret number of cubes of each color
// Goal: figure out info about the number of cubes
// which games would have been possible
// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
// add up the ids of these games

use std::{env};
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::read_to_string;
use eyre::{bail, eyre, Error, ErrReport};
use jane_eyre::owo_colors::OwoColorize;
use jane_eyre::Result;
use regex::{Regex};
use log::debug;

fn main() -> Result<()> {
	jane_eyre::install()?;
	
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		bail!("Must provide part #. Allowed values: {:?}", vec![1,2]);
	}
	
	let part_number = &args[1];
	match part_number.as_str() {
		"1" => part1(),
		"2" => part2(),
		_ => Err(eyre!("Select part 1 or 2")),
	}
}

fn read_input() -> String {
	read_to_string("input.txt").expect("Can't read file")
}


#[derive(Debug)]
struct Game {
	id: u32,
	rounds: Vec<Round>,
}

fn max_scores(game: &Game) -> (u32, u32, u32) {
	(
		game.rounds.iter().max_by_key(|round| round.r).unwrap().r,
		game.rounds.iter().max_by_key(|round| round.g).unwrap().g,
		game.rounds.iter().max_by_key(|round| round.b).unwrap().b,
	)
}

impl FromStr for Game {
	type Err = ErrReport;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut tokens: Vec<&str> = input.splitn(2, ": ").collect();
		let rounds_substr = tokens.pop().unwrap();
		let id_token = tokens.pop().unwrap();
		
		let id_digits: Vec<String> = id_token.chars()
			.filter(|char| char.is_numeric())
			.map(|char| char.to_string())
			.collect();
		let id = id_digits.join("").parse().unwrap();
		
		let rounds_tokens: Vec<&str> = rounds_substr.split(";").collect();
		
		let rounds: Vec<Round> = rounds_tokens.iter()
			.map(|token| Round::from_str(*token).unwrap()).collect();
		
		Ok(Game {
			id,
			rounds,
		})
	}
}

#[derive(Debug)]
struct Round {
	r: u32,
	g: u32,
	b: u32,
}

fn get_digits(input: &str) -> u32 {
	let digits: String = input.chars()
		.filter(|c| c.is_numeric()).collect();
	
	digits.parse::<u32>().unwrap()
}

impl FromStr for Round {
	type Err = ErrReport;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (mut r, mut g, mut b): (u32, u32, u32) = (0, 0, 0);
        
		for token in input.split(",") {
			match token {
				token if token.contains("red") => {
					// "red"
					r = get_digits(token);
				}
				token if token.contains("green") => {
					// "green"
					g = get_digits(token);
				}
				token if token.contains("blue") => {
					// "blue"
					b = get_digits(token);
				}
				_ => {
					dbg!("none");
				}
			};
		}
		
		Ok(Round {
			r,
			g,
			b,
		})
	}
}

fn parse_id(line: &str) -> u32 {
	0
}

fn game_is_possible(game: &Game, spec: Round) -> bool {
	// compare each value in max_scores(game) to see if the are all >= spec
	let (r, g, b) = max_scores(game);
	r <= spec.r &&
	g <= spec.g &&
	b <= spec.b
}

fn part1() -> Result<()> {
	let input: String = read_input();
	let mut acc = 0;
	for line in input.lines() {
		let game = Game::from_str(line).unwrap();
		let spec_round = Round {
			r: 12, g: 13, b: 14
		};
		
		if game_is_possible(&game, spec_round) { acc += game.id; }
	}
	
	println!("{acc}");
	Ok(())
}

fn max_power(game: &Game) -> u32 {
	let (r, g, b) = max_scores(game);
	r * g * b
}

fn part2() -> Result<()> {
	// fewest number of cubes of each color that could have been in the bag
	// to make the game possible?
	let input: String = read_input();
	let mut acc = 0;
	
	for line in input.lines() {
		let game = Game::from_str(line).unwrap();
		acc += max_power(&game)
	}
	
	println!("{acc}");
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	
	// #[test]
	// fn it_parses() {
	//     assert_eq!(18, parse_alphanumeric("oneight"))
	// }
}
