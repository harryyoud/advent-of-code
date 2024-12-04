use aoc_2016::assembunny;
use aoc_2016::assembunny::AssembunnyMachine;
use aoc_2016::get_input;

fn main() {
    let input = get_input(23);
    let instructions = assembunny::parse_input(&input);

    dbg!(part_1(&instructions));
    dbg!(part_2(&instructions));
}

fn part_1(instructions: &[assembunny::Instruction]) -> u32 {
    let mut machine = AssembunnyMachine::new(instructions.to_owned());
    machine.set_register('a'.into(), 7);
    let machine = machine.execute();
    machine.get_register('a'.into())
}

fn part_2(instructions: &[assembunny::Instruction]) -> u32 {
    let mut machine = AssembunnyMachine::new(instructions.to_owned());
    machine.set_register('a'.into(), 12);
    let machine = machine.execute();
    machine.get_register('a'.into())
}
