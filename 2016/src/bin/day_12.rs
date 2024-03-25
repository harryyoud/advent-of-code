use aoc_2016::{assembunny::{self, AssembunnyMachine}, get_input};

fn main() {
    let input = get_input(12);
    let instructions = assembunny::parse_input(&input);

    dbg!(part_1(&instructions));
    dbg!(part_2(&instructions));
}

fn part_1(instructions: &[assembunny::Instruction]) -> u32 {
    let machine = AssembunnyMachine::new(instructions.to_owned());
    let machine = machine.execute();
    machine.get_register('a'.into())
}

fn part_2(instructions: &[assembunny::Instruction]) -> u32 {
    let mut machine = AssembunnyMachine::new(instructions.to_owned());
    machine.set_register('c'.into(), 1);
    let machine = machine.execute();
    machine.get_register('a'.into())
}
