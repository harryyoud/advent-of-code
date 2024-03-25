use std::{collections::{HashMap, VecDeque}, str::FromStr};
use aoc_2017::get_input;
use aoc_lib::Paragraphs;
use regex::Regex;

fn main() {
    let input = get_input(25);
    let (starting_state, checksum_step, states) = parse_input(&input);

    dbg!(part_1(starting_state, checksum_step, states));
}

fn part_1(starting_state: StateKey, checksum_step: usize, states: HashMap<StateKey, State>) -> usize {
    let mut machine = Machine::new(starting_state, states);
    machine.run_n(checksum_step);
    machine.tape.iter().filter(|x| **x).count()
}

struct Machine {
    tape: VecDeque<bool>,
    cursor: usize,
    states: HashMap<StateKey, State>,
    next_state: StateKey,
}

impl Machine {
    fn new(starting_state: StateKey, states: HashMap<StateKey, State>) -> Self {
        Machine {
            tape: VecDeque::from_iter([false]),
            cursor: 0,
            states,
            next_state: starting_state,
        }
    }

    fn run_n(&mut self, iterations: usize) {
        for _ in 0..iterations {
            let current_value = &mut self.tape[self.cursor];
            let current_state = &self.states[&self.next_state];

            let instruction = if *current_value {
                &current_state.if_true
            } else {
                &current_state.if_false
            };

            *current_value = instruction.write;
            self.next_state = instruction.next_state;
            match instruction.direction {
                Direction::Left => self.move_left(),
                Direction::Right => self.move_right(),
            }
        }
    }

    fn move_left(&mut self) {
        if self.cursor == 0 {
            self.tape.push_front(false);
        } else {
            self.cursor -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor == self.tape.len() - 1 {
            self.tape.push_back(false);
        }
        self.cursor += 1;
    }
}


fn parse_input(input: &str) -> (StateKey, usize, HashMap<StateKey, State>) {
    let mut paragraphs = input.paragraphs();
    let (starting_state, checksum_step) = extract_starting_state(&paragraphs.next().unwrap());

    let re = Regex::new(r#"In state (?<state_key>\w):
  If the current value is 0:
    - Write the value (?<value_0_writes>\d)\.
    - Move one slot to the (?<value_0_moves>left|right)\.
    - Continue with state (?<value_0_next>\w)\.
  If the current value is 1:
    - Write the value (?<value_1_writes>\d)\.
    - Move one slot to the (?<value_1_moves>left|right)\.
    - Continue with state (?<value_1_next>\w)\."#).unwrap();

    (starting_state, checksum_step, paragraphs.map(|p| {
        let p = p.join("\n");
        let capture = re.captures(&p).unwrap();
        (capture["state_key"].chars().next().unwrap(), State {
            if_false: Instruction {
                write: match &capture["value_0_writes"] {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid write, can only be 1 or 0"),
                },
                direction: Direction::from_str(&capture["value_0_moves"]).unwrap(),
                next_state: capture["value_0_next"].chars().next().unwrap(),
            },
            if_true: Instruction {
                write: match &capture["value_1_writes"] {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid write, can only be 1 or 0"),
                },
                direction: Direction::from_str(&capture["value_1_moves"]).unwrap(),
                next_state: capture["value_1_next"].chars().next().unwrap(),
            },
        })
    }).collect())
}

fn extract_starting_state(paragraph: &[&str]) -> (StateKey, usize) {
    let re_start = Regex::new(r#"^Begin in state (?<starting_state>[A-Z])\.$"#).unwrap();
    let capture_start = re_start.captures(paragraph[0]).unwrap();
    
    let re_checksum = Regex::new(r#"^Perform a diagnostic checksum after (?<checksum_steps>\d+) steps.$"#).unwrap();
    let capture_checksum = re_checksum.captures(paragraph[1]).unwrap();

    (
        capture_start["starting_state"].chars().next().unwrap(),
        capture_checksum["checksum_steps"].parse().unwrap(),
    )
}




struct State {
    if_false: Instruction,
    if_true: Instruction,
}

struct Instruction {
    write: bool,
    direction: Direction,
    next_state: StateKey,
}

enum Direction {
    Left, Right
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        match s.to_lowercase().as_str() {
            "left" => Ok(Left),
            "right" => Ok(Right),
            _ => Err(()),
        }
    }
}

type StateKey = char;