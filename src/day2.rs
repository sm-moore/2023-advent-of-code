use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
    .unwrap()  // panic on possible file-reading errors
    .lines()  // split the string into an iterator of string slices
    .map(String::from)  // make each slice into a string
    .collect()  // gather them together into a vector
}

fn parse_color(color: &str, handful: &str) -> i32 {
    let captures = Regex::new(format!(r"(\d) {}", color).as_str())
    .unwrap()
    .captures(handful);

    match captures {
        Some(capture) => capture[1].parse::<i32>().unwrap(),
        None => 0,
    }
}

// RGB
fn parse_handful(handful: &str) -> (i32, i32, i32) {
    let red: i32 = parse_color("red", handful);
    let green: i32 = parse_color("green", handful);
    let blue: i32 = parse_color("blue", handful);
    println!("{}", red);
    // 3 blue, 4 red
    // {"blue": 3,
    // "red": 4,
    // "green": 0}
    (0, 0, 0)

}

// 12 red cubes, 13 green cubes, and 14 blue cubes
// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn is_game_possible(line: &String) -> bool {
    let red_total: i32 = 12;
    let green_total: i32 = 13;
    let blue_total: i32 = 14;

    // at any point are there more reds than red_total?
    let handfulls: Vec<&str> = line.split(";").collect();
    true
}

pub fn solution(filename: &str) -> i32 {
    // let mut digits = Vec::new();
    let lines = read_lines(filename);
    // compute the max
    // for line in &lines {
        // digits.push(find_digits_in_line(line));
    // }
    // sum them all together
    // digits.iter().sum()
    5
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    // part1
    #[test]
    fn test_solution() {
        assert_eq!(solution("inputs/day2/test1.txt"), 8);
    }

    #[test]
    fn test_is_game_possible() {
        let stt = String::from("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(is_game_possible(&stt), true);
    }

    #[test]
    fn test_parse_handful(){
        assert_eq!(parse_handful("3 blue, 4 red"), (4, 0, 3))
    }
}