extern crate utils;

use utils::Opt;

use regex::Regex;
use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day3";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> u64 {
    let program: String = fs::read_to_string(input_path(input)).unwrap();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let caps = re.captures_iter(&program);
    let pairs = caps.map(|cap| (cap[1].parse::<u64>().unwrap(),
                                                                            cap[2].parse::<u64>().unwrap()));
    let results = pairs.map(|(x,y)| x * y);
    let total: u64 = results.sum();                                                                        
    return total;
}

fn problem2(input: &str) -> u64 {
    let program: String = fs::read_to_string(input_path(input)).unwrap();
    let re = Regex::new(r"don't\(\)|do\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let caps = re.captures_iter(&program);
    let mut total = 0;
    let mut enabled = true;
    for c in caps{
        match &c[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let x = c[1].parse::<u64>().unwrap();
                    let y = c[2].parse::<u64>().unwrap();
                    total = total + (x*y);
                }
            }
        }
    }
    return total;
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
        assert_eq!(problem1("sample1"), 161);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 48);
    }
}