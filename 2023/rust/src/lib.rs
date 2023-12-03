use std::fs::read_to_string;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_input() -> String {
    read_to_string("./input.txt").expect("Can't read file")
}

pub fn parse_digits(input: &str) -> u32 {
    let digits: String = input.chars()
        .filter(|c| c.is_numeric()).collect();
    
    digits.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
