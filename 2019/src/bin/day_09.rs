use aoc_2019::get_input;
use aoc_2019::intcode::{parse_input, Machine};

fn main() {
    let instructions = parse_input(&get_input(9));

    let mut machine = Machine::new(instructions.clone());
    machine.inputs = vec![1];
    machine.run();
    dbg!(machine.outputs);

    let mut machine = Machine::new(instructions);
    machine.inputs = vec![2];
    machine.run();
    dbg!(machine.outputs);

}

#[test]
fn test_quine() {
    let instructions = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let mut machine = Machine::new(instructions.clone());
    machine.run();
    assert_eq!(instructions, machine.outputs);
}

#[test]
fn test_16digit() {
    let instructions = vec![1102,34915192,34915192,7,4,7,99,0];
    let mut machine = Machine::new(instructions.clone());
    machine.run();
    assert_eq!(vec![1219070632396864], machine.outputs);
}

#[test]
fn test_output_num() {
    let instructions = vec![104,1125899906842624,99];
    let mut machine = Machine::new(instructions.clone());
    machine.run();
    assert_eq!(vec![instructions[1]], machine.outputs);
}