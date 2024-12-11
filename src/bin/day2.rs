extern crate utils;

use utils::read_lines;
use utils::Opt;

use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day2";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> u32 {
    let mut safe_count = 0;
    if let Ok(lines) = read_lines(input_path(input)) {
        for line in lines.flatten() {
            println!("new line");
            let levels = line.split_whitespace();
            let mut it = levels.into_iter();
            let mut safe = true;
            let mut direction = 0;
            let mut previous = it.next().unwrap().parse::<i32>().unwrap();
            for str_level in it {
                let level = str_level.parse::<i32>().unwrap();
                println!("previous={} level={}", previous, level);
                if direction > 0 && level < previous {
                    println!("exit cond 1");
                    safe = false;
                    break;
                } else if direction < 0 && level > previous {
                    println!("exit cond 2");
                    safe = false;
                    break;
                }
                let diff = level - previous;
                let abs_diff = diff.abs();
                if abs_diff < 1 || abs_diff > 3 {
                    println!("exit cond 3, diff={}", diff);
                    safe = false;
                    break;
                }
                if direction == 0 {
                    println!("set diff={}", diff);
                    direction = diff;
                }
                previous = level;
            }
            if safe {
                safe_count += 1;
            }
        }
    }
    return safe_count;
}

fn check_line(line: &String, skip: i32) -> bool {
    let levels = line.split_whitespace();
    let mut it = levels.into_iter();
    let mut safe = true;
    let mut direction = 0;
    let mut previous = it.next().unwrap().parse::<i32>().unwrap();
    if skip == 0 {
        previous = it.next().unwrap().parse::<i32>().unwrap();
    }
    let mut position = 0;
    for str_level in it {
        position += 1;
        if position == skip {
            continue;
        }
        let level = str_level.parse::<i32>().unwrap();
        if direction > 0 && level < previous {
            println!("previous={} level={} exit cond 1", previous, level);
            safe = false;
            break;
        } else if direction < 0 && level > previous {
            println!("previous={} level={} exit cond 2", previous, level);
            safe = false;
            break;
        }
        let diff = level - previous;
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            println!("previous={} level={} exit cond 3", previous, level);
            safe = false;
            break;
        }
        if direction == 0 {
            println!("set diff={}", diff);
            direction = diff;
        }
        previous = level;
    }
    return safe;
}

fn problem2(input: &str) -> u32 {
    let mut safe_count = 0;
    if let Ok(lines) = read_lines(input_path(input)) {
        for line in lines.flatten() {
            println!("{}", line);
            let safe = check_line(&line, -1);
            if safe {
                println!("is safe");
                safe_count += 1;
            } else {
                println!("is not safe");
                let count: i32 = line.split_whitespace().count().try_into().unwrap();
                let mut n: i32 = 0;
                let mut safe = false;
                while !safe && n < count {
                    println!("is not safe, retry skip {}", n);
                    safe = check_line(&line, n);
                    n += 1;
                }
                if safe {
                    println!("is safe");
                    safe_count += 1;
                } else {
                    println!("is not safe but no more retries");
                }
            }
            println!();
        }
    }
    return safe_count;
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
        assert_eq!(problem1("sample1"), 2);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 4);
    }
}