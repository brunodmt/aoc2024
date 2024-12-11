extern crate utils;

use utils::Opt;

use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "dayX";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> u64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    for line in puzzle.lines() {

    }
    return 0;
}

fn problem2(input: &str) -> u64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    for line in puzzle.lines() {

    }
    return 0;
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
        assert_eq!(problem1("sample1"), 0);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 0);
    }
}