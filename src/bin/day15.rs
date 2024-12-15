extern crate utils;

use utils::read_lines;
use utils::Opt;

use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day15";
    return format!("input/{}/{}", day, file)
}

#[derive(Clone, Copy, Debug)]
struct Delta {
    i: isize,
    j: isize,
}

impl Delta {
    fn new(i: isize, j: isize) -> Delta {
        Delta { i, j }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn next(&self, delta: Delta) -> Option<Point> {
        Some(Point {
            i: self.i.checked_add_signed(delta.i)?,
            j: self.j.checked_add_signed(delta.j)?
        })
    }

    fn prev(&self, delta: Delta) -> Option<Point> {
        Some(Point {
            i: self.i.checked_add_signed(-delta.i)?,
            j: self.j.checked_add_signed(-delta.j)?
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Box,
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'O' => Tile::Box,
            '@' => Tile::Empty,
            _ => panic!("Tile not recognized"),
        }   
    }

    fn double_from_char(c: char) -> [Tile;2] {
        match c {
            'O' => [Tile::BoxLeft, Tile::BoxRight],
            _ => {
                let tile = Tile::from_char(c);
                [tile, tile]
            }
        }   
    }
}

enum Inst {
    Down,
    Left,
    Right,
    Up,
}


impl Inst {
    fn from_char(c: char) -> Inst {
        match c {
            'v' => Inst::Down,
            '<' => Inst::Left,
            '>' => Inst::Right,
            '^' => Inst::Up,
            _ => panic!("Instruction not recognized"),
        }
    }

    fn get_delta(&self) -> Delta {
        match self {
            Inst::Down => Delta::new(1,0),
            Inst::Left => Delta::new(0,-1),
            Inst::Right => Delta::new(0,1),
            Inst::Up => Delta::new(-1,0),
        }
    }
}

fn print_warehouse(warehouse: &Vec<Vec<Tile>>, robot: &Point) {
    for (i, row) in warehouse.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            match col {
                Tile::Box => print!("O"),
                Tile::Empty => if i == robot.i && j == robot.j { 
                    print!("@");
                } else {
                    print!(".");
                },
                Tile::Wall => print!("#"),
                Tile::BoxLeft => print!("["),
                Tile::BoxRight => print!("]"),
            }
        }
        println!();
    }
}

fn problem1(input: &str) -> u64 {
    let mut warehouse: Vec<Vec<Tile>> = Vec::new();
    let mut instructions: Vec<Inst> = Vec::new();
    let mut robot = Point { i: 0, j: 0};

    let mut warehouse_parsing = true;
    if let Ok(lines) = read_lines(input_path(input)) {
        for (i, line) in lines.flatten().enumerate() {
            match warehouse_parsing {
                true => {
                    if line.is_empty() {
                        warehouse_parsing = false;
                    } else {
                        let row = line.chars().enumerate().map(|(j,c)| {
                            if c == '@' { robot.i = i; robot.j = j }
                            Tile::from_char(c)
                        }).collect();
                        warehouse.push(row);
                    }
                },
                false => {
                    let row: Vec<Inst> = line.chars().map(Inst::from_char).collect();
                    instructions.extend(row);
                },
            }
        }
    }

    for inst in instructions {
        let delta = inst.get_delta();
        let mut next = robot.next(delta).unwrap();
        let mut next_tile = &warehouse[next.i][next.j];
        while !matches!(next_tile, Tile::Wall) && !matches!(next_tile, Tile::Empty) {
            next = next.next(delta).unwrap();
            next_tile = &warehouse[next.i][next.j];
        }
        if matches!(next_tile, Tile::Empty) {
            while next.i != robot.i || next.j != robot .j {
                let prev = next.prev(delta).unwrap();
                warehouse[next.i][next.j] = warehouse[prev.i][prev.j];
                next = prev;
            }
            robot = robot.next(delta).unwrap();
        }
        //print_warehouse(&warehouse, &robot);
    }

    let result = warehouse.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, col)| {
            match col {
                Tile::Box => 100 * (i as u64) + (j as u64),
                _ => 0,
            }
        }).sum::<u64>()
    }).sum();

    result
}

fn can_move(p: Point, inst: &Inst, warehouse: &Vec<Vec<Tile>>) -> bool {
    let d = inst.get_delta();
    let next = p.next(d);
    if let Some(next) = next {
        return match warehouse[next.i][next.j] {
            Tile::Wall => false,
            Tile::Empty => true,
            Tile::Box => can_move(next, inst, warehouse),
            Tile::BoxLeft => {
                let mut result = can_move(next, inst, warehouse);
                if matches!(inst, Inst::Up) || matches!(inst, Inst::Down) {
                    result = result && can_move(Point { i: next.i, j: next.j + 1 }, inst, warehouse);
                }
                return result;
            },
            Tile::BoxRight => {
                let mut result = can_move(next, inst, warehouse);
                if matches!(inst, Inst::Up) || matches!(inst, Inst::Down) {
                    result = result && can_move(Point { i: next.i, j: next.j - 1 }, inst, warehouse);
                }
                return result;
            }
        }
    }
    false
}

fn move_tile(p: Point, inst: &Inst, warehouse: &mut Vec<Vec<Tile>>) {
    let d = inst.get_delta();
    let next = p.next(d).unwrap();
    match warehouse[next.i][next.j] {
        Tile::Wall => panic!("Moved into a wall!"),
        Tile::Empty => { },
        Tile::Box => move_tile(next, inst, warehouse),
        Tile::BoxLeft => {
            if matches!(inst, Inst::Up) || matches!(inst, Inst::Down) {
                move_tile(Point { i: next.i, j: next.j + 1 }, inst, warehouse);
            }
            move_tile(next, inst, warehouse);
        },
        Tile::BoxRight => {
            if matches!(inst, Inst::Up) || matches!(inst, Inst::Down) {
                move_tile(Point { i: next.i, j: next.j - 1 }, inst, warehouse);
            }
            move_tile(next, inst, warehouse);
        }
    }
    warehouse[next.i][next.j] = warehouse[p.i][p.j];
    warehouse[p.i][p.j] = Tile::Empty;
}

fn problem2(input: &str) -> u64 {
    let mut warehouse: Vec<Vec<Tile>> = Vec::new();
    let mut instructions: Vec<Inst> = Vec::new();
    let mut robot = Point { i: 0, j: 0};

    let mut warehouse_parsing = true;
    if let Ok(lines) = read_lines(input_path(input)) {
        for (i, line) in lines.flatten().enumerate() {
            match warehouse_parsing {
                true => {
                    if line.is_empty() {
                        warehouse_parsing = false;
                    } else {
                        
                        let row = line.chars().enumerate().map(|(j,c)| {
                            if c == '@' { robot.i = i; robot.j = j * 2 }
                            Tile::double_from_char(c)
                        }).flatten().collect();
                        warehouse.push(row);
                    }
                },
                false => {
                    let row: Vec<Inst> = line.chars().map(Inst::from_char).collect();
                    instructions.extend(row);
                },
            }
        }
    }

    for inst in instructions {
        let delta = inst.get_delta();
        if can_move(robot, &inst, &warehouse) {
            move_tile(robot, &inst, &mut warehouse);
            robot = robot.next(delta).unwrap();
        }
        //print_warehouse(&warehouse, &robot);
    }

    let result = warehouse.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, col)| {
            match col {
                Tile::BoxLeft => {
                    return 100 * (i as u64) + (j as u64)},
                _ => 0,
            }
        }).sum::<u64>()
    }).sum();

    result
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
        assert_eq!(problem1("sample11"), 2028);
    }

    #[test]
    fn test_problem12() {
        assert_eq!(problem1("sample12"), 10092);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 105);
    }

    #[test]
    fn test_problem22() {
        assert_eq!(problem2("sample12"), 9021);
    }
}