// Secret number of cubes of each color
// Goal: figure out info about the number of cubes
// which games would have been possible
// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
// add up the ids of these games

use std::{env};
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::read_to_string;
use std::iter::Iterator;
use eyre::{bail, eyre};
use jane_eyre::owo_colors::OwoColorize;
use jane_eyre::Result;
use regex::{Regex};

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

const SYMBOL_STR: &str = "*/%@+=$#&-";

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

#[derive(Debug, Clone)]
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
                .get(pos.0 - 1)
                .and_then(|row| row.get(pos.1));
            *cell
        }
    }
    
    fn down(&self, pos: Position) -> Option<&GridCell> {
        // &self.data[pos.0][pos.1 + 1]
        // todo!("add array boundary checks");
        if pos.1 > self.data[0].len() {
            None
        } else {
            let cell = &self.data
                .get(pos.0 + 1)
                .and_then(|row| row.get(pos.1));
            *cell
        }
    }
    
    fn left(&self, pos: Position) -> Option<&GridCell> {
        // &self.data[pos.0 - 1][pos.1]
        if pos.1 > self.data[0].len() {
            None
        } else {
            let cell = &self.data
                .get(pos.0)
                .and_then(|row| row.get(pos.1 - 1));
            *cell
        }
    }
    
    fn right(&self, pos: Position) -> Option<&GridCell> {
        // &self.data[pos.0 + 1][pos.1]
        if pos.1 > self.data[0].len() {
            None
        } else {
            let cell = &self.data
                .get(pos.0)
                .and_then(|row| row.get(pos.1 + 1));
            *cell
        }
    }
    
    fn expand_part_number(&self, pos: &Position) -> String {
        // move left until "."
        
        let right = &self.data[pos.0]
            .iter()
            .skip(pos.1)
            .by_ref()
            .take_while(|cell| cell.val.is_numeric())
            .cloned()
            .collect::<Vec<GridCell>>();
        
        let mut part_number = String::new();
        for cell in right.iter() {
            part_number.push(cell.val)
        }
        
        part_number
        // let mut left: &_ = &self.data[pos.0].iter().rev().skip(pos.1+1).take_while(|cell| cell.val.is_numeric()).collect();
        
        
        //  center?
        // move right until "."
        
        // concat
    }
}

#[derive(Debug, Clone)]
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
        // check for adjacent numeric chars
        
        let (up_cell, down_cell, right_cell, left_cell);
        if grid.up(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            up_cell = grid.up(symbol_cell.pos).unwrap();
            dbg!(up_cell);
        }
        
        if grid.down(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            down_cell = grid.down(symbol_cell.pos).unwrap();
            dbg!(down_cell);
        }
        
        if grid.left(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            left_cell = grid.left(symbol_cell.pos).unwrap();
            dbg!(left_cell);
        }
        
        if grid.right(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            right_cell = grid.right(symbol_cell.pos).unwrap();
            dbg!(right_cell);
        }
        
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
    
    #[test]
    fn it_concats_left_right_and_center() {
        let chars = vec!['.', '1', '2', '3', '4', '.', '1'];
        
        let right: Vec<&char> = chars.iter().skip(3).take_while(|char| char.is_numeric()).collect();
        assert_eq!(vec![&'3', &'4'], right);
        
        let mut left: Vec<&char> = chars.iter().rev().skip(3+1).take_while(|char| char.is_numeric()).collect();
        left.reverse();
        assert_eq!(vec![&'1', &'2'], left)
    }
}
