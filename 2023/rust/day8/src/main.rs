#![feature(iter_map_windows)]

use std::{env};
use std::collections::HashMap;
use std::fs::read_to_string;
use eyre::{bail, eyre};
use jane_eyre::Result;
use regex::{Regex};
use aoc;

fn main() -> Result<()> {
    jane_eyre::install()?;
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        color_eyre::eyre::bail!("Must provide part #. Allowed values: {:?}", vec![1,2]);
    }
    
    let part_number = &args[1];
    match part_number.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => Err(color_eyre::eyre::eyre!("Select part 1 or 2")),
    }
}

struct Node {
    id: String,
    children: (String, String)
}

// struct network {}


fn part1() -> Result<()> {
    let input: String = aoc::read_input();
    // let mut nodes: Vec<Node> = vec![];
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (id, children_str) = line.split_once(" = ").unwrap();
        let children: Vec<String> = children_str
            .trim_end_matches(')')
            .split_terminator(['(', ',', ' '])
            .filter(|c| *c != "")
            .map(|c| c.to_string()).collect();
        
        let children = (children[0].to_string(), children[1].to_string());
        nodes.insert(id.to_string(), children);
    }
    
    Ok(())
}

/// returns the number of steps taken until a terminator is reached.
fn traverse(path: [char; 2], network: HashMap<String, (String, String)>, terminator: &str) -> u64 {
    
    dbg!();
    0
}

fn part2() -> Result<()> {
    todo!();
    
    dbg!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_traverses_the_wasteland() {
        let network: HashMap<String, (String,String)> = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string())),
             ("BBB".to_string(), ("DDD".to_string(), "EEE".to_string())),
              ("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string())),
               ("DDD".to_string(), ("DDD".to_string(), "DDD".to_string())),
                ("EEE".to_string(), ("EEE".to_string(), "EEE".to_string())),
                 ("GGG".to_string(), ("GGG".to_string(), "GGG".to_string())),
                  ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);
        traverse(['L','R'], network, "ZZZ");
        assert_eq!(1, 1);
    }
}
