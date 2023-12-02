use std::{env, fs};
use eyre::{bail, eyre};
use jane_eyre::Result;

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

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Can't read file")
}

fn part1() -> Result<()> {
    dbg!("day 1, part 1");
    Ok(())
}

fn part2() -> Result<()> {
    dbg!("day 1, part 2");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }
}
