extern crate utils;

use utils::Opt;

use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day12";
    return format!("input/{}/{}", day, file)
}

type Position = (usize, usize);
type Perimeter = i64;
type Area = i64;
type Solution = (Perimeter, Area);

#[derive(Debug)]
struct Plot {
    plant: char,
    researched: bool,
    fences: Vec<(isize, isize)>
}

impl Plot {
    fn new(plant: char) -> Plot {
        Plot {
            plant,
            researched: false,
            fences: vec![],
        }
    }
}

fn research(puzzle: &mut Vec<Vec<Plot>>, from: Position, bulk_buy: bool) -> Solution {
    let plant = puzzle[from.0][from.1].plant;
    let mut perimeter = 0;
    let mut area = 1;
   
    let directions = [(-1,0),(0,1),(1,0),(0,-1)];
    let sides: [Option<(usize, usize)>; 4] = directions.map(|(di, dj)| {
        let i = from.0.checked_add_signed(di)?;
        let j = from.1.checked_add_signed(dj)?;
        if i < puzzle.len() && j < puzzle.len() && puzzle[i][j].plant == plant {
            Some((i, j))    
        } else {
            None
        }
    });

    let plots = sides.iter().filter_map(|x| *x);
    let fences = directions.iter().enumerate().filter(|&(x,_)| {
        sides[x].is_none()
    }).map(|(_,d)| d);

    puzzle[from.0][from.1].researched = true;
    puzzle[from.0][from.1].fences.extend(fences.clone());

    if bulk_buy {
        for fence in fences {
            let pcount = plots.clone().filter(|&(i, j)| puzzle[i][j].fences.contains(fence)).count();
            match pcount {
                0 => perimeter += 1,
                2 => perimeter -= 1,
                _ => {}
            }
        }
    } else {
        perimeter += fences.count() as i64;
    }

    for (i, j) in plots {
        if !puzzle[i][j].researched {
            let (p, a) = research(puzzle, (i, j), bulk_buy);
            perimeter += p;
            area += a;
        }
    }

    (perimeter, area)
}

fn resolve(input: &str, bulk_buy: bool) -> i64 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let mut puzzle: Vec<Vec<Plot>> = puzzle_str.lines().map(|l| l.chars().map(|c| Plot::new(c)).collect()).collect();

    let mut result = 0;
    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            if !puzzle[i][j].researched {
                let (p,a) = research(&mut puzzle, (i, j), bulk_buy);
                result += p * a;
            }
        }
    }
    result
}

fn problem1(input: &str) -> i64 {
    resolve(input, false)
}


fn problem2(input: &str) -> i64 {
    resolve(input, true)
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
        assert_eq!(problem1("sample11"), 140);
        assert_eq!(problem1("sample12"), 772);
        assert_eq!(problem1("sample13"), 1930);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample11"), 80);
        assert_eq!(problem2("sample12"), 436);
        assert_eq!(problem2("sample21"), 236);
        assert_eq!(problem2("sample22"), 368);
        assert_eq!(problem2("sample13"), 1206);
    }
}