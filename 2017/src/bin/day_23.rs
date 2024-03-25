use aoc_2017::{duet_asm::{parsing::parse_instructions, Instruction, Operand, Registers}, get_input};

fn main() {
    let input = get_input(23);
    let instructions = parse_instructions(&input).into_iter().collect::<Result<Vec<_>, _>>().unwrap();

    dbg!(part_1(instructions.clone()));
    dbg!(part_2(instructions.clone()));
}

fn part_1(instructions: Vec<Instruction>) -> usize {
    let mut machine = Machine {
        instructions,
        multiply_instruction_count: 0,
        registers: Registers::new(),
        cursor: 0,
    };

    machine.run_to_completion();

    machine.multiply_instruction_count
}

fn part_2(instructions: Vec<Instruction>) -> isize {
    optimised(match instructions[0] {
        Instruction::Set('b', Operand::Value(x)) => x,
        _ => panic!("Expected set b as first instruction"),
    })
}

struct Machine {
    instructions: Vec<Instruction>,
    multiply_instruction_count: usize,
    registers: Registers,
    cursor: isize,
}

impl Machine {
    fn run_to_completion(&mut self) {
        if self.cursor < 0 {
            return;
        }
        while let Some(instruction) = self.instructions.get(self.cursor as usize) {
            use Instruction::*;
            match instruction {
                Set(x, y) => {
                    self.registers.insert(*x, self.eval(*y));
                },
                SubtractAssign(x, y) => {
                    *self.registers.entry(*x).or_insert(0) -= self.eval(*y);
                }
                MultiplyAssign(x, y) => {
                    *self.registers.entry(*x).or_insert(0) *= self.eval(*y);
                    self.multiply_instruction_count += 1;
                },
                JumpIfNotZero(x, y) => {
                    if self.eval(*x) != 0 {
                        self.cursor += self.eval(*y) - 1;
                    }
                },
                _ => panic!("Invalid instruction"),
            }
            self.cursor += 1;
        }
    }

    pub fn eval(&self, operand: Operand) -> isize {
        use Operand::*;
        match operand {
            RegisterName(x) => self.registers.get(&x).copied().unwrap_or(0),
            Value(x) => x,
        }
    }
}

fn optimised(initial_b: isize) -> isize {
    // optimised input

    let mut b = (initial_b * 100) + 100_000;
    let c = b + 17_000;
    let mut h = 0;

    loop {
        let upper = (b as f64).sqrt() as isize + 1;

        for d in 2..upper {
            if b % d == 0 {
                h += 1;
                break;
            }
        }

        if b == c {
            break;
        }

        b += 17;
    }

    h
}