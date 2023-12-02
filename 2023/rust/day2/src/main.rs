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


// loop through each game and
struct Game {
    id: u32,
    rounds: Vec<Round>,
    r: u32,
    g: u32,
    b: u32,
}

impl FromStr for Game {
    type Err = ErrReport;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        
        Ok(Game {
            id: 0,
            rounds: vec![Round { r: 1, g: 2, b: 3 }],
            r: 0,
            g: 0,
            b: 0,
        })
    }
}

struct Round {
    r: u32,
    g: u32,
    b: u32,
}

impl FromStr for Round {
    type Err = ErrReport;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        
        Ok(Round {
            r: 0,
            g: 0,
            b: 0,
        })
    }
}

fn parse_id(line: &str) -> u32 {
    0
}

fn parse_game(line: &str) -> Game {
    println!("{}", line);
    
    Game {
        id: parse_id(line),
        rounds: vec![],
        r: 1,
        g: 2,
        b: 3,
    }
}

fn part1() -> Result<()> {
    let input: String = read_input();
    for line in input.lines() {
        let game = parse_game(line);
    }
    
    println!("");
    Ok(())
}

fn part2() -> Result<()> {
    todo!();
    let input: String = read_input();
    
    for line in input.lines() {
    }
    
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
