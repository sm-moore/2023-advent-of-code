use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
    .unwrap()  // panic on possible file-reading errors
    .lines()  // split the string into an iterator of string slices
    .map(String::from)  // make each slice into a string
    .collect()  // gather them together into a vector
}

pub fn solution(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut part_numbers: Vec<i32> = Vec::new();

    for line in &lines {
    }
    // sum them all together
    part_numbers.iter().sum()
}

pub fn solution2(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut powers: Vec<i32> = Vec::new();

    // for line in &lines {
    //     powers.push();
    // }

    powers.iter().sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    // part1
    #[test]
    fn test_solution() {
        assert_eq!(solution("inputs/day3/test1.txt"), 4361);
    }

    // part2
    // #[test]
    // fn test_solution2() {
    //     assert_eq!(solution2("inputs/day3/test1.txt"), 2286);
    // }

}