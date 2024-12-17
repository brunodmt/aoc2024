extern crate utils;

use utils::Opt;
use utils::read_lines;

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day16";
    return format!("input/{}/{}", day, file)
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    End,
    Start,    
    Wall,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            'E' => Tile::End,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            _ => panic!("Tile not recognized"),
        }   
    }
}

#[derive(Debug,Clone,Copy,Eq,Hash,PartialEq)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    fn get_fw_move(&self) -> (isize, isize) {
        match self {
            Direction::East => (0,1),
            Direction::North => (-1,0),
            Direction::South => (1,0),
            Direction::West => (0,-1),
        }
    }

    fn get_left_direction(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }     
    }

    fn get_right_direction(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }     
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Hash,Debug)]
struct Node {
    i: usize,
    j: usize,
    dir: Direction,
}

impl Node {
    fn move_forward(&self) -> Node {
        let step = self.dir.get_fw_move();
        let i = self.i.checked_add_signed(step.0).unwrap();
        let j = self.j.checked_add_signed(step.1).unwrap();
        Node { i, j, dir: self.dir }
    }

    fn rotate_left(&self) -> Node {
        Node { i: self.i, j: self.j, dir: self.dir.get_left_direction() }
    }

    fn rotate_right(&self) -> Node {
        Node { i: self.i, j: self.j, dir: self.dir.get_right_direction() }
    }
}

enum Solution {
    Cost,
    Path,
}

fn print_maze(maze: &Vec<Vec<Tile>>) {
    for row in maze.iter() {
        for col in row.iter() {
            match col {
                Tile::Empty => print!("."),
                Tile::End => print!("E"),
                Tile::Wall => print!("#"),
                Tile::Start => print!("S"),
            }
        }
        println!();
    }
}

fn print_maze_with_seats(maze: &Vec<Vec<Tile>>, seats: &HashSet<(usize, usize)>) {
    for (i, row) in maze.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if seats.contains(&(i,j)) {
                print!("O");
            } else {
                match col {
                    Tile::Empty => print!("."),
                    Tile::End => print!("E"),
                    Tile::Wall => print!("#"),
                    Tile::Start => print!("S"),
                }
            }
        }
        println!();
    }
}

fn resolve(input: &str, solution: Solution) -> usize {
    let mut maze: Vec<Vec<Tile>> = Vec::new();
    let mut start = (0,0);

    if let Ok(lines) = read_lines(input_path(input)) {
        for (i, line) in lines.flatten().enumerate() {
            let row = line.chars().enumerate().map(|(j, c)| {
                if c == 'S' { start = (i, j); }
                Tile::from_char(c)
            }).collect();
            maze.push(row);
        }
    }

    let ops: [(fn(&Node) -> Node, usize); 3] = [
        (|node| { Node::move_forward(&node) }, 1),
        (|node| { Node::rotate_left(&node) }, 1000),
        (|node| { Node::rotate_right(&node) }, 1000),
    ];

    let mut costs: HashMap<Node, usize> = HashMap::new();
    let mut paths: HashMap<Node, HashSet<(usize, usize)>> = HashMap::new();
    let mut pending: Vec<Node> = Vec::new();

    pending.push(Node { i: start.0, j: start.1, dir: Direction::East });
    costs.insert(pending[0], 0);
    paths.insert(pending[0], HashSet::new());
    
    while !pending.is_empty() {
        pending.sort_by(|a, b| {
            let cost_a = costs.get(a).unwrap_or(&usize::MAX);
            let cost_b = costs.get(b).unwrap_or(&usize::MAX);
            cost_a.cmp(cost_b)
        });

        let node = pending.remove(0);

        if maze[node.i][node.j] == Tile::End {
            match solution {
                Solution::Cost => {
                    print_maze(&maze);
                    return *costs.get(&node).unwrap();
                },
                Solution::Path => {
                    print_maze_with_seats(&maze, paths.get(&node).unwrap());
                    return paths.get(&node).unwrap().len() + 1;
                },
            }
        }

        if !costs.contains_key(&node) {
            panic!("Node with no cost!");
        }
        
        for (op, cost) in ops.iter() {
            let next_node = op(&node);
            if maze[next_node.i][next_node.j] != Tile::Wall {
                let new_cost = costs.get(&node).unwrap() + cost;
                if let Some(&prev_cost) = costs.get(&next_node) {
                    if new_cost > prev_cost { continue; }
                    let path = paths.get(&node).unwrap().clone();
                    let next_path = paths.get_mut(&next_node).unwrap();
                    if new_cost < prev_cost {
                        costs.insert(next_node, new_cost);
                        next_path.clear();
                    }
                    next_path.extend(path);
                    next_path.insert((node.i, node.j));
                } else {
                    pending.push(next_node);
                    costs.insert(next_node, new_cost);
                    let mut path = paths.get(&node).unwrap().clone();
                    path.insert((node.i, node.j));
                    paths.insert(next_node, path);
                }
            }
        }
    }
    
    return 0;
}

fn problem1(input: &str) -> usize {
    resolve(input, Solution::Cost)
}

fn problem2(input: &str) -> usize {
    resolve(input, Solution::Path)
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
    fn test_problem11() {
        assert_eq!(problem1("sample11"), 7036);
    }

    #[test]
    fn test_problem12() {
        assert_eq!(problem1("sample12"), 11048);
    }

    #[test]
    fn test_problem21() {
        assert_eq!(problem2("sample11"), 45);
    }

    #[test]
    fn test_problem22() {
        assert_eq!(problem2("sample12"), 64);
    }
}