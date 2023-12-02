use regex::Regex;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn english_to_int(en: &str) -> i64 {
    match en {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ =>  en.parse::<i64>().unwrap(),
    }
}

fn find_digits_in_line_pt2(line: &String) -> i64 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let er = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();
    let reversed_line = line.chars().rev().collect::<String>();

    let first = re.find(line).unwrap().as_str();
    let first_d = english_to_int(first);

    let last = er.find(&reversed_line).unwrap().as_str();
    let last_d = english_to_int(last);
    
    format!("{}{}", first_d, last_d).parse::<i64>().unwrap()
}

pub fn day1_pt2(filename: &str) -> i64 {
    let mut digits = Vec::new();
    let lines = read_lines(filename);
    // loop through each line and find the first and last digit.
    for line in &lines {
        digits.push(find_digits_in_line_pt2(line));
    }

    // sum them all together
    digits.iter().sum()
}

fn find_digits_in_line(line: &String) -> i64 {
    // "1abc2"
    let re = Regex::new(r"\d").unwrap(); // match any digit
    
    let matches: Vec<_> = re.find_iter(line).collect();
    
    format!("{}{}", matches.first().unwrap().as_str(), matches.last().unwrap().as_str()).parse::<i64>().unwrap()
}

pub fn day1(filename: &str) -> i64 {
    let mut digits = Vec::new();
    let lines = read_lines(filename);
    // loop through each line and find the first and last digit.
    for line in &lines {
        digits.push(find_digits_in_line(line));
    }
    // sum them all together
    digits.iter().sum()
}

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;
    
//     // part1
//     #[test]
//     fn test_day1_test() {
//         assert_eq!(day1("inputs/day1/testinput.txt"), 142);
//     }
    
//     #[test]
//     fn test_find_digits_in_line() {
//         let line: String = String::from("1abc2");
//         assert_eq!(find_digits_in_line(&line), 12);
//     }

//     // part 2
//     #[test]
//     fn test_day1_pt2_test() {
//         assert_eq!(day1_pt2("inputs/day1/testpt2.txt"), 281);
//     }
    
//     #[test]
//     fn test_find_digits_in_line_pt2() {
//         let line: String = String::from("zoneight234");
//         assert_eq!(find_digits_in_line_pt2(&line), 14);
//     }

//     #[test]
//     fn test_find_digits_in_line_pt2_a() {
//         let line: String = String::from("fxfrqrxbrkdfivepstvtqhlrnhvqbtzvfnineh6");
//         assert_eq!(find_digits_in_line_pt2(&line), 56);
//     }

//     #[test]
//     fn test_find_digits_in_line_pt2_b() {
//         let line: String = String::from("fqckxpqrjk8eighteighttwo6fivejps4");
//         assert_eq!(find_digits_in_line_pt2(&line), 84);
//         let line: String = String::from("4");
//         assert_eq!(find_digits_in_line_pt2(&line), 44);
//         let line: String = String::from("13");
//         assert_eq!(find_digits_in_line_pt2(&line), 13);
//         let line: String = String::from("four");
//         assert_eq!(find_digits_in_line_pt2(&line), 44);
//         let line: String = String::from("four5");
//         assert_eq!(find_digits_in_line_pt2(&line), 45);
//         let line: String = String::from("4five");
//         assert_eq!(find_digits_in_line_pt2(&line), 45);
//         let line: String = String::from("a4fiveb");
//         assert_eq!(find_digits_in_line_pt2(&line), 45);
//         let line: String = String::from("8k");
//         assert_eq!(find_digits_in_line_pt2(&line), 88);
//         let line: String = String::from("oneight");
//         assert_eq!(find_digits_in_line_pt2(&line), 18);
//     }

//     #[test]
//     fn test_read_lines() {
//         let expected: Vec<String> = vec![String::from("1abc2"), String::from("pqr3stu8vwx"), String::from("a1b2c3d4e5f"), String::from("treb7uchet")];
//         assert_eq!(read_lines("inputs/day1/testinput.txt"), expected);
//     }
// }