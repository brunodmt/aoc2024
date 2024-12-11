extern crate utils;

use utils::Opt;

use std::{fs, str::FromStr, string::ParseError};
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day7";
    return format!("input/{}/{}", day, file)
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl FromStr for Equation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (string_val, string_nums) = s.split_once(':').expect("incorrect line fomrat found");
        let test_value = string_val.parse().expect("test value is not a valid");
        let numbers = string_nums.split_whitespace().map(|n| n.parse().expect("number is not valid")).collect();
        return Ok(Equation {
            test_value,
            numbers
        })
    }
}

impl Equation {
    fn try_resolve(&self, concat_enabled: bool) -> bool {
        return next_op(self.numbers[0], self.test_value, &self.numbers[1..], concat_enabled);
    }
}

fn concat_numbers(left: u64, right: u64) -> u64 {
    let rdigits = right.ilog10() + 1;
    return left * 10_u64.pow(rdigits) + right;
}

fn next_op(curr_val: u64, targ_val: u64, numbers: &[u64], concat_enabled: bool) -> bool {
    if numbers.len() == 0 {
        return curr_val == targ_val;
    } else if curr_val > targ_val {
        return false;
    } else {
        let option1 = next_op(curr_val + numbers[0], targ_val, &numbers[1..], concat_enabled);
        let option2 = next_op(curr_val * numbers[0], targ_val, &numbers[1..], concat_enabled);
        let option3 = concat_enabled && next_op(concat_numbers(curr_val, numbers[0]), targ_val, &numbers[1..], concat_enabled);
        return option1 || option2 || option3;
    }
}

fn resolve_puzzle(input: &str, concat_enabled: bool) -> u64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    let equations: Vec<Equation> = puzzle.lines().map(|l| l.parse::<Equation>().expect("invalid line found")).collect();
    let result = equations.iter().map(|eq| {
        if eq.try_resolve(concat_enabled) {
            eq.test_value
        } else {
            0
        }
    }).sum();
    return result;
}

fn problem1(input: &str) -> u64 {
    resolve_puzzle(input, false)
}

fn problem2(input: &str) -> u64 {
    resolve_puzzle(input, true)
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
        assert_eq!(problem1("sample1"), 3749);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 11387);
    }
}