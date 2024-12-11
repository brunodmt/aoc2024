extern crate utils;

use utils::Opt;

use std::{cmp::Ordering, collections::HashMap, collections::HashSet, fs};
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day5";
    return format!("input/{}/{}", day, file)
}

fn problem1(input: &str) -> u64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut lines = puzzle.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        let mut values = line.split('|').map(|x| x.parse::<u64>().unwrap());
        let lv = values.next().unwrap();
        let rv = values.next().unwrap();
        println!("new rule {} < {}", lv, rv);
        if rules.contains_key(&lv) {
            rules.get_mut(&lv).unwrap().push(rv);
        } else {
            rules.insert(lv, vec![rv; 1]);
        }
    }

    let mut result = 0;
    while let Some(line) = lines.next() {
        println!("line {}", line);
        let values: Vec<u64> = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        let mut visited = HashSet::new();
        let mut safe = true;
        for v in values.iter() {
            if let Some(v_rules) = rules.get(&v) {
                for rule in v_rules {
                    if visited.contains(rule) {
                        safe = false;
                        break;
                    }
                }
                if !safe { break; }
            }
            visited.insert(v);
        }
        println!("is safe {}", safe);
        if safe {
            let midpos = values.len() / 2;
            let midvalue = values[midpos];
            println!("midpos is {} with midval {}", midpos, midvalue);
            result += midvalue;
        }

    }

    return result;
}

fn problem2(input: &str) -> u64 {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut lines = puzzle.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        let mut values = line.split('|').map(|x| x.parse::<u64>().unwrap());
        let lv = values.next().unwrap();
        let rv = values.next().unwrap();
        println!("new rule {} < {}", lv, rv);
        if rules.contains_key(&lv) {
            rules.get_mut(&lv).unwrap().push(rv);
        } else {
            rules.insert(lv, vec![rv; 1]);
        }
    }

    let mut result = 0;
    while let Some(line) = lines.next() {
        println!("line {}", line);
        let mut visited = HashSet::new();
        let mut values: Vec<u64> = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        let mut safe = true;
        for v in values.iter() {
            if let Some(v_rules) = rules.get(&v) {
                for rule in v_rules {
                    if visited.contains(rule) {
                        safe = false;
                        break;
                    }
                }
                if !safe { break; }
            }
            visited.insert(v);
        }
        println!("is safe {}", safe);
        if !safe {
            // Sort the vector based on my custom ordering function
            values.sort_by(|x, y| {
                // if there are rules containing x on the left side (meaning it needs to come before other values)
                if let Some(x_rules) = rules.get(&x) {
                    // and there's a rule that says it needs to come before y
                    if x_rules.contains(&y) {
                        // then say x is Less than y so it's sorted before
                        return Ordering::Less;
                    }
                }
                // Otherwise just say they're equal, from rust doc:
                // "This sort is stable (i.e., does not reorder equal elements)"
                return Ordering::Equal;
            });
            let midpos = values.len() / 2;
            let midvalue = values[midpos];
            println!("midpos is {} with midval {}", midpos, midvalue);
            result += midvalue;
        }

    }

    return result;
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
        assert_eq!(problem1("sample1"), 143);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 0);
    }
}