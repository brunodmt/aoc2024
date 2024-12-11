extern crate utils;

use utils::Opt;

use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day4";
    return format!("input/{}/{}", day, file)
}

fn search_from_position(puzzle: &Vec<Vec<char>>, i: usize, j: usize, i_dir: isize, j_dir: isize) -> u64 {
    let chars = ['M', 'A', 'S'];
    let rem_chars = 3;
    let mut matches = true;
    let mut pos = 0;
    let mut x = j;
    let mut y = i;
    while pos < rem_chars && matches {
        y = y.checked_add_signed(i_dir).unwrap();
        x = x.checked_add_signed(j_dir).unwrap();
        if puzzle[y][x] != chars[pos] {
            matches = false;
            break;
        }
        pos += 1;
    }
    
    return if matches { 1 } else { 0 };
}

fn search_everywhere_from_position(puzzle: &Vec<Vec<char>>, i: usize, j: usize, max_i: usize, max_j: usize) -> u64 {
    let rem_chars = 3;
    let mut result = 0;
    if i >= rem_chars {
        // search up
        result += search_from_position(puzzle, i, j,-1, 0);
    }
    if (i >= rem_chars) && (j + rem_chars < max_j) {
        // search up right
        result += search_from_position(puzzle, i, j,-1, 1);
    }
    if j + rem_chars < max_j {
        // search right
        result += search_from_position(puzzle, i, j,0, 1);
    }
    if (i + rem_chars < max_i) && (j + rem_chars < max_j) {
        // search down right
        result += search_from_position(puzzle, i, j, 1, 1);
    }
    if i + rem_chars < max_i {
        // search down
        result += search_from_position(puzzle, i, j, 1, 0);
    }
    if (i + rem_chars < max_i) && (j >= rem_chars) {
        // search down left
        result += search_from_position(puzzle, i, j, 1, -1);
    }
    if j >= rem_chars {
        // search left
        result += search_from_position(puzzle, i, j,0, -1);
    }
    if (i >= rem_chars) && (j >= rem_chars) {
        // search up left
        result += search_from_position(puzzle, i, j,-1, -1);
    }
    return result;
}

fn problem1(input: &str) -> u64 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let puzzle = puzzle_str.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let max_i = puzzle.len();
    let max_j = puzzle[0].len();
    let mut result = 0;
    for (i, line) in puzzle.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'X' {
                result += search_everywhere_from_position(&puzzle, i, j, max_i, max_j, );
            }
        }
    }
    return result;
}

fn search_x(puzzle: &Vec<Vec<char>>, i: usize, j: usize, i_dir: isize, j_dir: isize) -> bool {
    let first_i = i.checked_add_signed(i_dir).unwrap();
    let first_j = j.checked_add_signed(j_dir).unwrap();
    if puzzle[first_i][first_j] != 'M' {
        return false;
    }
    
    let second_i = i.checked_add_signed(-i_dir).unwrap();
    let second_j = j.checked_add_signed(-j_dir).unwrap();

    if puzzle[second_i][second_j] != 'S' {
        return false;
    }

    return true;
}

fn search_x_from_position(puzzle: &Vec<Vec<char>>, i: usize, j: usize, max_i: usize, max_j: usize) -> bool {
    if !(1..(max_i-1)).contains(&i) {
        return false;
    }
    
    if !(1..(max_j-1)).contains(&j) {
        return false;
    }

    let downwards = search_x(puzzle, i, j, -1, 1) || search_x(puzzle, i, j, 1, -1);
    let upwards = search_x(puzzle, i, j, 1, 1) || search_x(puzzle, i, j, -1, -1);
    return downwards && upwards;
}

fn problem2(input: &str) -> u32 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let puzzle = puzzle_str.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let max_i = puzzle.len();
    let max_j = puzzle[0].len();
    let mut result = 0;
    for (i, line) in puzzle.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'A' {
                if search_x_from_position(&puzzle, i, j, max_i, max_j, ) {
                    result += 1;
                }
            }
        }
    }
    return result;
}

fn main() {
    let opt = Opt::from_args();
    match opt.problem {
        1 => println!("Problem 1: {:?}", problem1("problem1")),
        2 => println!("Problem 2: {:?}", problem2("problem2")),
        _ => println!("No such problem"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1("sample1"), 18);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 9);
    }
}