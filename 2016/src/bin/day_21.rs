use std::collections::VecDeque;

use aoc_2016::get_input;
use itertools::Itertools;
use lazy_regex::regex_captures;

fn main() {
    let input = get_input(21);
    let input = input.trim();

    let operations = parse_input(input);

    dbg!(part_1(&operations));
    dbg!(part_2(&operations));
}

fn part_1(operations: &[Operation]) -> String {
    let mut string = "abcdefgh".chars().collect::<VecDeque<char>>();
    for operation in operations {
        apply_operation(&mut string, *operation);
    }
    return string.iter().collect::<String>();
}

fn part_2(operations: &[Operation]) -> String {
    let mut string = "fbgdceah".chars().collect::<VecDeque<char>>();
    for operation in operations.iter().rev() {
        apply_inverse_operation(&mut string, *operation);
    }
    return string.iter().collect::<String>();
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    SwapPosition(usize, usize),
    SwapChar(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateAt(char),
    ReverseAt(usize, usize),
    Move(usize, usize),
}

fn apply_operation(string: &mut VecDeque<char>, op: Operation) {
    match op {
        Operation::SwapPosition(x, y) => string.swap(x, y),
        Operation::SwapChar(x, y) => string.iter_mut().for_each(|a| {
            if *a == x {
                *a = y;
            } else if *a == y {
                *a = x;
            }
        }),
        Operation::RotateLeft(x) => string.rotate_left(x),
        Operation::RotateRight(x) => string.rotate_right(x),
        Operation::RotateAt(x) => {
            let mut pos = string.iter().position(|c| *c == x).unwrap();
            if pos >= 4 {
                pos += 1;
            }
            pos += 1;
            string.rotate_right(pos % string.len());
        }
        Operation::ReverseAt(x, y) => {
            let mut stuff = string.make_contiguous()[x..=y].to_owned();
            stuff.reverse();
            string.make_contiguous()[x..=y].copy_from_slice(&stuff);
        }
        Operation::Move(x, y) => {
            let out = string.remove(x).unwrap();
            string.insert(y, out);
        }
    };
}

fn apply_inverse_operation(string: &mut VecDeque<char>, op: Operation) {
    match op {
        Operation::SwapPosition(_, _) => apply_operation(string, op),
        Operation::SwapChar(_, _) => apply_operation(string, op),
        Operation::RotateLeft(x) => string.rotate_right(x),
        Operation::RotateRight(x) => string.rotate_left(x),
        Operation::RotateAt(x) => {
            if string.len() != 8 {
                panic!("Cannot reverse rotate operation on strings len != 8");
            }
            let shift_by = match string.iter().position(|c| *c == x).unwrap() {
                1 => 1,
                3 => 2,
                5 => 3,
                7 => 4,
                2 => 6,
                4 => 7,
                6 => 0,
                0 => 1,
                _ => panic!("String is too long!"),
            };
            string.rotate_left(shift_by % string.len());
        }
        Operation::ReverseAt(_, _) => apply_operation(string, op),
        Operation::Move(x, y) => {
            let out = string.remove(y).unwrap();
            string.insert(x, out);
        }
    };
}

fn parse_input(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(
            |line| match line.splitn(3, ' ').collect_tuple::<(_, _, _)>().unwrap() {
                ("swap", "position", _) => {
                    let (_, x, y) =
                        regex_captures!(r#"swap position (\d+) with position (\d+)"#, line)
                            .expect("Couldn't parse SwapPosition");
                    Operation::SwapPosition(x.parse().unwrap(), y.parse().unwrap())
                }
                ("swap", "letter", _) => {
                    let (_, x, y) = regex_captures!(r#"swap letter (\w) with letter (\w)"#, line)
                        .expect("Couldn't parse SwapChar");
                    Operation::SwapChar(x.chars().next().unwrap(), y.chars().next().unwrap())
                }
                ("rotate", "left", _) => {
                    let (_, x) = regex_captures!(r#"rotate left (\d+)"#, line)
                        .expect("Couldn't parse RotateLeft");
                    Operation::RotateLeft(x.parse().unwrap())
                }
                ("rotate", "right", _) => {
                    let (_, x) = regex_captures!(r#"rotate right (\d+)"#, line)
                        .expect("Couldn't parse RotateRight");
                    Operation::RotateRight(x.parse().unwrap())
                }
                ("rotate", "based", _) => {
                    let (_, x) =
                        regex_captures!(r#"rotate based on position of letter (\w)"#, line)
                            .expect("Couldn't parse RotateAt");
                    Operation::RotateAt(x.chars().next().unwrap())
                }
                ("reverse", "positions", _) => {
                    let (_, x, y) =
                        regex_captures!(r#"reverse positions (\d+) through (\d+)"#, line)
                            .expect("Couldn't parse ReverseAt");
                    Operation::ReverseAt(x.parse().unwrap(), y.parse().unwrap())
                }
                ("move", "position", _) => {
                    let (_, x, y) =
                        regex_captures!(r#"move position (\d+) to position (\d+)"#, line)
                            .expect("Couldn't parse Move");
                    Operation::Move(x.parse().unwrap(), y.parse().unwrap())
                }
                _ => panic!("Invalid operation: {line}"),
            },
        )
        .collect_vec()
}
