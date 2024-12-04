use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, Not, Shl, Shr},
};

use aoc_2015::get_input;
use itertools::Itertools;

#[derive(Debug)]
enum Operand {
    LeftShift(Wire, Wire),
    RightShift(Wire, Wire),
    Or(Wire, Wire),
    And(Wire, Wire),
    Complement(Wire),
}

#[derive(Debug)]
enum Wire {
    Number(u32),
    String(String),
}

#[derive(Debug)]
enum Input {
    Number(u32),
    Operand(Operand),
    Direct(String),
}

fn main() {
    let input = get_input(7);
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let mut cache = HashMap::new();
    let circuit = parse_input(input);

    calculate_input_recursive(&mut cache, &circuit, "a")
}

fn part_2(input: &str) -> u32 {
    let mut cache = HashMap::new();
    let mut circuit = parse_input(input);

    let a_result = calculate_input_recursive(&mut cache, &circuit, "a");

    circuit.insert("b".to_string(), Input::Number(a_result));

    let mut cache = HashMap::new();
    calculate_input_recursive(&mut cache, &circuit, "a")
}

fn parse_wire(input: &str) -> Wire {
    if input.chars().all(|x| x.is_ascii_digit()) {
        Wire::Number(input.parse().unwrap())
    } else {
        Wire::String(input.to_string())
    }
}

fn calculate_input_recursive(
    cache: &mut HashMap<String, u32>,
    circuit: &HashMap<String, Input>,
    target: &str,
) -> u32 {
    if let Some(output) = cache.get(target) {
        return *output;
    }

    let target_input = circuit.get(target).unwrap();

    let mut resolve = |a: &Wire| -> u32 {
        match a {
            Wire::Number(x) => *x,
            Wire::String(x) => calculate_input_recursive(cache, circuit, x),
        }
    };

    let output = match target_input {
        Input::Number(x) => *x,
        Input::Operand(x) => match x {
            Operand::LeftShift(a, b) => {
                let a = resolve(a);
                let b = resolve(b);
                a.shl(b)
            }
            Operand::RightShift(a, b) => {
                let a = resolve(a);
                let b = resolve(b);
                a.shr(b)
            }
            Operand::Or(a, b) => {
                let a = resolve(a);
                let b = resolve(b);
                a.bitor(b)
            }
            Operand::And(a, b) => {
                let a = resolve(a);
                let b = resolve(b);
                a.bitand(b)
            }
            Operand::Complement(a) => {
                let a = resolve(a);
                a.not()
            }
        },
        Input::Direct(x) => calculate_input_recursive(cache, circuit, x),
    };

    cache.insert(target.to_string(), output);
    output
}

fn parse_input(input: &str) -> HashMap<String, Input> {
    let mut wires: HashMap<String, Input> = HashMap::new();

    for line in input.lines() {
        let (lhs, rhs) = line.split(" -> ").collect_tuple().unwrap();
        if lhs.starts_with("NOT ") {
            let (_not, input) = lhs.split_whitespace().collect_tuple().unwrap();
            let input = parse_wire(input);
            wires.insert(rhs.to_string(), Input::Operand(Operand::Complement(input)));
        } else if lhs.contains(" AND ") {
            let (and_left, and_right) = lhs.split(" AND ").collect_tuple().unwrap();
            wires.insert(
                rhs.to_string(),
                Input::Operand(Operand::And(parse_wire(and_left), parse_wire(and_right))),
            );
        } else if lhs.contains(" OR ") {
            let (and_left, and_right) = lhs.split(" OR ").collect_tuple().unwrap();
            wires.insert(
                rhs.to_string(),
                Input::Operand(Operand::Or(parse_wire(and_left), parse_wire(and_right))),
            );
        } else if lhs.contains(" LSHIFT ") {
            let (lshift_left, lshift_right) = lhs.split(" LSHIFT ").collect_tuple().unwrap();
            wires.insert(
                rhs.to_string(),
                Input::Operand(Operand::LeftShift(
                    parse_wire(lshift_left),
                    parse_wire(lshift_right),
                )),
            );
        } else if lhs.contains(" RSHIFT ") {
            let (rshift_left, rshift_right) = lhs.split(" RSHIFT ").collect_tuple().unwrap();
            wires.insert(
                rhs.to_string(),
                Input::Operand(Operand::RightShift(
                    parse_wire(rshift_left),
                    parse_wire(rshift_right),
                )),
            );
        } else if lhs.chars().all(|x| x.is_ascii_digit()) {
            let num = lhs.parse::<u32>().unwrap();
            wires.insert(rhs.to_string(), Input::Number(num));
        } else {
            wires.insert(rhs.to_string(), Input::Direct(lhs.to_string()));
        }
    }

    wires
}
