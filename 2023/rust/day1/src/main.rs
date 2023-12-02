use std::{env};
use std::collections::HashMap;
use std::fs::read_to_string;
use eyre::{bail, eyre};
use jane_eyre::owo_colors::OwoColorize;
use jane_eyre::Result;
use regex::{Regex};

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

fn parse_digits(input: &str) -> u32 {
    // let regex = Regex::new(r"[0-9]g").unwrap();
    let digits: Vec<u32> = input.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    let first = digits.clone().drain(..1).next();
    let last = digits.clone().drain(digits.len()-1..).next();
    
    let mut first_char= String::new();
    if let Some(first) = first {
        first_char = first.to_string();
    }
    
    let mut last_char = String::new();
    if let Some(last) = last {
        last_char = last.to_string();
    }
    
    let output = format!("{first_char}{last_char}");
    println!("{output}");
    output.parse().unwrap()
}

fn parse_alphanumeric(line: &str) -> u32 {
    let alphanumeric_patterns =
        Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine")
            .unwrap();
    
    let hashmap = HashMap::from([
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    
    let rev_alphanumeric_patterns =
        Regex::new(r"1|2|3|4|5|6|7|8|9|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin")
            .unwrap();
    
    let rev_hashmap = HashMap::from([
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("eno", "1"),
        ("owt", "2"),
        ("eerht", "3"),
        ("ruof", "4"),
        ("evif", "5"),
        ("xis", "6"),
        ("neves", "7"),
        ("thgie", "8"),
        ("enin", "9"),
    ]);
    
    let first = alphanumeric_patterns.find(line).unwrap().as_str();
    let rev_line = &line.chars().rev().collect::<String>();
    let last = rev_alphanumeric_patterns.find(rev_line).unwrap().as_str();
    let first_num = hashmap.get(first).unwrap();
    let last_num = rev_hashmap.get(last).unwrap();
    
    let output = format!("{first_num}{last_num}");
    // println!("{output}");
    output.parse().unwrap()
}

fn read_input() -> String {
    read_to_string("input.txt").expect("Can't read file")
}

fn part1() -> Result<()> {
    let input: String = read_input();
    let mut acc = 0;
    for line in input.lines() {
        acc += parse_digits(line);
    }
    
    println!("{acc}");
    Ok(())
}

fn part2() -> Result<()> {
    let input: String = read_input();
    let mut acc = 0;
    for line in input.lines() {
        println!("{line}");
        let num = parse_alphanumeric(line);
        println!("{num}");
        acc += num;
        // println!("{acc}");
        println!("\n");
    }
    
    println!("{acc}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_parses_digits() {
        assert_eq!(34, parse_digits("u3hats4"))
    }
    
    #[test]
    fn it_parses() {
        assert_eq!(18, parse_alphanumeric("oneight"))
    }
}
