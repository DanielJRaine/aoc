use std::borrow::Borrow;
use std::collections::HashSet;
use std::fs;
use itertools::Itertools;
use indexmap::{IndexSet};

fn main() {
    let input = read_input();
    
    puzzle(&input);
}

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Can't read file")
}

fn puzzle(input: &str) {
    // find a sequence of characters with no repeats. At what position (1 indexed?) does this sequence?
    let mut peeky_iter = input.chars().multipeek();
    
    let mut index = 0;
    while peeky_iter.next().is_some() {
        index += 4;
        let peeks = vec![
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
            peeky_iter.peek().unwrap().clone(),
        ];
        
        let set: IndexSet<char> = IndexSet::from_iter(peeks);
        println!("{:?}", &set);
        if set.len() == 14 {
            println!("{}", index);
            break;
        }
    }
}