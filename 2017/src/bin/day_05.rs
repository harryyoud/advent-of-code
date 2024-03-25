use aoc_2017::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(5);
    let instructions = parse_instructions(&input);

    dbg!(part_1(&instructions));
    dbg!(part_2(&instructions));
}

fn part_1(instructions: &[Instruction]) -> usize {
    fn jump_inc_1(cursor: &mut isize, instruction: &mut Instruction) {
        *cursor += *instruction;
        *instruction += 1;
    }

    let mut machine = Machine::new(instructions.to_owned(), jump_inc_1);
    machine.run()
}

fn part_2(instructions: &[Instruction]) -> usize {
    fn jump_dec_1_if_3(cursor: &mut isize, instruction: &mut Instruction) {
        *cursor += *instruction;
        if *instruction >= 3 {
            *instruction -= 1;
        } else {
            *instruction += 1;
        }
    }

    let mut machine = Machine::new(instructions.to_owned(), jump_dec_1_if_3);
    machine.run()
}

type Instruction = isize;

struct Machine {
    cursor: isize,
    instructions: Vec<Instruction>,
    step_count: usize,
    on_each_step: fn(&mut isize, &mut Instruction),
}

impl Machine {
    fn new(instructions: Vec<Instruction>, f: fn(&mut isize, &mut Instruction)) -> Self {
        Machine {
            instructions,
            on_each_step: f,
            step_count: 0,
            cursor: 0,
        }
    }

    fn run(&mut self) -> usize {
        loop {
            if self.cursor.is_negative() {
                break;
            }
            let Some(instruction) = self.instructions.get_mut(self.cursor.unsigned_abs()) else {
                break;
            };
            (self.on_each_step)(&mut self.cursor, instruction);
            self.step_count += 1;
        }
        self.step_count
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(|x| x.parse().ok()).collect_vec()
}
