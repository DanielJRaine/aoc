extern crate core;

use std::env;

use aoc;
use eyre::{bail, eyre};
use jane_eyre::Result;
use regex::Regex;

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

#[derive(Debug, PartialEq)]
struct ResourceMap {
	ranges: Vec<ResourceRange>
}

impl ResourceMap {
	// checks source range, returns correct destination resource
	fn to_destination(&self, resource: usize) -> usize {
		for range in &self.ranges {
			if let Some(destination) = range.to_destination(resource) {
				return destination
			};
		}
		
		// if resource is not mapped, return the self-mapped value
		resource
	}
}

#[derive(Debug, PartialEq)]
struct ResourceRange {
	destination_range_start: usize,
	source_range_start: usize,
	range_length: usize
}

impl ResourceRange {
	fn to_destination(&self, resource: usize) -> Option<usize> {
		// checks source range, returns correct destination resource
		if (self.source_range_start..self.source_range_start+self.range_length).contains(&resource) {
			// calculate the offset from the range start
			let offset = resource - self.source_range_start;
			
			// take the same offset and apply it to the destination range to get the returned resource
			Some(self.destination_range_start + offset)
		} else {
			None
		}
	}
}

fn parse_ranges(input: &str) -> Vec<ResourceRange> {
	let mut ranges = vec![];
	for line in input.trim().lines() {
		let mut iter = line.split_ascii_whitespace()
			.map(|num| num.parse::<usize>().unwrap());
		ranges.push(ResourceRange {
			destination_range_start: iter.next().to_owned().unwrap(),
			source_range_start: iter.next().to_owned().unwrap(),
			range_length: iter.next().to_owned().unwrap(),
		});
	}
	
	ranges
}

fn part1() -> Result<()> {
	// let input: String = aoc::read_input();
	// for line in input.lines() {}
	
	// split on ':'
	
	let seeds_input = "79 14 55 13";
	let seeds: Vec<usize> = seeds_input.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
	
	let seed_to_soil_input = "50 98 2
		52 50 48";
	let seed_to_soil_map = ResourceMap {
		ranges: parse_ranges(seed_to_soil_input)
	};
	
	let soil_to_fertilizer_input = "0 15 37
		37 52 2
		39 0 15";
	let soil_to_fertilizer_map = ResourceMap {
		ranges: parse_ranges(soil_to_fertilizer_input)
	};
	
	let fertilizer_to_water_input = "
		49 53 8
		0 11 42
		42 0 7
		57 7 4";
	let fertilizer_to_water_map = ResourceMap {
		ranges: parse_ranges(fertilizer_to_water_input)
	};
	
	let water_to_light_input = "
		88 18 7
		18 25 70";
	let water_to_light_map = ResourceMap {
		ranges: parse_ranges(water_to_light_input)
	};
	
	let light_to_temperature_input = "
		45 77 23
		81 45 19
		68 64 13";
	let light_to_temperature_map = ResourceMap {
		ranges: parse_ranges(light_to_temperature_input)
	};
	
	let temperature_to_humidity_input = "
		0 69 1
		1 0 69";
	let temperature_to_humidity_map = ResourceMap {
		ranges: parse_ranges(temperature_to_humidity_input)
	};
	
	let humidity_to_location_input = "
		60 56 37
		56 93 4";
	let humidity_to_location_map = ResourceMap {
		ranges: parse_ranges(humidity_to_location_input)
	};
	
	// lowest location id that corresponds to any of the initial seeds
	// we could solve this forwards...
	for seed in seeds {
		// find location
		
	}
	
	// ...or backwards
	// let locations =
	
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
	
	#[test]
	fn parses_ranges() {
		let range = "
		0 15 37
		37 52 2
		";
		
		let solution = vec![
			ResourceRange {
				destination_range_start: 0,
				source_range_start: 15,
				range_length: 37,
			},
			ResourceRange {
				destination_range_start: 37,
				source_range_start: 52,
				range_length: 2,
			},
		];
		
		let attempt = parse_ranges(range);
		debug_assert_eq!(attempt, solution)
	}
	
	#[test]
	fn finds_self_mapped_destination() {
		let res_range = ResourceRange {
			destination_range_start: 50,
			source_range_start: 98,
			range_length: 2,
		};
		
		let res_map = ResourceMap {
			ranges: vec!(res_range)
		};
		
		let source_resource = 1;
		let destination_resource = 1;
		// unmapped
		assert_eq!(res_map.to_destination(source_resource), destination_resource);
		
		// mapped
		assert_ne!(res_map.to_destination(99), 99);
	}
	
	#[test]
	fn finds_destination() {
		let res_range = ResourceRange {
			destination_range_start: 50,
			source_range_start: 98,
			range_length: 2,
		};
		
		let destination_resource = 51;
		let source_resource = 99;
		assert_eq!(res_range.to_destination(source_resource), Some(destination_resource))
	}
}
