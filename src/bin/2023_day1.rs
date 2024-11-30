extern crate utils;

use utils::read_lines;
use utils::Opt;

use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "2023_day1";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> u32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(input_path(input)) {
        for line in lines.flatten() {
            let mut is_first = true;
            let mut last = 0;
            for c in line.chars() { 
                if c.is_digit(10) {
                    let d = c.to_digit(10).unwrap();
                    if is_first {
                        result = result + (d * 10);
                        last = d;
                        is_first = false;
                    } else {
                        last = d;
                    }
                }
            }
            result = result + last;
        }
    }
    return result;
}

struct Digit<'a> {
    digit: u32,
    text: &'a str,
    length: usize,
    current: usize,
}

impl Digit<'_> {
    fn new(digit: u32, text: &str) -> Digit {
        Digit { digit, text, length: text.len(), current: 0 }
    }

    fn feed_and_check(self: &mut Self, c: char) -> bool {
        if c == self.text.chars().nth(self.current).unwrap() {
            self.current += 1;
        } else if c == self.text.chars().nth(0).unwrap() {
            self.current = 1;
        } else {
            self.current = 0;
        }

        if self.current == self.length {
            self.current = 0;
            return true;
        } else {
            return false;
        }
    }
    
    fn reset(self: &mut Self) {
        self.current = 0;
    }
}

fn problem2(input: &str) -> u32 {
    let mut result = 0;
    let mut digits = [
        Digit::new(1, "one"),
        Digit::new(2, "two"),
        Digit::new(3, "three"),
        Digit::new(4, "four"),
        Digit::new(5, "five"),
        Digit::new(6, "six"),
        Digit::new(7, "seven"),
        Digit::new(8, "eight"),
        Digit::new(9, "nine"),
    ];
    if let Ok(lines) = read_lines(input_path(input)) {
        for line in lines.flatten() {
            let mut line_result = 0;
            for d in digits.iter_mut() {
                d.reset();
            }
            let mut is_first = true;
            let mut last = 0;
            for c in line.chars() { 
                if c.is_digit(10) {
                    let d = c.to_digit(10).unwrap();
                    if is_first {
                        line_result = line_result + (d * 10);
                        is_first = false;
                    }
                    last = d;
                } else {
                    for d in digits.iter_mut() {
                        let is_match = d.feed_and_check(c);
                        if is_match {
                            if is_first {
                                line_result = line_result + (d.digit * 10);
                                is_first = false;
                            }
                            last = d.digit;
                        }
                    }
                }
            }
            line_result += last;
            result = result + line_result;
        }
    }
    return result;
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
        assert_eq!(problem1("sample1"), 142);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 281);
    }
}