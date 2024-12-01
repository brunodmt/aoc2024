extern crate utils;

use utils::read_lines;
use utils::Opt;

use regex::Regex;
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day1";
    return format!("input/{}/{}", day, file)
}

fn load_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    if let Ok(lines) = read_lines(input_path(input)) {
        for line in lines.flatten() {
            let re = Regex::new(r"^([0-9]+)   ([0-9]+)$").unwrap();
            let caps = re.captures(&line).unwrap();
            let first_value = caps[1].parse::<u64>().unwrap();
            first_list.push(first_value);
            let second_value = caps[2].parse::<u64>().unwrap();
            second_list.push(second_value);
        }
    }

    first_list.sort();
    second_list.sort();
    return (first_list, second_list);
}

fn problem1(input: &str) -> u64 {
    let (first_list, second_list) = load_lists(input);
    let mut result = 0;
    let mut first_iterator = first_list.iter();
    let mut second_iterator = second_list.iter();
    while let Some(first) = first_iterator.next() {
        let second = second_iterator.next().unwrap();
        let diff = first.abs_diff(*second);
        result += diff;
    }

    return result;
}

fn problem2(input: &str) -> u64 {
    let (first_list, second_list) = load_lists(input);
    println!("First list is {:?}", first_list);
    println!("Second list is {:?}", second_list);
    let mut result = 0;
    let mut first_iterator = first_list.iter();
    let mut second_iterator = second_list.iter();
    let mut second = second_iterator.next().unwrap();
    let mut previous_first = 0;
    let mut previous_count = 0;
    while let Some(first) = first_iterator.next() {
        let mut count = 0;
        if *first == previous_first {
            count = previous_count;
        } else {
            while second < first {
                if let Some(new_second) = second_iterator.next() {
                    second = new_second;
                } else {
                    break;
                }
            }
            while second == first {
                count +=1;
                if let Some(new_second) = second_iterator.next() {
                    second = new_second;
                } else {
                    break;
                }
            }
        }
        println!("Number {} appears {}", first, count);
        result += first * count;
        previous_first = *first;
        previous_count = count;
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
        assert_eq!(problem1("sample1"), 11);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 31);
    }
}