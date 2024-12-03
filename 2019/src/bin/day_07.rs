use itertools::Itertools;
use aoc_2019::get_input;
use aoc_2019::intcode::{parse_input, Machine, MachineState};

const MACHINE_COUNT: usize = 5;

fn main() {
    let instructions = parse_input(&get_input(7));

    dbg!(part_1(&instructions));
    dbg!(part_2(&instructions));
}

fn part_1(instructions: &[i64]) -> i64 {
    let mut phase_combinations = (0..=4).permutations(MACHINE_COUNT);
    let mut max = 0;
    for inputs in phase_combinations {
        let mut last = 0;
        for x in 0..MACHINE_COUNT {
            let mut machine = Machine::new(instructions.to_vec());
            machine.inputs.extend(vec![inputs[x], last]);
            machine.run();
            last = *machine.outputs.last().unwrap();
        }
        max = last.max(max);
    }
    max
}

fn part_2(instructions: &[i64]) -> i64 {
    let mut phase_combinations = (5..=9).permutations(MACHINE_COUNT);
    let mut max = 0;

    for inputs in phase_combinations {
        let mut machines = vec![Machine::new(instructions.to_vec()); MACHINE_COUNT];
        for (machine, input) in machines.iter_mut().zip(inputs) {
            machine.inputs.push(input);
        }
        let mut last = 0;
        loop {
            for machine in machines.iter_mut() {
                machine.inputs.push(last);
                machine.run();
                last = *machine.outputs.last().unwrap();
            }
            if machines.iter().all(|x| matches!(x.state, MachineState::HitStopInstruction)) {
                break;
            }
        }
        max = max.max(*machines.last().unwrap().outputs.last().unwrap());
    }
    max
}