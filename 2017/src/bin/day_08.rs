use std::collections::HashMap;

use aoc_2017::get_input;
use regex::Regex;

fn main() {
    let input = get_input(8);
    let instructions = parse_input(&input);
    let mut machine = Machine::new(instructions);
    machine.run_to_completion();

    dbg!(part_1(&machine));
    dbg!(part_2(&machine));
}

fn part_1(machine: &Machine) -> isize {
    machine.largest_value_in_registers()
}

fn part_2(machine: &Machine) -> isize {
    machine.highest_seen
}

struct Machine<'a> {
    cursor: usize,
    instructions: Vec<Instruction<'a>>,
    registers: HashMap<&'a str, isize>,
    highest_seen: isize,
}

impl Machine<'_> {
    fn new(instructions: Vec<Instruction<'_>>) -> Machine<'_> {
        Machine {
            instructions,
            cursor: 0,
            registers: HashMap::new(),
            highest_seen: 0,
        }
    }

    fn run_to_completion(&mut self) {
        while let Some(instruction) = self.instructions.get(self.cursor) {
            self.cursor += 1;

            let left = *self.registers.get(instruction.comparison_left).unwrap_or(&0);
            if !instruction.comparison.eval(left, instruction.comparison_right) {
                continue;
            }

            let register = self.registers.entry(instruction.register).or_insert(0);
            match instruction.operation {
                Operation::Increment(x) => {
                    *register += x;
                },
                Operation::Decrement(x) => {
                    *register -= x;
                },
            }

            self.highest_seen = self.highest_seen.max(*register);
        }
    }

    fn largest_value_in_registers(&self) -> isize {
        *self.registers.values().max().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Instruction<'_>> {
    let re = Regex::new(r#"(?<register>\w+)\s+(?<operation_type>inc|dec)\s+(?<operation_num>-?\d+)\s+if\s+(?<comparison_left>\w+)\s+(?<comparison_type>[!><=]+)\s+(?<comparison_right>-?\d+)"#).unwrap();
    let mut instructions = vec![];
    for line in input.lines() {
        let captures = re.captures(line).expect("Line didn't match regex");
        let operation_num = captures["operation_num"].parse().unwrap();
        instructions.push(Instruction {
            register: captures.name("register").unwrap().as_str(),
            operation: match &captures["operation_type"] {
                "inc" => Operation::Increment(operation_num),
                "dec" => Operation::Decrement(operation_num),
                _ => unreachable!("Impossible operation type"),
            },
            comparison_left: captures.name("comparison_left").unwrap().as_str(),
            comparison: match &captures["comparison_type"] {
                "==" => Comparison::Equal,
                "!=" => Comparison::NotEqual,
                ">" => Comparison::MoreThan,
                "<" => Comparison::LessThan,
                ">=" => Comparison::MoreThanOrEqual,
                "<=" => Comparison::LessThanOrEqual,
                c => panic!("Invalid comparison: {c}"),
            },
            comparison_right: captures["comparison_right"].parse().unwrap(),
        });
    }
    instructions
}

#[derive(Debug)]
struct Instruction<'a> {
    register: &'a str,
    operation: Operation,
    comparison_left: &'a str,
    comparison: Comparison,
    comparison_right: isize,
}

#[derive(Debug)]
enum Operation {
    Increment(isize),
    Decrement(isize),
}

#[derive(Debug)]
enum Comparison {
    LessThan,
    MoreThan,
    LessThanOrEqual,
    MoreThanOrEqual,
    Equal,
    NotEqual,
}

impl Comparison {
    fn eval(&self, left: isize, right: isize) -> bool {
        match self {
            Comparison::LessThan => left < right,
            Comparison::MoreThan => left > right,
            Comparison::LessThanOrEqual => left <= right,
            Comparison::MoreThanOrEqual => left >= right,
            Comparison::Equal => left == right,
            Comparison::NotEqual => left != right,
        }
    }
}