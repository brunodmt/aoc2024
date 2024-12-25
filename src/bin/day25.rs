extern crate utils;

use utils::Opt;

use core::str::Lines;
use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day25";
    return format!("input/{}/{}", day, file)
}

fn parse_key(lines: &mut Lines) -> [usize; 5] {
    let mut pins = [0_usize;5];
    while let Some(line) = lines.next() {
        println!("parsing line `{line}` in key loop");
        if line == "" { break; }
        for (i,c) in line.chars().enumerate() {
            match c {
                '#' => pins[i] += 1,
                '.' => {},
                _ => panic!("unexpected char {c}")
            }
        }
    }
    for i in 0..pins.len() {
        pins[i] -= 1;
    }
    return pins;
}

fn parse_lock(lines: &mut Lines) -> [usize;5] {
    let mut pins = [0_usize;5];
    while let Some(line) = lines.next() {
        println!("parsing line `{line}` in lock loop");
        if line == "" { break; }
        for (i,c) in line.chars().enumerate() {
            match c {
                '#' => pins[i] += 1,
                '.' => {},
                _ => panic!("unexpected char {c}")
            }
        }
    }
    return pins;
}

fn problem1(input: &str) -> u64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    let mut line_it = puzzle.lines();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    while let Some(line) = line_it.next() {
        println!("parsing line `{line}` in main loop");
        match line {
            "#####" => locks.push(parse_lock(&mut line_it)),
            "....." => keys.push(parse_key(&mut line_it)),
            "" => {},
            _ => panic!("Unexpected line `{line}`")
        }
    }

    let mut result = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut matches = 0;
            for i in 0..5 {
                if lock[i] + key[i] <= 5 {
                    matches += 1;
                } else {
                    break;
                }
            }
            if matches == 5 {
                result += 1;
            }
        }
    }

    return result;
}

fn problem2(input: &str) -> u64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    for _line in puzzle.lines() {

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
        assert_eq!(problem1("sample1"), 3);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 0);
    }
}
