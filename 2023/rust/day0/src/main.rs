// Secret number of cubes of each color
// Goal: figure out info about the number of cubes
// which games would have been possible
// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
// add up the ids of these games

use std::{env};
use std::collections::HashMap;
use std::fs::read_to_string;
use eyre::{bail, eyre};
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

fn part1() -> Result<()> {
    let input: String = read_input();
    for line in input.lines() {
    
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
