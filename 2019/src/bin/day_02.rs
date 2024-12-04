use aoc_2019::get_input;
use aoc_2019::intcode::{parse_input, Machine};

const EXPECTED: i64 = 19690720;

fn main() {
    let mut instructions = parse_input(&get_input(2));

    instructions[1] = 12;
    instructions[2] = 02;
    let mut machine = Machine::new(instructions.clone());
    machine.run();
    dbg!(machine.program[0]);
    dbg!(brute_force_noun_verb(&instructions, EXPECTED));
}

fn brute_force_noun_verb(instructions: &[i64], expected: i64) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut instructions = instructions.to_vec();
            instructions[1] = noun;
            instructions[2] = verb;
            let mut machine = Machine::new(instructions);
            machine.run();
            if machine.program[0] == expected {
                return (100 * noun) + verb;
            }
        }
    }

    panic!("No solution found")
}

#[test]
fn test() {
    let checks = [
        (
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        ),
        (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
        (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
        (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
        (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
    ];

    for (left, right) in checks {
        let mut machine = Machine::new(left);
        machine.run();
        assert_eq!(right, machine.program);
    }
}
