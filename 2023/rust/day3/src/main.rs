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

const UNICODE_NULL: char = '\0';
const NULL_GRIDCELL: GridCell = GridCell {
    pos: (0,0),
    val: UNICODE_NULL,
};

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
        "1" => part1::<char>(),
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
struct Grid {
    data: Vec<Vec<GridCell>>,
    col_cursor: usize,
    row_cursor: usize,
}

impl Grid {
    fn new (input: &str) -> Grid {
        let mut lines = input.lines();
        let mut data: Vec<Vec<GridCell>> = vec![vec![]];
        // let mut data: Vec<Vec<GridCell>> = vec![vec![GridCell { val: '.', pos: (0,0) }; 140]; 140];
        for (i, line) in lines.enumerate() {
            data.push(vec![]);
            for (j, char) in line.chars().enumerate() {
                dbg!(&char);
                // todo: use push to expand the vecs
                data[i].push(GridCell {
                    pos: (i, j),
                    val: char,
                });
            }
        }
        Grid {
            data,
            col_cursor: 0,
            row_cursor: 0,
        }
    }
    
    fn find_adjacent_numbers(pos: Position) -> Vec<GridCell> {
        vec![]
    }
    
    fn up(&self, pos: Position) -> Option<&GridCell> {
        // todo!("add array boundary checks");
        if pos.1 > self.data[0].len() {
            None
        } else {
            let cell = &self.data
                .get(pos.0)
                .and_then(|row| row.get(pos.1 - 1));
            dbg!(&cell);
            *cell
        }
    }
    
    fn down(&self, pos: Position) -> &GridCell {
        &self.data[pos.0][pos.1 + 1]
    }
    
    fn left(&self, pos: Position) -> &GridCell {
        &self.data[pos.0 - 1][pos.1]
    }
    
    fn right(&self, pos: Position) -> &GridCell {
        &self.data[pos.0 + 1][pos.1]
    }
}

#[derive(Debug)]
struct GridCell {
    pos: Position,
    val: char,
}

fn scan_for_symbols(line: &str, row_cursor: usize) -> Vec<GridCell> {
    // Compile a set matching any of our patterns.
    let mut grid_cells: Vec<GridCell> = vec![];
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    for (column_cursor, c) in line.chars().enumerate() {
        if SYMBOLS.contains(&c) {
            // translate this to positions (x, y) (, i)
            grid_cells.push(GridCell {
                pos: (row_cursor, column_cursor),
                val: c,
            });
            // map.entry(c).or_insert(Vec::new()).push(i);
        }
    }
    
    dbg!(&grid_cells);
    grid_cells
}

fn part1<T>() -> Result<()> {
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
    
    let symbol_cells: Vec<&GridCell> = symbol_vec.iter().flatten().collect();
    
    // check for adjacency
    for symbol_cell in symbol_cells {
        grid.up(symbol_cell.pos);
        grid.down(symbol_cell.pos);
        grid.left(symbol_cell.pos);
        grid.right(symbol_cell.pos);
        
        // todo: make these chainable
        // let upleft = grid.up(symbol_cell.pos)
        //     .left();
        
        // dbg!(upleft);
        
        // let upright = grid
        //     .up(symbol_cell.pos)
        //     .and_then()
        //     .right();
        
        // grid.down(symbol_cell.pos)
        //     .left();
        
        // grid.down(symbol_cell.pos)
        //     .right()
    }
    
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
