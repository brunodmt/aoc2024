extern crate utils;

use regex::Regex;
use utils::Opt;

use std::{cmp::min, fs};
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day13";
    return format!("input/{}/{}", day, file)
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn problem(input: &str, p_correction: i64) -> i64 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    let puzzle: Vec<_> = re.captures_iter(&puzzle_str).map(|c| {
        Machine {
            a: (c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()),
            b: (c[3].parse::<i64>().unwrap(), c[4].parse::<i64>().unwrap()),
            prize: (c[5].parse::<i64>().unwrap() + p_correction, c[6].parse::<i64>().unwrap() + p_correction),
        }
    }).collect();

    let mut result = 0;
    for machine in puzzle {
        let Machine { a, b, prize } = machine;

        let b_max_x = prize.0 / b.0;
        let b_max_y = prize.1 / b.1;
        let b_max = min(b_max_x, b_max_y);

        let c = (prize.0 - b.0 * b_max, prize.1 - b.1 * b_max);
        let b_int =  (c.0 * a.1 - c.1 * a.0) / (b.1 * a.0 - b.0 * a.1);
        let a_int = (b_int * b.0 + c.0) / a.0;

        let a_n = a_int;
        let b_n = (prize.0 - a_n * a.0) / b.0;

        if a_n * a.0 + b_n * b.0 == prize.0 && a_n * a.1 + b_n * b.1 == prize.1 {
            result += a_n * 3 + b_n;
        }
    }
    return result;
}

fn main() {
    let opt = Opt::from_args();
    match opt.problem {
        1 => println!("Problem 1: {:?}", problem("input", 0)),
        2 => println!("Problem 2: {:?}", problem("input", 10000000000000)),
        _ => println!("No such problem"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem("sample1", 0), 480);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem("sample2", 10000000000000), 0);
    }
}
