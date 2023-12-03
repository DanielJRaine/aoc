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

use aoc;

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

// row, column
type Position = (u32, u32);

struct Grid {
    data: Vec<Vec<GridCell>>,
    col_cursor: u32,
    row_cursor: u32,
}

impl Grid {
    fn new (input: &str) -> Grid {
        Grid {
            data: vec![],
            col_cursor: 0,
            row_cursor: 0,
        }
    }
    
    fn find_adjacent_numbers(pos: Position) -> Vec<GridCell> {
        
        vec![]
    }
}

struct GridCell {
    pos: Position,
    val: char
}

fn scan_for_symbols(symbols: [char; 10]) -> Vec<GridCell> {
    vec![]
}

fn part1() -> Result<()> {
    // scan for symbols
    let grid_positions = scan_for_symbols([
        '*',
        '/',
        '%',
        '@',
        '+',
        '=',
        '$',
        '#',
        '&',
        '-',
    ]);
    
    // build up a matrix? assign coordinates?
    // check for adjacency
    // find the rest of the part number
    // remove duplicate part numbers
    // add part numbers
    
    let input: String = aoc::read_input();
    println!("{input}");
    let mut acc = 0;
    for line in input.lines() {
        // acc += ?
    }
    
    // println!("{acc}");
    Ok(())
}

fn part2() -> Result<()> {
    // fewest number of cubes of each color that could have been in the bag
    // to make the game possible?
    let input: String = aoc::read_input();
    let mut acc = 0;
    
    for line in input.lines() {
        // acc += ?
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
