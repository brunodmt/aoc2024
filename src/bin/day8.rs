extern crate utils;

use utils::Opt;

use std::{collections::{HashMap, HashSet}, fs};
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day8";
    return format!("input/{}/{}", day, file)
}

type Point = (usize, usize);

fn check_antinode(p: Point, idiff: isize, jdiff: isize, max: Point) -> Option<(usize, usize)> {
    let ai = p.0.checked_add_signed(idiff)?;
    let aj = p.1.checked_add_signed(jdiff)?;
    if ai < max.0 && aj < max.1 {
        return Some((ai, aj));
    } else {
        return None;
    }
}

fn problem1(input: &str) -> usize {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let puzzle: Vec<Vec<char>> = puzzle_str.lines().map(|l| l.chars().collect()).collect();
    let max = (puzzle.len(), puzzle[0].len());
    
    let mut antenna_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    puzzle.iter().enumerate().for_each(|(i, row)| row.iter().enumerate().for_each(|(j, c)| {
        if *c != '.' {
            if let Some(char_positions) = antenna_positions.get_mut(c) {
                char_positions.push((i, j));
            } else {
                antenna_positions.insert(*c, vec![(i, j)]);
            }
        }
    }));

    let mut antinodes_positions: HashSet<(usize, usize)> = HashSet::new();
    for char_antennas in antenna_positions.values() {
        for (n, &p0) in char_antennas.iter().enumerate() {
            for &p1 in &char_antennas[n+1..] {
                println!("check {:?} vs {:?}", p0, p1);
                let idiff = p1.0 as isize - p0.0 as isize;
                let jdiff = p1.1 as isize - p0.1 as isize;

                if let Some(an) = check_antinode(p0, -idiff, -jdiff, max) {
                    println!("antinode 1 in {:?}", an);
                    antinodes_positions.insert(an);
                }

                if let Some(an) = check_antinode(p1, idiff, jdiff, max) {
                    println!("antinode 2 in {:?}", an);
                    antinodes_positions.insert(an);
                }
            }
        }
    }
    println!("{:?}", antinodes_positions);
    return antinodes_positions.len();
}

fn problem2(input: &str) -> usize {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let puzzle: Vec<Vec<char>> = puzzle_str.lines().map(|l| l.chars().collect()).collect();
    let max = (puzzle.len(), puzzle[0].len());
    
    let mut antenna_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    puzzle.iter().enumerate().for_each(|(i, row)| row.iter().enumerate().for_each(|(j, c)| {
        if *c != '.' {
            if let Some(char_positions) = antenna_positions.get_mut(c) {
                char_positions.push((i, j));
            } else {
                antenna_positions.insert(*c, vec![(i, j)]);
            }
        }
    }));

    let mut antinodes_positions: HashSet<(usize, usize)> = HashSet::new();
    for char_antennas in antenna_positions.values() {
        for (n, &p0) in char_antennas.iter().enumerate() {
            for &p1 in &char_antennas[n+1..] {
                println!("check {:?} vs {:?}", p0, p1);
                let idiff = p1.0 as isize - p0.0 as isize;
                let jdiff = p1.1 as isize - p0.1 as isize;

                antinodes_positions.insert(p0);

                let mut current = p0;
                while let Some(an) = check_antinode(current, -idiff, -jdiff, max) {
                    println!("antinode in {:?}", an);
                    antinodes_positions.insert(an);
                    current = an;
                }

                current = p0;
                while let Some(an) = check_antinode(current, idiff, jdiff, max) {
                    println!("antinode in {:?}", an);
                    antinodes_positions.insert(an);
                    current = an;
                }
            }
        }
    }
    println!("{:?}", antinodes_positions);
    return antinodes_positions.len();
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
        assert_eq!(problem1("sample1"), 14);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 34);
    }
}