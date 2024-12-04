use aoc_2020::get_input;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = get_input(8);

    let instructions: Vec<Instruction> = input
        .lines()
        .map(Instruction::try_from)
        .try_collect()
        .expect("Invalid instructions");

    dbg!(part_1(instructions.clone()));
    dbg!(part_2(instructions.clone()));
}

fn part_1(instructions: Vec<Instruction>) -> isize {
    let mut machine = Machine::new(instructions);
    assert_eq!(machine.run_until(), ExitCondition::LoopDetected);
    machine.accumulator
}

fn part_2(instructions: Vec<Instruction>) -> isize {
    for instructions in InstructionsChanger::new(instructions) {
        let mut machine = Machine::new(instructions);
        if matches!(machine.run_until(), ExitCondition::EndOfInstructions) {
            return machine.accumulator;
        }
    }

    unreachable!("No solution found")
}

struct InstructionsChanger {
    original_instructions: Vec<Instruction>,
    cursor: usize,
}

impl InstructionsChanger {
    fn new(original_instructions: Vec<Instruction>) -> Self {
        Self {
            original_instructions,
            cursor: 0,
        }
    }
}

// Step through the original instructions and on each iteration produce the instructions with one of
// jump swapped with noop or noop with jump
impl Iterator for InstructionsChanger {
    type Item = Vec<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut instructions = self.original_instructions.clone();

        while self.cursor < instructions.len() {
            match instructions[self.cursor] {
                Instruction::Jump(x) => {
                    instructions[self.cursor] = Instruction::NoOp(x);
                    self.cursor += 1;
                    return Some(instructions);
                }
                Instruction::NoOp(x) => {
                    instructions[self.cursor] = Instruction::Jump(x);
                    self.cursor += 1;
                    return Some(instructions);
                }
                _ => {
                    self.cursor += 1;
                }
            }
        }
        None
    }
}

struct Machine {
    instructions: Vec<Instruction>,
    cursor: usize,
    visited: HashSet<usize>,
    accumulator: isize,
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            cursor: 0,
            accumulator: 0,
            visited: HashSet::new(),
        }
    }

    fn run_until(&mut self) -> ExitCondition {
        loop {
            if self.visited.contains(&self.cursor) {
                return ExitCondition::LoopDetected;
            }
            if self.cursor >= self.instructions.len() {
                return ExitCondition::EndOfInstructions;
            }
            self.visited.insert(self.cursor);
            self.execute_single();
        }
    }

    fn execute_single(&mut self) {
        match self.instructions[self.cursor] {
            Instruction::Jump(v) => {
                self.cursor = self.cursor.saturating_add_signed(v);
            }
            Instruction::NoOp(_) => {
                self.cursor += 1;
            }
            Instruction::Accumulator(v) => {
                self.accumulator += v;
                self.cursor += 1;
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ExitCondition {
    EndOfInstructions,
    LoopDetected,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Jump(isize),
    NoOp(isize),
    Accumulator(isize),
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((ins_type, value)) = value.split_once(" ") else {
            return Err(());
        };
        let Ok(value) = value.parse::<isize>() else {
            return Err(());
        };

        match ins_type {
            "jmp" => Ok(Instruction::Jump(value)),
            "nop" => Ok(Instruction::NoOp(value)),
            "acc" => Ok(Instruction::Accumulator(value)),
            _ => Err(()),
        }
    }
}
