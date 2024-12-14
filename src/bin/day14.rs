extern crate utils;

use regex::Regex;
use utils::Opt;

use std::fs;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day14";
    return format!("input/{}/{}", day, file)
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn problem1(input: &str, max_x: usize, max_y: usize) -> i64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots: Vec<_> = re.captures_iter(&puzzle).map(|c| {
        Robot {
            x: c[1].parse::<i64>().unwrap(),
            y: c[2].parse::<i64>().unwrap(),
            vx: c[3].parse::<i64>().unwrap(),
            vy: c[4].parse::<i64>().unwrap(),
        }
    }).collect();

    let iters = 100;
    let imax_x = max_x as i64;
    let imax_y = max_y as i64;

    let mut tiles = vec![vec![0;max_x];max_y];
    
    for robot in robots.iter() {
        let tx = robot.x + iters * robot.vx;
        let ty = robot.y + iters * robot.vy;
        let mx = tx % imax_x;
        let my = ty % imax_y;
        let x = if mx >= 0 { mx } else { imax_x + mx } as usize;
        let y = if my >= 0 { my } else { imax_y + my } as usize;
        tiles[y][x] += 1;
    }

    let mid_y = max_y / 2;
    let mid_x = max_x / 2;

    let mut result = 1;
    for qy in 0..2 {
        let y0 = qy * mid_y + qy;
        let y1 = y0 + mid_y;
        for qx in 0..2 {
            let x0 = qx * mid_x + qx;
            let x1 = x0 + mid_x;
            let mut qresult = 0;
            for y in y0..y1 {
                for x in x0..x1 {
                    qresult += tiles[y][x];
                }
            }
            result = result * qresult;
        }
    }

    return result;
}

fn print_tree(tree: &Vec<Vec<i64>>) {
    for row in tree.iter() {
        for &tile in row.iter() {
            if tile == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn problem2(input: &str, max_x: usize, max_y: usize) -> i64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots: Vec<_> = re.captures_iter(&puzzle).map(|c| {
        Robot {
            x: c[1].parse::<i64>().unwrap(),
            y: c[2].parse::<i64>().unwrap(),
            vx: c[3].parse::<i64>().unwrap(),
            vy: c[4].parse::<i64>().unwrap(),
        }
    }).collect();

    let imax_x = max_x as i64;
    let imax_y = max_y as i64;

    let tree_height = 3;
    let tree_width = (tree_height - 1) * 2 + 1;
    let tree_half_width = tree_width / 2;
    
    let mut i = 0;
    loop {
        let mut tiles = vec![vec![0;max_x];max_y];
        for robot in robots.iter() {
            let tx = robot.x + i * robot.vx;
            let ty = robot.y + i * robot.vy;
            let mx = tx % imax_x;
            let my = ty % imax_y;
            let x = if mx >= 0 { mx } else { imax_x + mx } as usize;
            let y = if my >= 0 { my } else { imax_y + my } as usize;
            tiles[y][x] += 1;
        }

        for y in 0..max_y-tree_height {
            for x in 0..max_x-tree_width {
                let mut tree = true;
                for i in 0..tree_height {
                    for j in tree_half_width-i..tree_half_width+i+1 {
                        let cy = y + i;
                        let cx = x + j;
                        tree &= tiles[cy][cx] > 0;
                        if !tree { break; }
                    }
                    if !tree { break; }
                }
                if tree {
                    print_tree(&tiles);
                    return i;
                }
            }
        }

        i += 1;
    }
}

fn main() {
    let opt = Opt::from_args();
    match opt.problem {
        1 => println!("Problem 1: {:?}", problem1("input", 101, 103)),
        2 => println!("Problem 2: {:?}", problem2("input", 101, 103)),
        _ => println!("No such problem"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1("sample1", 11, 7), 12);
    }

}
