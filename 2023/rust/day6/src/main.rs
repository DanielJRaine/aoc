use std::{env};
use regex::{Regex};
use std::collections::HashMap;
use std::fs::read_to_string;

use jane_eyre::Result;
use eyre::{bail, eyre};

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

struct Race {
    time_allowed: u32,
    best_distance: u32,
}

// For each whole millisecond you spend at the beginning of the race holding down the button,
// the boat's speed increases by one millimeter per millisecond.
fn distance(speed: u32, time_allowed: u32) -> u32 {
    let time_charging = speed;
    let time_racing = time_allowed - time_charging;
    
    speed * time_racing
}

fn parse_input(line: &str) {

}

fn part1() -> Result<()> {
    let input: String = aoc::read_input();
    
    let times: String = input
        .lines().next().unwrap()
        .split(":")
        .skip(1)
        .collect();
    
    let times: Vec<u32> = times.split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap()).collect();
    
    let dists: String = input
        .lines().next().unwrap()
        .split(":")
        .skip(1)
        .collect();
    
    let dists: Vec<u32> = dists.split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    dbg!(dists);
    
    let ways_to_win: Vec<u32> = vec![];
    // multiply the number of ways to win in each race together
    let product = ways_to_win.into_iter().reduce(|acc, w| acc * w ).unwrap();
    
    println!("{product}");
    dbg!();
    Ok(())
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
    fn its_going_the_distance() {
        let dist = distance(1, 7);
        assert_eq!(dist, 6);
    }
}
