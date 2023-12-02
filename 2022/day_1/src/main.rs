use std::fs;

fn main() {
    let total_calories_string = fs::read_to_string("input/calories.txt")
        .expect("Cannot read file");
    
    let elf_calories_strings: Vec<&str> = total_calories_string.as_str().split("\n\n").collect();
    
    let elf_calories: Vec<Vec<&str>> = elf_calories_strings.iter().map(|elf_calories_string| elf_calories_string.lines().collect()).collect();
    
    println!("{:?}", elf_calories);
}

// guesses: 11188