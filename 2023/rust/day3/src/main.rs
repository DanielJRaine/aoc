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
use regex::{Regex, RegexSet};
use log::debug;

use aoc;

const SYMBOLS: [char; 10] = [
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
];

use lazy_static::lazy_static;
lazy_static! {
    static ref REGEXES: [Regex; 10] = [
        Regex::new("[*]").unwrap(),
        Regex::new("[/]").unwrap(),
        Regex::new("[%]").unwrap(),
        Regex::new("[@]").unwrap(),
        Regex::new("[+]").unwrap(),
        Regex::new("[=]").unwrap(),
        Regex::new("[$]").unwrap(),
        Regex::new("[#]").unwrap(),
        Regex::new("[&]").unwrap(),
        Regex::new("[-]").unwrap(),
    ];
}

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
type Position = (usize, usize);

#[derive(Debug)]
struct Grid<'a> {
    data: Vec<Vec<GridCell<'a>>>,
    col_cursor: usize,
    row_cursor: usize,
}

impl Grid<'_> {
    fn new (input: &str) -> Grid {
        let mut data = vec![];
        let mut lines = input.lines();
        for line in lines {
            data.push(line.chars().collect::<Vec<char>>());
        }
        dbg!(data.len());
        Grid {
            data: vec![],
            col_cursor: 0,
            row_cursor: 0,
        }
    }
    
    fn find_adjacent_numbers(pos: Position) -> Vec<GridCell<'static>> {
        vec![]
    }
}

#[derive(Debug)]
struct GridCell<'a> {
    grid: &'a Grid<'a>,
    pos: Position,
    val: char
}

impl GridCell<'_> {
    fn up(&self) -> &GridCell {
        dbg!(&self.grid.data);
        // self.grid.data[*self.pos.0.clone()][*self.pos.1 - 1.clone()];
        todo!()
    }
    fn down(&self) -> &GridCell {
        
        todo!()
    }
    fn left(&self) -> &GridCell {
        
        todo!()
    }
    fn right(&self) -> &GridCell {
        
        todo!()
    }
}

fn scan_for_symbols(line: &str, index: usize) -> Vec<GridCell<'static>> {
    dbg!(line);
    // Compile a set matching any of our patterns.
    
    vec![]
}

fn part1() -> Result<()> {
    // build up a matrix? assign coordinates?
    let input: String = aoc::read_input();
    let grid = Grid::new(&input);
    
    let mut acc = 0;
    let mut symbol_cells = vec![];
    
    let mut i = 0usize;
    for line in input.lines() {
         symbol_cells.push(scan_for_symbols(line, i));
        // acc += ?
        i+=1;
    }
    // check for adjacency
    // find the rest of the part number
    // remove duplicate part numbers
    // add part numbers
    
    
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
