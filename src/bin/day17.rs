extern crate utils;

use utils::Opt;
use utils::read_lines;

use structopt::StructOpt;

fn input_path(file: &str) -> String {
    let day = "day17";
    return format!("input/{}/{}", day, file)
}

#[derive(Clone, Debug)]
struct Computer {
    registers: [usize;3],
    pc: usize,
    memory: Vec<usize>,
    output: Vec<usize>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            registers: [0;3],
            pc: 0,
            memory: Vec::new(),
            output: Vec::new(),
        }
    }

    fn get_combo_value(&self, op: usize) -> usize {
        match op {
            0..4 => op,
            4..7 => self.registers[op-4],
            _ => panic!("Invalid operand")
        }
    }

    fn adv(&mut self) {
        let num = self.registers[0];
        let combo = self.get_combo_value(self.memory[self.pc+1]);
        let den = 2_usize.pow(combo as u32);
        let result = num / den;
        self.registers[0] = result;
        self.pc += 2;
    }

    fn bxl(&mut self) {
        self.registers[1] = self.registers[1] ^ self.memory[self.pc+1];
        self.pc += 2;
    }

    fn bst(&mut self) {
        let combo = self.get_combo_value(self.memory[self.pc+1]);
        self.registers[1] = combo % 8;
        self.pc += 2;
    }

    fn jnz(&mut self) {
        if self.registers[0] != 0 {
            self.pc = self.memory[self.pc+1];
        } else {
            self.pc += 2;
        }
    }

    fn bxc(&mut self) {
        self.registers[1] = self.registers[1] ^ self.registers[2];
        self.pc += 2;
    }

    fn out(&mut self) {
        let combo = self.get_combo_value(self.memory[self.pc+1]);
        let result = combo % 8;
        self.output.push(result);
        self.pc += 2;
    }

    fn bdv(&mut self) {
        let num = self.registers[0];
        let combo = self.get_combo_value(self.memory[self.pc+1]);
        let den = 2_usize.pow(combo as u32);
        let result = num / den;
        self.registers[1] = result;
        self.pc += 2;
    }

    fn cdv(&mut self) {
        let num = self.registers[0];
        let combo = self.get_combo_value(self.memory[self.pc+1]);
        let den = 2_usize.pow(combo as u32);
        let result = num / den;
        self.registers[2] = result;
        self.pc += 2;
    }

    fn run(&mut self) {
        let insts= [
            Computer::adv,
            Computer::bxl,
            Computer::bst,
            Computer::jnz,
            Computer::bxc,
            Computer::out,
            Computer::bdv,
            Computer::cdv,
        ];

        while self.pc < self.memory.len() {
            let inst = self.memory[self.pc];
            insts[inst](self);
        }
    }
}

fn problem1(input: &str) -> Vec<usize> {
    let mut computer = Computer::new();
    if let Ok(lines) = read_lines(input_path(input)) {
        let mut lines = lines.flatten();
        computer.registers[0] = lines.next().unwrap().replace("Register A: ", "").parse().unwrap();
        computer.registers[1] = lines.next().unwrap().replace("Register B: ", "").parse().unwrap();
        computer.registers[2] = lines.next().unwrap().replace("Register C: ", "").parse().unwrap();
        lines.next();
        computer.memory = lines.next().unwrap().replace("Program: ", "").split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    }
    println!("Initial state: {:?}", computer);
    computer.run();
    println!("{:?}", computer.output);
    return computer.output;
}

fn problem2(input: &str) -> usize {
    let mut computer = Computer::new();
    if let Ok(lines) = read_lines(input_path(input)) {
        let mut lines = lines.flatten();
        computer.registers[0] = lines.next().unwrap().replace("Register A: ", "").parse().unwrap();
        computer.registers[1] = lines.next().unwrap().replace("Register B: ", "").parse().unwrap();
        computer.registers[2] = lines.next().unwrap().replace("Register C: ", "").parse().unwrap();
        lines.next();
        computer.memory = lines.next().unwrap().replace("Program: ", "").split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    }
    let mut a = 0;
    loop {
        println!("a={}", a);
        let mut test_computer = computer.clone();
        test_computer.registers[0] = a;
        test_computer.run();
        if test_computer.output.eq(&test_computer.memory) {
            break;
        }
        a += 1;
    }
    return a;
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
        assert_eq!(problem1("sample1"), vec![4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("sample2"), 117440);
    }
}