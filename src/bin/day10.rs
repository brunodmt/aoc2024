extern crate utils;

use utils::Opt;

use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day10";
    return format!("input/{}/{}", day, file)
}

type Position = (usize, usize);

fn find_exits(next: u32, from: Position, puzzle: &Vec<Vec<u32>>) -> HashSet<Position> {
    let mut result = HashSet::new();
    if next == 10 {
        result.insert(from);
    } else {
        if from.0 > 0 {
            let (i,j) = (from.0 - 1, from.1);
            if puzzle[i][j] == next {
                result.extend(find_exits(next+1, (i,j), puzzle));
            }
        }
        if from.0 < puzzle.len() - 1 {
            let (i,j) = (from.0 + 1, from.1);
            if puzzle[i][j] == next {
                result.extend(find_exits(next+1, (i,j), puzzle));
            }
        }
        if from.1 > 0 {
            let (i,j) = (from.0, from.1 - 1);
            if puzzle[i][j] == next {
                result.extend(find_exits(next+1, (i,j), puzzle));
            }
        }
        if from.1 < puzzle[0].len() - 1 {
            let (i,j) = (from.0, from.1 + 1);
            if puzzle[i][j] == next {
                result.extend(find_exits(next+1, (i,j), puzzle));
            }
        }
    }
    result
}

fn problem1(input: &str) -> usize {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let puzzle: Vec<Vec<u32>> = puzzle_str.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let trailheads: Vec<Position> = puzzle.iter().enumerate().flat_map(|(i, l)| {
        l.iter().enumerate().filter(|&(_, &e)| e == 0).map(move |(j,_)| (i,j))
    }).collect();

    let mut result = 0;
    for th in trailheads {
        let exits = find_exits(1, th, &puzzle);
        println!("{:?} has ths {:?}", th, exits);
        result += exits.len();
    }

    return result;
}

fn count_exits(next: u32, from: Position, puzzle: &Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    if next == 10 {
        result = 1;
    } else {
        if from.0 > 0 {
            let (i,j) = (from.0 - 1, from.1);
            if puzzle[i][j] == next {
                result += count_exits(next+1, (i,j), puzzle);
            }
        }
        if from.0 < puzzle.len() - 1 {
            let (i,j) = (from.0 + 1, from.1);
            if puzzle[i][j] == next {
                result += count_exits(next+1, (i,j), puzzle);
            }
        }
        if from.1 > 0 {
            let (i,j) = (from.0, from.1 - 1);
            if puzzle[i][j] == next {
                result += count_exits(next+1, (i,j), puzzle);
            }
        }
        if from.1 < puzzle[0].len() - 1 {
            let (i,j) = (from.0, from.1 + 1);
            if puzzle[i][j] == next {
                result += count_exits(next+1, (i,j), puzzle);
            }
        }
    }
    result
}

fn problem2(input: &str) -> u32 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let puzzle: Vec<Vec<u32>> = puzzle_str.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let trailheads: Vec<Position> = puzzle.iter().enumerate().flat_map(|(i, l)| {
        l.iter().enumerate().filter(|&(_, &e)| e == 0).map(move |(j,_)| (i,j))
    }).collect();

    let mut result = 0;
    for th in trailheads {
        let exits = count_exits(1, th, &puzzle);
        println!("{:?} has ths {:?}", th, exits);
        result += exits;
    }

    return result;
}

fn main() {
    let opt = Opt::from_args();
    match opt.problem {
        1 => println!("Problem 1: {:?}", problem1("input")),
        2 => println!("Problem 2: {:?}", problem2("input")),
        _ => println!("No such problem"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1("sample1"), 36);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 81);
    }
}