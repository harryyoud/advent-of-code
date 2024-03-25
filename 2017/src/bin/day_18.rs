use std::collections::VecDeque;

use aoc_2017::{duet_asm::{parsing::parse_instructions, Instruction, RegisterName, Registers}, get_input};
use itertools::Itertools;

fn main() {
    let input = get_input(18);
    let instructions = parse_instructions(&input);
    let instructions = instructions.into_iter().map(|x| x.unwrap()).collect_vec();

    dbg!(part_1(instructions.clone()));

    let instructions = instructions.into_iter()
        .map(|x| if matches!(x, Instruction::Recover(_)) { x.try_as_receive().unwrap() } else { x })
        .collect_vec();

    dbg!(part_2(instructions));
}

fn part_1(instructions: Vec<Instruction>) -> isize {
    let mut sound_card = SoundCard::new(instructions);
    sound_card.run_until_recover().unwrap()
}

fn part_2(instructions: Vec<Instruction>) -> usize {
    let mut sound_card_0 = SoundCard::new(instructions.clone());
    let mut sound_card_1 = SoundCard::new(instructions);
    sound_card_1.registers.insert('p', 1);

    let mut sound_card_1_sends = 0;

    loop {
        use ExecutionResult::*;

        match (sound_card_0.tick(), sound_card_1.tick()) {
            (EndOfInstructions, _) => break,
            (_, EndOfInstructions) => break,
            (Waiting, Waiting) => break,
            _ => {},
        }

        sound_card_1_sends += sound_card_1.sounds_out.len();

        sound_card_1.sounds_in.extend(sound_card_0.sounds_out.drain(..).rev());
        sound_card_0.sounds_in.extend(sound_card_1.sounds_out.drain(..).rev());
    }

    sound_card_1_sends
}

#[derive(Debug, Default)]
struct SoundCard {
    sounds_out: Vec<Sound>,
    sounds_in: VecDeque<Sound>,
    registers: Registers,
    cursor: isize,
    instructions: Vec<Instruction>,
}

enum ExecutionResult {
    EndOfInstructions,
    Ok,
    Recover(isize),
    FailedRecover,
    Waiting,
}

impl SoundCard {
    fn new(instructions: Vec<Instruction>) -> Self {
        SoundCard {
            instructions,
            ..Default::default()
        }
    }

    fn run_until_recover(&mut self) -> Result<isize, ()> {
        loop {
            match self.tick() {
                ExecutionResult::Ok => continue,
                ExecutionResult::Waiting => continue,
                ExecutionResult::EndOfInstructions => return Err(()),
                ExecutionResult::FailedRecover => return Err(()),
                ExecutionResult::Recover(x) => return Ok(x),
            };
        }
    }

    fn tick(&mut self) -> ExecutionResult {
        if self.cursor < 0 || self.cursor >= self.instructions.len() as isize {
            return ExecutionResult::EndOfInstructions;
        }
        
        let instruction = &self.instructions[self.cursor as usize];
        let result;

        match instruction {
            Instruction::Send(x) => {
                self.sounds_out.push(x.eval(&self.registers));
                result = ExecutionResult::Ok;
            },
            Instruction::Set(x, y) => {
                self.registers.insert(*x, y.eval(&self.registers));
                result = ExecutionResult::Ok;
            },
            Instruction::AddAssign(x, y) => {
                let y = y.eval(&self.registers);
                *self.get_register(*x) += y;
                result = ExecutionResult::Ok;
            },
            Instruction::MultiplyAssign(x, y) => {
                let y = y.eval(&self.registers);
                *self.get_register(*x) *= y;
                result = ExecutionResult::Ok;
            },
            Instruction::ModuloAssign(x, y) => {
                let y = y.eval(&self.registers);
                *self.get_register(*x) %= y;
                result = ExecutionResult::Ok;
            },
            Instruction::Recover(x) => {
                let x = x.eval(&self.registers);
                if x != 0 {
                    if let Some(last_sound) = self.sounds_out.pop() {
                        result = ExecutionResult::Recover(last_sound);
                    } else {
                        result = ExecutionResult::FailedRecover;
                    }
                } else {
                    result = ExecutionResult::Ok;
                }
            },
            Instruction::Receive(x) => {
                if let Some(sound) = self.sounds_in.pop_front() {
                    *self.get_register(*x) = sound;
                    result = ExecutionResult::Ok;
                } else {
                    // return early so we don't advance the
                    // cursor until we can receive
                    return ExecutionResult::Waiting;
                }
            }
            Instruction::JumpIfGreaterThanZero(x, y) => {
                let x = x.eval(&self.registers);
                let y = y.eval(&self.registers);
                if x > 0 {
                    self.cursor += y;
                    // return early so we don't advance the cursor
                    return ExecutionResult::Ok;
                }
                result = ExecutionResult::Ok;
            },
            _ => panic!("Invalid instruction"),
        };

        self.cursor += 1;
        result
    }

    fn get_register(&mut self, x: RegisterName) -> &mut isize {
        self.registers.entry(x).or_insert(0)
    }
}

type Sound = isize;
