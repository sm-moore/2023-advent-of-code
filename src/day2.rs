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
    let captures = Regex::new(format!(r"(\d+) {}", color).as_str())
    .unwrap()
    .captures(handful);

    // println!("{:?}", captures);
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
    (red, green, blue)
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn is_game_possible(line: &str) -> bool {
    let red_total: i32 = 12;
    let green_total: i32 = 13;
    let blue_total: i32 = 14;

    // at any point are there more reds than red_total?
    let handfuls: Vec<&str> = line.split(";").collect();
    for handful in handfuls.iter() {
        let (r, g, b) = parse_handful(handful);

        // println!("{} : {}", handful, b);
        if r > red_total { return false; }
        if g > green_total { return false; }
        if b > blue_total { return false; }
    }
    true
}

fn max_rgb(line: &str) -> (i32, i32, i32) {
    let mut red_msf: i32 = 0;
    let mut green_msf: i32 = 0;
    let mut blue_msf: i32 = 0;

    // at any point are there more reds than red_total?
    let handfuls: Vec<&str> = line.split(";").collect();
    for handful in handfuls.iter() {
        let (r, g, b) = parse_handful(handful);

        // println!("{} : {}", handful, b);
        if r > red_msf { red_msf = r; }
        if g > green_msf { green_msf = g; }
        if b > blue_msf { blue_msf = b; }
    }

    (red_msf, green_msf, blue_msf)
}

pub fn solution2(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut powers: Vec<i32> = Vec::new();

    for line in &lines {
        let game_handfuls: Vec<&str> = line.split(":").collect();
        let (r, g, b) = max_rgb(game_handfuls[1]);
        powers.push(r*g*b);
    }

    powers.iter().sum()
}

pub fn solution(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut possible_games: Vec<i32> = Vec::new();

    for line in &lines {
        let game_handfuls: Vec<&str> = line.split(":").collect();
        let game = Regex::new(r"(\d+)")
            .unwrap()
            .find(game_handfuls[0])
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        if is_game_possible(game_handfuls[1]) {
            possible_games.push(game);
        }
    }
    // sum them all together
    possible_games.iter().sum()
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
    fn test_is_game_possible2() {
        let stt = String::from("20 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(is_game_possible(&stt), false);
    }

    #[test]
    fn test_parse_handful(){
        assert_eq!(parse_handful("3 blue, 4 red"), (4, 0, 3))
    }

    // part2
    #[test]
    fn test_solution2() {
        assert_eq!(solution2("inputs/day2/test1.txt"), 2286);
    }

    #[test]
    fn test_max_rgb() {
        let stt = String::from("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(max_rgb(&stt), (4, 2, 6));
    }
    // #[test]
    // fn test_is_game_possible2() {
    //     let stt = String::from("20 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    //     assert_eq!(is_game_possible(&stt), false);
    // }
}