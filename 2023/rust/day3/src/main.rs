// Secret number of cubes of each color
// Goal: figure out info about the number of cubes
// which games would have been possible
// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
// add up the ids of these games

use std::{env};
use std::collections::HashMap;
use std::iter::Iterator;
use eyre::{bail, eyre};
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

// row, column
type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<GridCell>>,
}

impl Grid {
    fn new (input: &str) -> Grid {
        let lines = input.lines();
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
        }
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
    
    fn expand_part_number(&self, pos: &Position) -> (Position, String) {
        let mut part_number_pos= pos.clone();
        let right = &self.data[pos.0]
            .iter()
            .skip(pos.1)
            .by_ref()
            .take_while(|cell| cell.val.is_numeric())
            .cloned()
            .collect::<Vec<GridCell>>();
        
        let left = &self.data[pos.0]
            .iter()
            .rev()
            .skip(&self.data[pos.0].len() - pos.1)
            .by_ref()
            .take_while(|cell| {
                if cell.val.is_numeric() {
                    part_number_pos = cell.pos.clone();
                    true
                } else { false }
            })
            .cloned()
            .collect::<Vec<GridCell>>();
        
        let mut part_number = String::new();
        for cell in left.iter().rev() {
            part_number.push(cell.val);
        }
        
        for cell in right.iter() {
            part_number.push(cell.val)
        }
        
        (part_number_pos, part_number)
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
    for (column_cursor, c) in line.chars().enumerate() {
        if SYMBOLS.contains(&c) {
            grid_cells.push(GridCell {
                pos: (row_cursor, column_cursor),
                val: c,
            });
        }
    }
    
    grid_cells
}

fn scan_for_symbol(line: &str, symbol: char, row_cursor: usize) -> Vec<GridCell> {
    // Compile a set matching any of our patterns.
    let mut grid_cells: Vec<GridCell> = vec![];
    for (column_cursor, c) in line.chars().enumerate() {
        if symbol.eq(&c) {
            // translate this to positions (x, y) (, i)
            grid_cells.push(GridCell {
                pos: (row_cursor, column_cursor),
                val: c,
            });
        }
    }
    
    grid_cells
}

fn part1<T>() -> Result<()> {
    // build up a matrix? assign coordinates?
    let input: String = aoc::read_input();
    let grid = Grid::new(&input);
    let mut part_numbers: Vec<(Position, String)> = vec![];
    let mut symbol_vec = vec![];
    
    let mut i = 0usize;
    for line in input.lines() {
         symbol_vec.push(scan_for_symbols(line, i));
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
            part_numbers.push(grid.expand_part_number(&up_cell.pos));
        }
        
        if grid.down(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            down_cell = grid.down(symbol_cell.pos).unwrap();
            part_numbers.push(grid.expand_part_number(&down_cell.pos));
        }
        
        if grid.left(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            left_cell = grid.left(symbol_cell.pos).unwrap();
            part_numbers.push(grid.expand_part_number(&left_cell.pos));
        }
        
        if grid.right(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            right_cell = grid.right(symbol_cell.pos).unwrap();
            part_numbers.push(grid.expand_part_number(&right_cell.pos));
        }
        
        // todo: make these chainable
        let (upleft, upright, downleft, downright): (Option<&GridCell>, Option<&GridCell>, Option<&GridCell>, Option<&GridCell>);
        if grid.up(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            upleft = grid.up(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos));
            part_numbers.push(grid.expand_part_number(&upleft.unwrap().pos));
        }
        
        if grid.up(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            upright = grid.up(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos));
            part_numbers.push(grid.expand_part_number(&upright.unwrap().pos));
        }
        
        if grid.down(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            downleft = grid.down(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos));
            part_numbers.push(grid.expand_part_number(&downleft.unwrap().pos));
        }
        
        if grid.down(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            downright = grid.down(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos));
            part_numbers.push(grid.expand_part_number(&downright.unwrap().pos));
        }
    }
    
    // add part numbers
    let acc: Vec<(Position, i32)> = part_numbers
        .iter()
        .map(|(pos, pn)| (*pos, pn.parse::<i32>().unwrap()))
        .collect();
    
    let mut map = HashMap::new();
    for (pos, part_num) in acc {
        map.insert(pos, part_num);
    }
    
    let sum = map.iter()
        .map(|(_, val)| *val)
        .reduce(|sum, val| sum + val)
        .unwrap();
    
    println!("{sum}");
    Ok(())
}

fn part2() -> Result<()> {
    // build up a matrix? assign coordinates?
    let input: String = aoc::read_input();
    let grid = Grid::new(&input);
    let mut part_numbers: Vec<(Position, String)> = vec![];
    
    let mut symbol_vec = vec![];
    
    let mut i = 0usize;
    for line in input.lines() {
        symbol_vec.push(scan_for_symbol(line, '*', i));
        // acc += ?
        i+=1;
    }
    
    let symbol_cells: Vec<&GridCell> = symbol_vec.iter().flatten().collect();
    
    // check for adjacency
    for symbol_cell in symbol_cells {
        // check for adjacency == 2
        let mut adjacency: u32 = 0;
        let mut adjacent_part_numbers: HashMap<Position, String> = HashMap::new();
        
        let (up_cell, down_cell, right_cell, left_cell);
        if grid.up(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            up_cell = grid.up(symbol_cell.pos).unwrap();
            let (pos, part_number) = grid.expand_part_number(&up_cell.pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        if grid.down(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            down_cell = grid.down(symbol_cell.pos).unwrap();
            let (pos, part_number) = grid.expand_part_number(&down_cell.pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        if grid.left(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            left_cell = grid.left(symbol_cell.pos).unwrap();
            let (pos, part_number) = grid.expand_part_number(&left_cell.pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        if grid.right(symbol_cell.pos)
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            right_cell = grid.right(symbol_cell.pos).unwrap();
            let (pos, part_number) = grid.expand_part_number(&right_cell.pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        // todo: make these chainable
        let (upleft, upright, downleft, downright): (Option<&GridCell>, Option<&GridCell>, Option<&GridCell>, Option<&GridCell>);
        if grid.up(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            upleft = grid.up(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos));
            
            let (pos, part_number) = grid.expand_part_number(&upleft.unwrap().pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        if grid.up(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            upright = grid.up(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos));
            let (pos, part_number) = grid.expand_part_number(&upright.unwrap().pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        if grid.down(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            downleft = grid.down(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.left(grid_cell.pos));
            let (pos, part_number) = grid.expand_part_number(&downleft.unwrap().pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        if grid.down(symbol_cell.pos)
            .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos))
            .is_some_and(|symbol_cell| symbol_cell.val.is_numeric()) {
            downright = grid.down(symbol_cell.pos)
                .and_then(|grid_cell: &GridCell| grid.right(grid_cell.pos));
            let (pos, part_number) = grid.expand_part_number(&downright.unwrap().pos);
            
            adjacent_part_numbers.insert(pos, part_number);
        }
        
        // todo: count adjacency
        if adjacent_part_numbers.len() == 2 {
            // multiply two adjacent numbers and add to acc
            dbg!()
        }
        
    }
    
    
    // add part numbers
    let acc: Vec<(Position, i32)> = part_numbers
        .iter()
        .map(|(pos, pn)| (*pos, pn.parse::<i32>().unwrap()))
        .collect();
    
    let mut map = HashMap::new();
    for (pos, part_num) in acc {
        map.insert(pos, part_num);
    }
    
    let sum = map.iter()
        .map(|(_, val)| *val)
        .reduce(|sum, val| sum + val)
        .unwrap();
    
    println!("{sum}");
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
