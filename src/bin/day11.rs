extern crate utils;

use utils::Opt;

use std::{collections::HashMap, fs};
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day11";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> usize {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let mut stones: Vec<u64> = puzzle_str.split_whitespace().map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
    for _ in 0..25 {
        let mut j = 0;
        while j < stones.len() {
            if stones[j] == 0 {
                stones[j] = 1;
            } else {
                let digits = stones[j].ilog10() + 1;
                if digits % 2 == 0 {
                    let digits = stones[j].ilog10() + 1;
                    let left = stones[j] / 10_u64.pow(digits / 2);
                    let right = stones[j] % 10_u64.pow(digits / 2);
                    stones[j] = right;
                    stones.insert(j, left);
                    j += 1;
                } else {
                    stones[j] = stones[j] * 2024;
                }
            }
            j += 1;
        }
    }
    return stones.len();
}

struct Cache {
    nodes: HashMap<u64, Vec<u64>>,
    results: HashMap<(u64, u64), u64>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            nodes: HashMap::new(),
            results: HashMap::new(),
        }
    }

    fn get_nexts(&mut self, value: u64) -> Vec<u64> {
        self.nodes.entry(value).or_insert_with(|| {
            let mut nexts = Vec::new();
            if value == 0 {
                nexts.push(1);
            } else {
                let digits = value.ilog10() + 1;
                if digits % 2 == 0 {
                    let left = value / 10_u64.pow(digits / 2);
                    let right = value % 10_u64.pow(digits / 2);
                    nexts.push(left);
                    nexts.push(right);
                } else {
                    nexts.push(value * 2024);
                }
            }
            nexts
        }).to_vec()
    }
}

fn resolve(cache: &mut Cache, nodes: Vec<u64>, blinks: u64) -> u64 {
    if blinks == 0 {
        nodes.len() as u64
    } else {
        let mut result = 0;
        for node in nodes {
            result += if let Some(cached) = cache.results.get(&(node, blinks)) {
                *cached
            } else {
                let nexts = cache.get_nexts(node);
                let new_stones = resolve(cache, nexts, blinks - 1);
                cache.results.insert((node, blinks), new_stones);
                new_stones
            }
        }
        result
    }
}

fn problem2(input: &str) -> u64 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let stones: Vec<u64> = puzzle_str.split_whitespace().map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
    let mut cache = Cache::new();
    resolve(&mut cache, stones, 75)
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
        assert_eq!(problem2("sample1"), 55312);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 0);
    }
}