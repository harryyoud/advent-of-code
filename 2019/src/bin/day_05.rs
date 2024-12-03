use aoc_2019::get_input;
use aoc_2019::intcode::{parse_input, Machine};

fn main() {
    let instructions = parse_input(&get_input(5));

    let mut machine = Machine::new(instructions.clone());
    machine.inputs = vec![1];
    machine.run();
    dbg!(machine.outputs);
    let mut machine = Machine::new(instructions.clone());
    machine.inputs = vec![5];
    machine.run();
    dbg!(machine.outputs);
}

#[test]
fn test() {
    let mut machine = Machine::new(vec![1002,4,3,4,33]);
    machine.run();
    assert_eq!(vec![1002,4,3,4,99], machine.program);
}

#[test]
fn test_eights() {
    // (program, [input], [output])
    let checks = [
        // equal to 8, position mode
        (vec![3,9,8,9,10,9,4,9,99,-1,8], vec![8], vec![1]),
        (vec![3,9,8,9,10,9,4,9,99,-1,8], vec![6], vec![0]),

        // less than 8, position mode
        (vec![3,9,7,9,10,9,4,9,99,-1,8], vec![6], vec![1]),
        (vec![3,9,7,9,10,9,4,9,99,-1,8], vec![10], vec![0]),

        // equal to 8, immediate mode
        (vec![3,3,1108,-1,8,3,4,3,99], vec![8], vec![1]),
        (vec![3,3,1108,-1,8,3,4,3,99], vec![2], vec![0]),

        // less than 8, immediate mode
        (vec![3,3,1107,-1,8,3,4,3,99], vec![6], vec![1]),
        (vec![3,3,1107,-1,8,3,4,3,99], vec![10], vec![0]),

        // less than 8 = 999, equal to 8 = 1000, larger than 8 => 1001
        (vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], vec![4], vec![999]),
        (vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], vec![8], vec![1000]),
        (vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], vec![20], vec![1001]),
    ];

    for (left, input, output) in checks {
        let mut machine = Machine::new(left.clone());
        machine.inputs = input.clone();
        machine.run();
        assert_eq!(output, machine.outputs);
    }
}
