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
        
        let children = [children[0].to_string(), children[1].to_string()];
        nodes.insert(id.to_string(), children);
    }
    
    let mut steps = 0usize;
    steps = traverse("AAA", &vec![
        0,0,1,0,1,1,1,0,0,0,1,0,1,1,0,1,1,1,0,1,0,1,1,0,1,0,1,0,1,1,1,0,1,1,1,0,1,0,1,0,1,1,0,0,1,1,1,0,1,1,0,1,1,0,0,1,0,1,1,1,0,1,0,1,0,0,1,1,1,0,0,1,1,1,0,1,0,1,1,1,0,1,1,1,0,1,1,1,0,0,0,1,1,1,0,1,1,0,1,1,0,1,0,1,1,0,1,0,1,1,1,0,1,0,1,1,0,1,0,1,0,1,1,1,0,1,0,0,0,1,1,1,0,0,0,1,0,1,1,1,0,1,0,1,1,0,1,0,1,0,1,0,1,1,0,1,1,0,1,1,0,1,0,1,1,1,0,1,1,1,0,1,1,0,1,1,1,0,1,1,0,1,1,0,1,1,1,0,0,1,0,1,1,0,0,0,1,1,0,1,1,0,1,0,1,0,0,0,1,1,0,1,1,0,1,1,1,0,1,1,0,0,1,0,1,1,1,0,1,1,1,0,1,1,0,1,1,0,1,0,1,1,0,1,0,1,1,1,0,1,1,0,1,1,1,0,0,1,1,1,0,1,0,1,0,0,0,1,1,1,0,0,0,1,1,0,0,1,1,0,1,0,1,1,0,1,0,0,0,1,1,1,1
    ], &nodes, "ZZZ", steps);
    
    println!("{steps}");
    Ok(())
}

/// returns the number of steps taken until a terminator is reached.
fn traverse(
    start: &str,
    path: &Vec<usize>,
    network: &HashMap<String, [String; 2]>,
    terminator: &str,
    mut steps: usize,
) -> usize {
    let mut next_step: &str = start;
    
    for step in path {
        steps += 1;
        let children = network.get(next_step).unwrap();
        dbg!();
        next_step = &children[*step];
        if next_step == terminator {
            return steps
        }
    }
    
    return traverse(next_step, path, network, terminator, steps)
}

fn part2() -> Result<()> {
    let input: String = aoc::read_input();
    let mut network = HashMap::new();
    for line in input.lines() {
        let (id, children_str) = line.split_once(" = ").unwrap();
        let children: Vec<String> = children_str
            .trim_end_matches(')')
            .split_terminator(['(', ',', ' '])
            .filter(|c| *c != "")
            .map(|c| c.to_string()).collect();
        
        let children = [children[0].to_string(), children[1].to_string()];
        network.insert(id.to_string(), children);
    }
    
    let starting_nodes = find_starting_nodes(&network);
    
    let mut steps = 0usize;
    println!("{:#?}", &starting_nodes);
    // steps = traverse("AAA", &vec![
    //     0,0,1,0,1,1,1,0,0,0,1,0,1,1,0,1,1,1,0,1,0,1,1,0,1,0,1,0,1,1,1,0,1,1,1,0,1,0,1,0,1,1,0,0,1,1,1,0,1,1,0,1,1,0,0,1,0,1,1,1,0,1,0,1,0,0,1,1,1,0,0,1,1,1,0,1,0,1,1,1,0,1,1,1,0,1,1,1,0,0,0,1,1,1,0,1,1,0,1,1,0,1,0,1,1,0,1,0,1,1,1,0,1,0,1,1,0,1,0,1,0,1,1,1,0,1,0,0,0,1,1,1,0,0,0,1,0,1,1,1,0,1,0,1,1,0,1,0,1,0,1,0,1,1,0,1,1,0,1,1,0,1,0,1,1,1,0,1,1,1,0,1,1,0,1,1,1,0,1,1,0,1,1,0,1,1,1,0,0,1,0,1,1,0,0,0,1,1,0,1,1,0,1,0,1,0,0,0,1,1,0,1,1,0,1,1,1,0,1,1,0,0,1,0,1,1,1,0,1,1,1,0,1,1,0,1,1,0,1,0,1,1,0,1,0,1,1,1,0,1,1,0,1,1,1,0,0,1,1,1,0,1,0,1,0,0,0,1,1,1,0,0,0,1,1,0,0,1,1,0,1,0,1,1,0,1,0,0,0,1,1,1,1
    // ], &network, "ZZZ", steps);
    //
    // println!("{steps}");
    //
    // dbg!();
    Ok(())
}

fn find_starting_nodes(network: &HashMap<String, [String; 2]>) -> Vec<&String> {
    let keys: Vec<&String> = network
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect();
    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_traverses_the_wasteland() {
        let network: HashMap<String, [String; 2]> = HashMap::from([
            ("AAA".to_string(), ["BBB".to_string(), "CCC".to_string()]),
             ("BBB".to_string(), ["DDD".to_string(), "EEE".to_string()]),
              ("CCC".to_string(), ["ZZZ".to_string(), "GGG".to_string()]),
               ("DDD".to_string(), ["DDD".to_string(), "DDD".to_string()]),
                ("EEE".to_string(), ["EEE".to_string(), "EEE".to_string()]),
                 ("GGG".to_string(), ["GGG".to_string(), "GGG".to_string()]),
                  ("ZZZ".to_string(), ["ZZZ".to_string(), "ZZZ".to_string()]),
        ]);
        
        let mut steps = 0;
        assert_eq!(traverse("AAA", &vec![1,0], &network, "ZZZ", steps), 2);
    }
    
    #[test]
    fn it_traverses_the_desert() {
        let network: HashMap<String, [String; 2]> = HashMap::from([
            ("AAA".to_string(), ["BBB".to_string(), "BBB".to_string()]),
            ("BBB".to_string(), ["AAA".to_string(), "ZZZ".to_string()]),
            ("ZZZ".to_string(), ["ZZZ".to_string(), "ZZZ".to_string()]),
        ]);
        
        let mut steps = 0;
        assert_eq!(traverse("AAA", &vec![0,0,1], &network, "ZZZ", steps), 6);
    }
}
