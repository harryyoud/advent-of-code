use std::collections::HashMap;

use aoc_2015::get_input;

fn main() {
    let input = get_input(23);
    let instructions = parse_input(&input);
    dbg!(part_1(&instructions));
    dbg!(part_2(&instructions));
}

fn part_1(instructions: &[Instruction]) -> u64 {
    let mut registers: HashMap<char, u64> = HashMap::new();
    registers.insert('a', 0_u64);
    registers.insert('b', 0_u64);
    run_program(&mut registers, instructions);
    *registers.get(&'b').unwrap()
}

fn part_2(instructions: &[Instruction]) -> u64 {
    let mut registers: HashMap<char, u64> = HashMap::new();
    registers.insert('a', 1_u64);
    registers.insert('b', 0_u64);
    run_program(&mut registers, instructions);
    *registers.get(&'b').unwrap()
}

fn run_program(registers: &mut HashMap<char, u64>, instructions: &[Instruction]) {
    let mut pointer = 0_usize;

    loop {
        match instructions.get(pointer) {
            None => break,
            Some(Instruction::Half { register }) => {
                registers.get_mut(register).map(|x| *x /= 2);
            },
            Some(Instruction::Triple { register }) => {
                registers.get_mut(register).map(|x| *x *= 3);
            }
            Some(Instruction::Increment { register }) => {
                registers.get_mut(register).map(|x| *x += 1);
            }
            Some(Instruction::Jump { amount }) => {
                pointer = ((pointer as isize) + (*amount as isize)) as usize;
                continue;
            },
            Some(Instruction::JumpIfEven { register, amount }) => {
                if registers.get(register).unwrap() % 2 == 0 {
                    pointer = ((pointer as isize) + (*amount as isize)) as usize;
                    continue;
                }
            },
            Some(Instruction::JumpIfOne { register, amount }) => {
                if *registers.get(register).unwrap() == 1 {
                    pointer = ((pointer as isize) + (*amount as isize)) as usize;
                    continue;
                }
            }
        };
        pointer += 1;
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input.lines() {
        let (instruction, detail) = line.split_once(" ").unwrap();
        instructions.push(match instruction {
            "hlf" => Instruction::Half { register: detail.chars().nth(0).unwrap() },
            "tpl" => Instruction::Triple { register: detail.chars().nth(0).unwrap() },
            "inc" => Instruction::Increment { register: detail.chars().nth(0).unwrap() },
            "jmp" => Instruction::Jump { amount: detail.parse().unwrap() },
            "jie" => {
                let (register, amount) = detail.split_once(", ").unwrap();
                Instruction::JumpIfEven {
                    register: register.chars().nth(0).unwrap(),
                    amount: amount.parse().unwrap(),
                }
            },
            "jio" => {
                let (register, amount) = detail.split_once(", ").unwrap();
                Instruction::JumpIfOne {
                    register: register.chars().nth(0).unwrap(),
                    amount: amount.parse().unwrap(),
                }
            },
            s => panic!("Invalid instruction: {s}"),
        });
    }

    instructions
}

#[derive(Debug)]
enum Instruction {
    Half { register: char },
    Triple { register: char },
    Increment { register: char },
    Jump { amount: i32 },
    JumpIfEven { register: char, amount: i32 },
    JumpIfOne { register: char, amount: i32 },
}