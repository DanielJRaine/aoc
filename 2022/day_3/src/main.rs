#![feature(iter_array_chunks)]

use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::str::Chars;
use ::phf::{OrderedMap, phf_map};
use phf::phf_ordered_map;

static CHAR_TO_PRIORITY_MAP: phf::OrderedMap<char, u32> = phf_ordered_map! {
    'a' => 1,
    'b' => 2,
    'c' => 3,
    'd' => 4,
    'e' => 5,
    'f' => 6,
    'g' => 7,
    'h' => 8,
    'i' => 9,
    'j' => 10,
    'k' => 11,
    'l' => 12,
    'm' => 13,
    'n' => 14,
    'o' => 15,
    'p' => 16,
    'q' => 17,
    'r' => 18,
    's' => 19,
    't' => 20,
    'u' => 21,
    'v' => 22,
    'w' => 23,
    'x' => 24,
    'y' => 25,
    'z' => 26,
    'A' => 26 + 1,
    'B' => 26 + 2,
    'C' => 26 + 3,
    'D' => 26 + 4,
    'E' => 26 + 5,
    'F' => 26 + 6,
    'G' => 26 + 7,
    'H' => 26 + 8,
    'I' => 26 + 9,
    'J' => 26 + 10,
    'K' => 26 + 11,
    'L' => 26 + 12,
    'M' => 26 + 13,
    'N' => 26 + 14,
    'O' => 26 + 15,
    'P' => 26 + 16,
    'Q' => 26 + 17,
    'R' => 26 + 18,
    'S' => 26 + 19,
    'T' => 26 + 20,
    'U' => 26 + 21,
    'V' => 26 + 22,
    'W' => 26 + 23,
    'X' => 26 + 24,
    'Y' => 26 + 25,
    'Z' => 26 + 26,
};

// the Elf that did the packing failed to follow this rule for exactly one item type per rucksack
fn main() {
    let input = read_input();
    // puzzle_one()
    puzzle_two(input)
}

// each rucksack has two large compartments
struct Rucksack {
    comp1: BTreeSet<char>,
    comp2: BTreeSet<char>,
}

impl Rucksack {
    pub fn new(mut self, contents: &str) -> Self {
        // a given rucksack always has the same number of items in each of its two compartments,
        let size = contents.len();
        
        // all items of a given type are meant to go into exactly one of the two compartments
        // so the first half of the characters represent items in the first compartment...
        let comp1_items = &contents[..size/2];
        for item in comp1_items.chars() {
            self.comp1.insert(item);
        }
        
        // while the second half of the characters represent items in the second compartment.
        let comp2_items = &contents[size/2..];
        for item in comp2_items.chars() {
            self.comp2.insert(item);
        }
        
        Self {
            comp1: self.comp1,
            comp2: self.comp2
        }
    }
}

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Can't read file")
}

fn puzzle_one(input: String) {
    let rucksack_contents = input.lines();
    let mut priority_sum = 0;
    for rucksack_contents in rucksack_contents {
        // find the item type that appears in both compartments of each rucksack.
        let rucksack: Rucksack = Rucksack::new(Rucksack{ comp1: BTreeSet::new(), comp2: BTreeSet::new() }, rucksack_contents);
        let mistakes: Vec<&char> = rucksack.comp1.intersection(&rucksack.comp2).collect();
        let mistake = mistakes.first().expect("This rucksack is properly sorted");
        
        // what is the sum of the priorities of those item types?
        priority_sum += CHAR_TO_PRIORITY_MAP.get(&mistake).expect("Invalid char");
    }
    
    println!("{priority_sum}");
}

fn puzzle_two(input: String) {
    let rucksack_contents = input.lines();
    let mut priority_sum = 0;
    
    for elf_group in rucksack_contents.array_chunks::<3>() {
        if let [rs1, rs2, rs3] = elf_group {
            let item_set_1 = BTreeSet::from_iter(rs1.chars());
            let item_set_2 = BTreeSet::from_iter(rs2.chars());
            
            let mut c_vec = vec![];
            for mut c in rs3.chars() {
                c_vec.push(c.clone())
            }
            let mut item_set_3: BTreeSet<&char> = BTreeSet::from_iter(&c_vec);
            
            let intersect_1_2: Vec<&char> = item_set_1.intersection(&item_set_2).collect();
            
            let potential_badges = BTreeSet::from_iter(intersect_1_2);
            let intersect_1_2_3 = potential_badges.intersection(&item_set_3);
            
            let team_badges = intersect_1_2_3.collect::<Vec<&&char>>();
            
            for badge in team_badges {
                priority_sum += CHAR_TO_PRIORITY_MAP.get(&badge).expect("Invalid badge");
            }
        }
    }
    
    println!("{priority_sum}");
}