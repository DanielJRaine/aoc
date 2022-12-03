use std::fs;

fn main() {
    let input_string = read_input();
	
    // let lines: Vec<&str> = input_string.lines().collect();
    let rock = rock();
    let paper = paper();
    let scissors = scissors();
    
    let mut score = 0;
    for line in input_string.lines() {
        let opponent_weapon_symbol = line.chars().nth(0).unwrap();
        
        let opponent_weapon = forge_opponent_weapon(opponent_weapon_symbol);
        
        // Part 1
        // let response_weapon_symbol = line.chars().nth(2).unwrap();
        // let response_weapon = forge_response_weapon(response_weapon_symbol);
        // let outcome = challenge(&opponent_weapon, &response_weapon);
        
        // Part 2
        let response_symbol = line.chars().nth(2).unwrap();
        let (outcome, response_weapon) = respond(opponent_weapon, response_symbol);
        
        println!("{:?}", outcome);
        println!("Weapon {:?}. Score {}", &response_weapon.class, &response_weapon.score);
        score += score_outcome(outcome, response_weapon);
    }
    
    println!("{score}");
}

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Can't read file")
}

fn challenge(opponent_weapon: &Weapon, response_weapon: &Weapon) -> Outcome {
    if opponent_weapon.wins_against == response_weapon.class {
        return Outcome::Lose
    } else if opponent_weapon.loses_against == response_weapon.class {
        return Outcome::Win
    } else if opponent_weapon.class == response_weapon.class {
        return Outcome::Draw
    } else {
        Outcome::Draw
    }
}

fn forge_opponent_weapon(opponent_symbol: char) -> Weapon {
    match opponent_symbol {
        'A' => rock(),
        'B' => paper(),
        'C' => scissors(),
        _ => panic!("Invalid symbol")
    }
}

fn respond(opponent_weapon: Weapon, outcome_symbol: char) -> (Outcome, Weapon) {
    match outcome_symbol {
        'X' => (Outcome::Lose, forge_weapon(opponent_weapon.wins_against)),
        'Y' => (Outcome::Draw, forge_weapon(opponent_weapon.class)),
        'Z' => (Outcome::Win, forge_weapon(opponent_weapon.loses_against)),
        _ => panic!("Invalid symbol")
    }
}

fn forge_response_weapon(response_symbol: char) -> Weapon {
    match response_symbol {
        'X' => rock(),
        'Y' => paper(),
        'Z' => scissors(),
        _ => panic!("Invalid symbol")
    }
}

fn forge_weapon(weapon_class: WeaponClass) -> Weapon {
    match weapon_class {
        WeaponClass::Rock => rock(),
        WeaponClass::Paper => paper(),
        WeaponClass::Scissors => scissors(),
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw
}

#[derive(PartialEq, Debug)]
enum WeaponClass {
    Rock,
    Paper,
    Scissors,
}

struct Weapon {
    class: WeaponClass,
    loses_against: WeaponClass,
    wins_against: WeaponClass,
    score: u32
}

fn score_outcome(outcome: Outcome, weapon: Weapon) -> u32 {
    let result = match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    } + weapon.score;
    
    println!("{result}");
    result
}

fn rock() -> Weapon {
    Weapon {
        class: WeaponClass::Rock,
        loses_against: WeaponClass::Paper,
        wins_against: WeaponClass::Scissors,
        score: 1
    }
}

fn paper() -> Weapon {
    Weapon {
        class: WeaponClass::Paper,
        loses_against: WeaponClass::Scissors,
        wins_against: WeaponClass::Rock,
        score: 2
    }
}

fn scissors() -> Weapon {
    Weapon {
        class: WeaponClass::Scissors,
        loses_against: WeaponClass::Rock,
        wins_against: WeaponClass::Paper,
        score: 3
    }
}