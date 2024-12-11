extern crate utils;

use utils::Opt;

use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day9";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> usize {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let mut puzzle: Vec<u32> = puzzle_str.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut last = puzzle.len() - 1;
    let mut is_file = true;
    let mut checksum = 0;
    let mut i = 0;
    let mut j = 0;
    while i <= last {
        if is_file {
            let id = i / 2;
            for _ in 0..puzzle[i] {
                checksum += id * j;
                j += 1;
            }
        } else {
            let mut gap = puzzle[i];
            while gap > 0 {
                let candidate = last;
                let id = candidate / 2;
                while gap > 0 &&  puzzle[candidate] > 0 {
                    checksum += j * id;
                    puzzle[candidate] -= 1;
                    j += 1;
                    gap -= 1;
                }
                if puzzle[candidate] == 0 {
                    last -=2 ;
                }
            }
        }
        i += 1;
        is_file = !is_file;
    }

    return checksum;
}

#[derive(Debug)]
enum Block {
    File(u64, usize),
    Free(u64),
}

fn problem2(input: &str) -> u64 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let mut puzzle: Vec<Block> = puzzle_str.chars().enumerate().map(|(i, c)| {
        let d = c.to_digit(10).unwrap().into();
        if i % 2 == 0 {
            Block::File(d, i / 2)
        } else {
            Block::Free(d)
        }
    }).collect();

    for i in (0..puzzle.len()).rev() {
        if let Block::File(size, id) = puzzle[i] {
            for j in 0..i {
                if let Block::Free(available) = puzzle[j] {
                    if available >= size {
                        puzzle[j] = Block::Free(available - size);
                        puzzle[i] = Block::Free(size);
                        puzzle.insert(j, Block::File(size, id));
                        break;
                    }
                }
            }
        }
    }

    let mut position: u64 = 0;
    let mut checksum: u64 = 0;
    for block in puzzle.iter() {
        match block {
            Block::File(size, id) => {
                let id64 = u64::try_from(*id).unwrap();
                let pos_sum: u64 = (position..position+size).sum();
                checksum += id64 * pos_sum;
                position += size;
            },
            Block::Free(size) => {
                position += size;
            }
        }
    }

    return checksum;
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
        assert_eq!(problem1("sample1"), 1928);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 2858);
    }
}