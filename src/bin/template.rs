extern crate utils;

use utils::read_lines;
use utils::Opt;

use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "dayX";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> u32 {
    if let Ok(lines) = read_lines(input_path(input)) {
        for line in lines.flatten() {
        }
    }
    return 0;
}

fn problem2(input: &str) -> u32 {
    if let Ok(lines) = read_lines(input_path(input)) {
        for line in lines.flatten() {
        }
    }
    return 0;
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
        assert_eq!(problem1("sample1"), 0);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 0);
    }
}