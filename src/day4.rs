use std::fs::read_to_string;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
    .unwrap()  // panic on possible file-reading errors
    .lines()  // split the string into an iterator of string slices
    .map(String::from)  // make each slice into a string
    .collect()  // gather them together into a vector
}

fn parse_line(line: &String) -> (HashSet<i32>, HashSet<i32>) {
    let card: Vec<&str> = line.split(":").collect();
    let numbers: Vec<&str> = card[1].split("|").collect();

    let winning: Vec<i32> = numbers[0].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let has: Vec<i32> = numbers[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    (HashSet::from_iter(winning), HashSet::from_iter(has))
}

pub fn solution(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut parsed: Vec<(HashSet<i32>, HashSet<i32>)> = Vec::new();

    // loop through each line and find the first and last digit.
    for line in &lines {
        parsed.push(parse_line(line));
    }

    let mut total_so_far: i32 = 0;

    for card in &parsed {
        let mut card_total: i32 = 0;
        let (win, has) = card;
        let diff: HashSet<_> = (*has).difference(win).collect::<HashSet<_>>();
        let matching = has.len() - diff.len();
        
        // For every matching card, you get 1 point for first one, and then after that it doubles. 
        for _i in 0..matching {
            if card_total == 0 { card_total = 1; }
            else { card_total = card_total * 2; }
        }

        total_so_far += card_total;
    }

    total_so_far
}

pub fn solution2(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut parsed: Vec<(HashSet<i32>, HashSet<i32>)> = Vec::new();
    
    // loop through each line and find the first and last digit.
    for line in &lines {
        parsed.push(parse_line(line));
    }

    // Set the 0 index to 0 and then index 1 will contain the count of card 1.
    let mut num_of_each_card = vec![1; parsed.len()+1];
    num_of_each_card[0] = 0;

    let max_card = parsed.len();


    for i in 0..max_card {
        let card_num = i + 1;

        let (win, has) = &parsed[i];
        
        let diff: HashSet<_> = has.difference(&win).collect::<HashSet<_>>();
        let matching = has.len() - diff.len();
        
        // Process each card n times.
        for repeats in 0..num_of_each_card[card_num] {

            // Winning cards are (current_card+1 - current_card+num_matching)
            // So of card 1 has 4 matching you get 2, 3, 4, 5.
            for win_more_of in card_num+1..card_num+matching+1 {
                if win_more_of <= max_card {
                    num_of_each_card[win_more_of] += 1;
                }
            }
        }
    }

    num_of_each_card.iter().sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    // part1
    #[test]
    fn test_solution() {
        assert_eq!(solution("inputs/day4/test1.txt"), 13);
    }

    #[test]
    fn test_parse_line(){
        let wining: HashSet<i32> = HashSet::from([41, 48, 83, 86, 17]);
        let has: HashSet<i32> = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(parse_line(&String::from("Card1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")), (wining, has));
    }

    // part2
    #[test]
    fn test_solution2() {
        assert_eq!(solution2("inputs/day4/test1.txt"), 30);
    }
}