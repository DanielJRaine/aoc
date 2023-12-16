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

fn part1() -> Result<()> {
    let input: String = aoc::read_input();
    for line in input.lines() {
    
    }
    
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
    fn it_eq() {
        assert_eq!(1, 1);
    }
}
