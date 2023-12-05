// Secret number of cubes of each color
// Goal: figure out info about the number of cubes
// which games would have been possible
// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
// add up the ids of these games

use std::{env};
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::read_to_string;
use std::marker::PhantomData;
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
struct Grid<'a, T> {
    data: Vec<Vec<GridCell<'a, T>>>,
    phantom: PhantomData<&'a mut T>,
    col_cursor: usize,
    row_cursor: usize,
}

impl Grid<'_, GridCell<'_, char>> {
    fn new (input: &str) -> Grid<GridCell<char>> {
        let mut data = vec![];
        let mut lines = input.lines();
        for line in lines {
            data.push(line.chars().collect::<Vec<char>>());
        }
        dbg!(data.len());
        Grid {
            data: vec![],
            phantom: Default::default(),
            col_cursor: 0,
            row_cursor: 0,
        }
    }
    
    fn find_adjacent_numbers(pos: Position) -> Vec<GridCell<'static, char>> {
        vec![]
    }
}

#[derive(Debug)]
struct GridCell<'a, T> {
    // grid: &'a Grid<'a>,
    pos: Position,
    val: char,
    phantom: PhantomData<&'a T>
}

impl GridCell<'_, char> {
    fn up(&self) -> &GridCell<char> {
        // dbg!(&self.grid.data);
        // self.grid.data[*self.pos.0.clone()][*self.pos.1 - 1.clone()];
        todo!()
    }
    fn down(&self) -> &GridCell<char> {
        
        todo!()
    }
    fn left(&self) -> &GridCell<char> {
        
        todo!()
    }
    fn right(&self) -> &GridCell<char> {
        
        todo!()
    }
}

fn scan_for_symbols(line: &str, row_cursor: usize) -> Vec<GridCell<'static, char>> {
    // Compile a set matching any of our patterns.
    let mut grid_cells: Vec<GridCell<char>> = vec![];
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    for (column_cursor, c) in line.chars().enumerate() {
        if SYMBOLS.contains(&c) {
            // translate this to positions (x, y) (, i)
            grid_cells.push(GridCell {
                pos: (row_cursor, column_cursor),
                val: c,
                phantom: Default::default(),
            });
            // map.entry(c).or_insert(Vec::new()).push(i);
        }
    }
    
    dbg!(&grid_cells);
    grid_cells
}

fn part1() -> Result<()> {
    // build up a matrix? assign coordinates?
    let input: String = aoc::read_input();
    let grid = Grid::new(&input);
    
    let mut acc = 0;
    let mut symbol_vec = vec![];
    
    let mut i = 0usize;
    for line in input.lines() {
         symbol_vec.push(scan_for_symbols(line, i));
        // acc += ?
        i+=1;
    }
    
    let symbol_cells: Vec<&GridCell<'_, char>> = symbol_vec.iter().flatten().collect();
    // check for adjacency
    // find the rest of the part number
    // remove duplicate part numbers
    // add part numbers
    
    dbg!(&symbol_cells);
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
