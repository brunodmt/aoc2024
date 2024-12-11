extern crate utils;

use utils::Opt;

use std::{borrow::BorrowMut, collections::HashSet, fs};
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day6";
    return format!("input/{}/{}", day, file)
}

fn rotate(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (-1,  0) => ( 0,  1),
        ( 0,  1) => ( 1,  0),
        ( 1,  0) => ( 0, -1),
        ( 0, -1) => (-1,  0),
        _ => panic!("unexpected direction")
    }
}

fn problem1(input: &str) -> u64 {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let mut puzzle = puzzle_str.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let max_x = puzzle.len();
    let max_y = puzzle[0].len();
    let mut x = puzzle.iter().position(|x| x.iter().any(|&y| y == '^')).unwrap();
    let mut y = puzzle[x].iter().position(|&y| y == '^').unwrap();
    let mut visited = 1;
    let mut direction = (-1, 0);
    println!("start is {},{}", x, y);
    loop {
        let next_x = x.checked_add_signed(direction.0);
        let next_y = y.checked_add_signed(direction.1);

        if next_x.is_none() || next_y.is_none() {
            break;
        }

        let next_x = next_x.unwrap();
        let next_y = next_y.unwrap();

        if next_x >= max_x || next_y >= max_y {
            break;
        }

        let next_char = puzzle[next_x][next_y];
        match next_char {
            '#' => {
                direction = rotate(direction);
                println!("rotating at {},{}", x, y);
            },
            '.' => {
                visited += 1;
                x = next_x;
                y = next_y;
                puzzle[x][y] = 'X';
                println!("visiting {},{} (new)", x, y);
            },      
            'X' | '^' => {
                x = next_x;
                y = next_y;
                println!("visiting {},{} (prev)", x, y);
            },
            _ => panic!("unrecognized char '{}' found at {} {}", next_char, next_x, next_y)
        }
    }
    return visited;
}

#[derive(Copy, Clone)]
struct Place {
    obstructed: bool,
    visited_up: bool,
    visited_left: bool,
    visited_down: bool,
    visited_right: bool,
}

impl Place {
    fn new(initial: char) -> Place {
        Place {
            obstructed: initial.eq(&'#'),
            visited_up: initial.eq(&'^'),
            visited_left: false,
            visited_down: false,
            visited_right: false,
        }
    }

    fn is_visited(&self) -> bool {
        self.visited_up || self.visited_left || self.visited_down || self.visited_right
    }

    fn is_visited_dir(&self, dir: (isize, isize)) -> bool {
        match dir {
            (-1,  0) => self.visited_up,
            ( 0,  1) => self.visited_right,
            ( 1,  0) => self.visited_down,
            ( 0, -1) => self.visited_left,
            _ => panic!("unexpected direction")
        }    
    }

    fn visit(&mut self, dir: (isize, isize)) {
        match dir {
            (-1,  0) => self.visited_up = true,
            ( 0,  1) => self.visited_right = true,
            ( 1,  0) => self.visited_down = true,
            ( 0, -1) => self.visited_left = true,
            _ => panic!("unexpected direction")
        }
    }
}

fn has_loop(mut x: usize, mut y: usize, mut direction: (isize, isize), mut puzzle: Vec<Vec<Place>>) -> bool {
    let max_x = puzzle.len();
    let max_y = puzzle[0].len();
    loop {
        let next_x = x.checked_add_signed(direction.0);
        let next_y = y.checked_add_signed(direction.1);

        if next_x.is_none() || next_y.is_none() {
            return false;
        }

        let next_x = next_x.unwrap();
        let next_y = next_y.unwrap();

        if next_x >= max_x || next_y >= max_y {
            return false;
        }

        let next_char = puzzle[next_x][next_y].borrow_mut();
        if next_char.obstructed {
            direction = rotate(direction);
            //println!("try> rotating at {},{}", x, y);
        } else if next_char.is_visited_dir(direction) {
            return true;
        } else {
            next_char.visit(direction);
            x = next_x;
            y = next_y;
            //println!("try> visiting {},{}", x, y);
        }
    }
}

fn problem2(input: &str) -> usize {
    let puzzle_str: String = fs::read_to_string(input_path(input)).unwrap();
    let orig_puzzle = puzzle_str.lines().map(|line| line.chars().map(|char| Place::new(char)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let max_x = orig_puzzle.len();
    let max_y = orig_puzzle[0].len();
    let start_x = orig_puzzle.iter().position(|x| x.iter().any(|y| y.visited_up)).unwrap();
    let start_y = orig_puzzle[start_x].iter().position(|y| y.visited_up).unwrap();
    let mut puzzle = orig_puzzle.clone();
    let mut x = start_x;
    let mut y = start_y;
    let mut possibles = HashSet::new();
    let mut solutions = 0;
    let mut direction = (-1, 0);
    //println!("start is {},{}", x, y);
    loop {
        let next_x = x.checked_add_signed(direction.0);
        let next_y = y.checked_add_signed(direction.1);

        if next_x.is_none() || next_y.is_none() {
            break;
        }

        let next_x = next_x.unwrap();
        let next_y = next_y.unwrap();

        if next_x >= max_x || next_y >= max_y {
            break;
        }

        let next_char = puzzle[next_x][next_y].borrow_mut();
        if next_char.obstructed {
            direction = rotate(direction);
            //println!("rotating at {},{}", x, y);
        } else {
            next_char.visit(direction);
            possibles.insert((next_x, next_y));
            x = next_x;
            y = next_y;
            //println!("visiting {},{}", x, y);
        }
    }
    possibles.remove(&(start_x, start_y));
    for p in possibles.iter() {
        let mut sim_puzzle = orig_puzzle.clone();
        sim_puzzle[p.0][p.1].obstructed = true;
        //println!("try obstruct at {},{}", p.0, p.1);
        if has_loop(start_x, start_y, (-1, 0), sim_puzzle) {
            //println!("can obstruct at {},{}", p.0, p.1);
            solutions += 1;
        }
    }
    return solutions;
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
        assert_eq!(problem1("sample1"), 41);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 6);
    }
}