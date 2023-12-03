use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
    .unwrap()  // panic on possible file-reading errors
    .lines()  // split the string into an iterator of string slices
    .map(String::from)  // make each slice into a string
    .collect()  // gather them together into a vector
}


fn build_matrix(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut input: Vec<Vec<char>>  = Vec::new();
    for line in &lines {
        input.push(line.chars().collect());
    }
    input
}

fn is_symbol(cchar: char) -> bool {
    !cchar.is_numeric() && cchar != '.'
}
    // 0  1  2
// [., ., .]0
// [., X, .]1
// [., ., .]2
// any number adjacent to a symbol, even diagonally, is a 'part number' and should be included in your sum.
fn near_symbol(col: usize, row: usize, matrix: &Vec<Vec<char>>) -> bool {
    // println!("{}", matrix[row][col]);
    // Check all adjacent boxes, for a symbol
    is_symbol(matrix[row-1][col-1]) ||
    is_symbol(matrix[row-1][col]) ||
    is_symbol(matrix[row-1][col+1]) ||
    is_symbol(matrix[row][col-1]) ||
    is_symbol(matrix[row][col+1]) ||
    is_symbol(matrix[row+1][col-1]) ||
    is_symbol(matrix[row+1][col]) ||
    is_symbol(matrix[row+1][col+1])
}

fn parse_row(matrix: &Vec<Vec<char>>, row_index: usize) -> Vec<i32> {
    let mut part_number_so_far: String = String::from("");
    let mut current_part_near_symbol = false;

    let mut part_numbers: Vec<i32> = Vec::new();
    
    // I modified my input to surround it with plain dots AND added an extra dot at the end of each line (so 2 dots extra at the end.)
    for i in 1..matrix[row_index].len()-1 {
        let cchar = matrix[row_index][i];
    
        // Process digits
        if cchar.is_numeric() {
            // store in part_number_so_far
            part_number_so_far = format!("{}{}", part_number_so_far, cchar);
        
            // Part isn't currently near a symbol but this new position is, update that.
            if !current_part_near_symbol && near_symbol(i, row_index, matrix) {
                current_part_near_symbol = true;
            }
        }
        // Process non digits
        // if cchar is not a digit and part_number_so_far is not an empty string
        if !cchar.is_numeric(){

            // If we have a part near a symbol
            if part_number_so_far.len() > 0  && current_part_near_symbol {
                // add part_number_so_far to part_numbers
                part_numbers.push(part_number_so_far.parse().unwrap());
            }

            // Make sure things are cleared out for next part.
            part_number_so_far = String::from("");
            current_part_near_symbol = false;
        }
    }
    part_numbers
}

pub fn solution(filename: &str) -> i32 {

    let matrix = build_matrix(read_lines(filename));

    let mut part_number_sum: i32 = 0;

    for i in 1..matrix.len()-1 {
        let part_sum: i32 = parse_row(&matrix, i).iter().sum();
        part_number_sum = part_number_sum + part_sum;
    }

    part_number_sum
}
    // 0  1  2
// [., ., .]0
// [., X, .]1
// [., ., .]2
// returns the index of any adjacent starts
fn near_star_idx(col: usize, row: usize, matrix: &Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let mut adjacent_starts: Vec<usize = Vec::new();
    if matrix[row-1][col-1] == '*' {
        adjacent_starts.push((row-1, col-1));
    }
    if matrix[row-1][col] == '*' {
        adjacent_starts.push((row-1, col));
    }
    if matrix[row-1][col+1] == '*' {
        adjacent_starts.push((row-1, col+1));
    }
    if matrix[row][col-1] == '*' {
        adjacent_starts.push((row, col-1));
    }
    if matrix[row][col+1] == '*' {
        adjacent_starts.push((row, col+1));
    }
    if matrix[row+1][col-1] == '*' {
        adjacent_starts.push((row+1, col-1));
    }
    if matrix[row+1][col] == '*' {
        adjacent_starts.push((row+1, col));
    }
    if matrix[row+1][col+1] == '*' {
        adjacent_starts.push((row+1, col+1));
    }

    adjacent_starts
}

// A gear is any * symbol that is adjacent to exactly two part numbers.
// Its gear ratio is the result of multiplying those two numbers together.
pub fn solution2(filename: &str) -> i32 {
    let lines = read_lines(filename);

    // let mut powers: Vec<i32> = Vec::new();

    // // for line in &lines {
    // //     powers.push();
    // // }

    // powers.iter().sum()
    5
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

    // 467..114.. [4, 6, 7, ., ., 1, 1, 4, ., .]
    // ...*...... [., ., ., *, ., ., ., ., ., .]
    // ..35..633. [., ., 3, 5, ., ., 6, 3, 3, .]
    #[test]
    fn test_build_matrix() {
        let expected = vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
        ];
        assert_eq!(build_matrix(read_lines("inputs/day3/test1small.txt")), expected);
    }

    #[test]
    fn test_parse_row() {
        let input = vec![
            vec!['.','.', '.', '.', '.', '.', '.', '.', '.', '.', '.','.','.',],
            vec!['.','4', '6', '7', '.', '.', '1', '1', '4', '.', '.','.','.',],
            vec!['.','.', '.', '.', '*', '.', '.', '.', '.', '.', '.','.','.',],
            vec!['.','.', '.', '3', '5', '.', '.', '6', '3', '3', '.','.','.',],
            vec!['.','.', '.', '.', '.', '.', '.', '.', '.', '.', '.','.','.',],
        ];
        assert_eq!(parse_row(&input, 1), vec![467]);
        assert_eq!(parse_row(&input, 2), vec![]);
        assert_eq!(parse_row(&input, 3), vec![35]);
        assert_eq!(parse_row(&input, 4), vec![]);
    }

    #[test]
    fn test_near_symbol() {
        let input = vec![
            vec!['.','.', '.', '.', '.', '.', '.', '.', '.', '.', '.','.','.',],
            vec!['.','4', '6', '7', '.', '.', '1', '1', '4', '.', '.','.','.',],
            vec!['.','.', '.', '.', '*', '.', '.', '.', '.', '.', '.','.','.',],
            vec!['.','.', '.', '3', '5', '.', '.', '6', '3', '3', '.','.','.',],
            vec!['.','.', '.', '.', '.', '.', '.', '.', '.', '.', '.','.','.',],
        ];
        assert_eq!(near_symbol(1, 1, &input), false);
        assert_eq!(near_symbol(3, 1, &input), true);
    }

    // part2
    // #[test]
    // fn test_solution2() {
    //     assert_eq!(solution2("inputs/day3/test1.txt"), 2286);
    // }

}