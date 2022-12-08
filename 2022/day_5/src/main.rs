#![feature(try_trait_v2_yeet)]

use std::{env, fs, io};
use std::env::{current_dir, set_current_dir};
use std::fmt::Error;
use std::fs::{create_dir, File, Metadata};
use std::path::PathBuf;
use std::io::prelude::*;
use std::ops::Yeet;

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
    
    for line in lines {
        // if starts_with;
        if line.starts_with('$') {
            parse_command(&line[2..]);
        } else {
            grow_tree(&line);
        }
    }
    
    Ok(())
}

fn parse_command(command: &str) {
    println!("parse_command:{}", command);
    let mut command_iter = command.split_whitespace();
    
    match command_iter.next() {
        Some("cd") => cd(command_iter.next().unwrap()),
        Some("ls") => ls(),
        None => println!("None {}", command),
        _ => println!("_ {command}"),
    };
}

fn create_file(file: &str) -> (Metadata, Error) {
    File::create(file);
    let metadata = fs::metadata("foo.txt").expect("read error");
    println!("create_file: {file}");
    println!("{:?}", metadata);
    println!("{:?}", metadata.file_type());
    println!("{:?}", metadata.is_file());
    println!("{:?}", metadata.is_dir());
    (metadata, Error)
}

fn cd(dir: &str) {
    // mkdir(dir);
    println!("cd: {dir}");
    if dir == "/" {
        set_current_dir("root_dir");
    } else {
        set_current_dir(dir);
    }
}

fn mkdir(dir: &str) {
    create_dir(dir);
}

fn ls() {
    let paths = fs::read_dir(current_dir().unwrap()).unwrap();
    
    for path in paths {
        println!("file_name: {}", path.unwrap().path().display())
    }
}

fn grow_tree(line: &str) -> io::Result<(&str, Error)> {
    let mut line_iter = line.split_whitespace();
    match line_iter.next() {
        Some("dir") => mkdir(line_iter.next().unwrap()),
        _ => Ok(create_file(line)),
    }
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