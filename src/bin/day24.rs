extern crate utils;

use utils::Opt;

use regex::Regex;
use std::{collections::HashMap, fs};
use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day24";
    return format!("input/{}/{}", day, file)
}

#[derive(Clone,Debug,PartialEq)]
enum Operator {
    AND,
    OR,
    XOR,
}

#[derive(Clone,Debug)]
struct Expression {
    op1: String,
    op2: String,
    operator: Operator,
}

struct Puzzle {
    values: HashMap<String, bool>,
    pending: HashMap<String, Expression>,
    z_size: usize,
}

fn resolve(wire: String, values: &mut HashMap<String, bool>, pending: &mut HashMap<String, Expression>) -> bool {
    let maybe_expr = pending.remove(&wire);

    if let Some(expr) = maybe_expr {
        let op1 = resolve(expr.op1.clone(), values, pending);
        let op2 = resolve(expr.op2.clone(), values, pending);
        let value = match expr.operator {
            Operator::AND => op1 & op2,
            Operator::OR => op1 | op2,
            Operator::XOR => op1 ^ op2,
        };
        values.insert(wire, value);
        return value;
    } else if let Some(&value) = values.get(&wire) {
        value
    } else {
        panic!("value for {wire} not found");
    }
}

fn parse_input(input: &str) -> Puzzle {
    let puzzle: String = fs::read_to_string(input_path(input)).unwrap();
    let mut puzzle_it = puzzle.lines();

    let mut z_size: usize = 0;
    let mut values = HashMap::new();
    let val_re = Regex::new(r"^([a-z0-9]{3}): (0|1)$").unwrap();
    while let Some(line) = puzzle_it.next() {
        if line.is_empty() { break; }
        let caps = val_re.captures(&line).unwrap();
        let name = caps[1].to_string();
        let value = match &caps[2] {
            "0" => false,
            "1" => true,
            _ => panic!("unexpected value {}", &caps[2])
        };
        values.insert(name, value);    
    }

    let mut pending = HashMap::new();
    let exp_re = Regex::new(r"^([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})$").unwrap();
    while let Some(line) = puzzle_it.next() {
        let caps = exp_re.captures(&line).unwrap();
        let op1 = &caps[1];
        let operator = match &caps[2] {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "XOR" => Operator::XOR,
            _ => panic!("unexpected operator {}", &caps[2])
        };
        let op2 = &caps[3];
        let name = caps[4].to_string();
        if name.starts_with("z") {
            z_size += 1;
        }
        pending.insert(name, Expression {
            op1: op1.to_string(),
            op2: op2.to_string(),
            operator,
        });
    }

    Puzzle {
        values,
        pending,
        z_size,
    }
}

fn problem1(input: &str) -> u64 {
    let mut puzzle = parse_input(input);

    let mut result = 0;
    for i in 0..puzzle.z_size {
        let wire = format!("z{:02}", i);
        let wire_val =  if resolve(wire, &mut puzzle.values, &mut puzzle.pending) { 1 } else { 0 };
        result += wire_val<<i;
    }

    return result;
}

fn problem2(input: &str) -> String {
    let puzzle = parse_input(input);
    let mut result: Vec<String> = Vec::new();

    let last_z = format!("z{:02}", puzzle.z_size-1);

    let inputs = puzzle.pending.iter().filter(|(_,v)| {
        (v.op1.starts_with("x") && v.op1 != "x00" && v.op2.starts_with("y"))
        ||
        (v.op1.starts_with("y") && v.op1 != "y00" && v.op2.starts_with("x"))
    });

    let input_xors = inputs.clone().filter(|(_,v)| v.operator == Operator::XOR);
    let input_ands = inputs.clone().filter(|(_,v)| v.operator == Operator::AND);

    // Z outputs must come from XOR, except the last one (carry)
    let rule1 = puzzle.pending.iter().filter(|&(k,v)| {
        k.starts_with("z") && **k != last_z && v.operator != Operator::XOR
    }).map(|(k,_)| (*k).clone());
    result.extend(rule1);

    // XOR can only come up in inputs or outputs
    let rule2 = puzzle.pending.iter().filter(|&(k,v)| {
        v.operator == Operator::XOR && !k.starts_with("z") && !input_xors.clone().any(|(k1,_)| k1 == k)
    }).map(|(k,_)| (*k).clone());
    result.extend(rule2);

    // Output of an input XOR must be use in another XOR
    let rule3 = input_xors.filter(|(k,_)| {
        !puzzle.pending.iter().any(|(_,v1)| {
            v1.operator == Operator::XOR && (v1.op1 == **k || v1.op2 == **k)
        })
    }).map(|(k,_)| (*k).clone());
    result.extend(rule3);

    // Output of an input XOR must be use in an AND
    let rule4 = input_ands.filter(|(k,_)| {
        !puzzle.pending.iter().any(|(_,v1)| {
            v1.operator == Operator::OR && (v1.op1 == **k || v1.op2 == **k)
        })
    }).map(|(k,_)| (*k).clone());
    result.extend(rule4);

    result.sort();
    result.dedup();

    return result.join(",");
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
        assert_eq!(problem1("sample11"), 4);
    }

    #[test]
    fn test_problem12() {
        assert_eq!(problem1("sample12"), 2024);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), "z00,z01,z02,z05");
    }
}