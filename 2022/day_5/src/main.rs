use std::{env, fs, io};
use std::fmt::Error;
use std::path::PathBuf;
use std::io::prelude::*;

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Can't read file")
}

fn main() -> Result<(), Error> {
    let input = read_input();
    
    puzzle1(&input);
    
    Ok(())
}

// find directories that are good candidates for deletion
// [] determine the total size of each directory
//    sum of the sizes of the files it contains, directly or indirectly.
// [] find all of the directories with a total size of at most 100000
// [] calculate the sum of their total sizes
fn puzzle1(input: &str) -> io::Result<()> {
    env::set_current_dir("./root_dir/");
    
    let lines = input.lines();
    
    let current_dir = env::current_dir()?;
    println!("{:?}", &current_dir);
    
    for line in lines {
        // if starts_with;
        if line.starts_with("$") {
            parse_command(&line[1..])
        }
            // mkdir
            // cd
            // ls (skip to next line?)
        // else
            // touch file (or create file)
    }
    
    Ok(())
}

fn parse_command(command: &str) {
    println!("{command}");
}

fn create_file() {
    todo!()
}

fn cd() {
    todo!()
}

fn mkdir() {
    todo!()
}

fn ls() {
    todo!()
}

fn grow_tree() -> io::Result<()> {
    Ok(())
}

fn traverse_tree() -> io::Result<()> {
    env::set_current_dir("./root_dir/");
    
    let current_dir = env::current_dir()?;
    println!("{:?}", &current_dir);
    
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        // println!("{entry:?}");
        let path = entry.path();
        let file_name = entry.file_name();
        println!("{file_name:?}");
    }
    
    Ok(())
}