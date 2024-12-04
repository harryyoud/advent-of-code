use aoc_2016::{
    assembunny::{self, AssembunnyMachine, Instruction, MachineResult},
    get_input,
};

fn main() {
    let input = get_input(25);
    let instructions = assembunny::parse_input(&input);

    dbg!(part_1(&instructions));
}

fn part_1(instructions: &[Instruction]) -> u32 {
    for i in 0..10000 {
        if trial(i, instructions, 10) {
            return i;
        }
    }
    panic!("Couldn't find solution");
}

fn trial(initialise_a_to: u32, instructions: &[Instruction], cycles: usize) -> bool {
    let mut machine = AssembunnyMachine::new(instructions.to_owned());
    machine.set_register('a'.into(), initialise_a_to);
    let mut target = vec![0, 1].into_iter().cycle();
    let mut seen = 0;
    loop {
        if seen >= cycles {
            return true;
        }
        if let MachineResult::OutOfInstructions = machine.step() {
            return false;
        }
        let Some(out) = machine.pop_out() else {
            continue;
        };
        seen += 1;
        if out != target.next().unwrap() {
            return false;
        }
    }
}
