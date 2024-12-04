use std::collections::HashMap;

use aoc_2023::get_input;
use itertools::Itertools;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum RatingType {
    CoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl RatingType {
    fn from_char(c: char) -> RatingType {
        match c {
            'x' => RatingType::CoolLooking,
            'm' => RatingType::Musical,
            'a' => RatingType::Aerodynamic,
            's' => RatingType::Shiny,
            x => panic!("Invalid rating type: {x}"),
        }
    }
}

#[derive(Clone)]
enum FunctionResult {
    Jump(String),
    Accepted,
    Rejected,
    Continue,
}

impl FunctionResult {
    fn from_str(input: &str) -> Self {
        match input {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Jump(input.to_owned()),
        }
    }
}

enum FunctionType {
    LessThan,
    GreaterThan,
}

struct Function {
    left_hand_side: RatingType,
    function_type: FunctionType,
    right_hand_side: u32,
    result: FunctionResult,
}

impl Function {
    fn from_str(input: &str) -> Self {
        let (function, result) = input.split(':').collect_tuple().unwrap();
        let result = FunctionResult::from_str(result);
        let (left_hand_side, function_type) = function.chars().take(2).collect_tuple().unwrap();
        let left_hand_side = RatingType::from_char(left_hand_side);
        let function_type = match function_type {
            '>' => FunctionType::GreaterThan,
            '<' => FunctionType::LessThan,
            x => panic!("Invalid operator: {x}"),
        };
        let right_hand_side = function[2..].parse().unwrap();
        Self {
            left_hand_side,
            function_type,
            right_hand_side,
            result,
        }
    }

    fn run(&self, part: &Part) -> FunctionResult {
        match self.function_type {
            FunctionType::LessThan => {
                if part.get(self.left_hand_side) < self.right_hand_side {
                    self.result.clone()
                } else {
                    FunctionResult::Continue
                }
            }
            FunctionType::GreaterThan => {
                if part.get(self.left_hand_side) > self.right_hand_side {
                    self.result.clone()
                } else {
                    FunctionResult::Continue
                }
            }
        }
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(&self, rating_type: RatingType) -> u32 {
        match rating_type {
            RatingType::CoolLooking => self.x,
            RatingType::Musical => self.m,
            RatingType::Aerodynamic => self.a,
            RatingType::Shiny => self.s,
        }
    }

    fn total_rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn update(&mut self, rating_type: RatingType, number: u32) {
        match rating_type {
            RatingType::CoolLooking => self.x = number,
            RatingType::Musical => self.m = number,
            RatingType::Aerodynamic => self.a = number,
            RatingType::Shiny => self.s = number,
        }
    }
}

struct Workflow {
    functions: Vec<Function>,
    finally: FunctionResult,
}

impl Workflow {
    fn run(&self, part: &Part) -> FunctionResult {
        for f in self.functions.iter() {
            let res = f.run(part);
            match res {
                FunctionResult::Continue => continue,
                _ => return res,
            }
        }
        self.finally.clone()
    }
}

fn main() {
    let input = get_input(19);
    let mut lines = input.lines();

    let mut workflows: HashMap<&str, Workflow> = HashMap::new();

    for line in lines.take_while_ref(|l| !l.is_empty()) {
        let (name, flow) = line
            .trim_end_matches('}')
            .split('{')
            .collect_tuple()
            .unwrap();
        let mut functions = vec![];
        let mut flow = flow.split(',').peekable();
        while let Some(s) = flow.next() {
            if flow.peek().is_none() {
                workflows.insert(
                    name,
                    Workflow {
                        functions,
                        finally: FunctionResult::from_str(s),
                    },
                );
                break;
            }
            functions.push(Function::from_str(s));
        }
    }

    lines.next().unwrap();

    let mut part_a = 0;

    for line in lines {
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for rating in line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
        {
            let (rating_type, rating_number) = rating.split('=').collect_tuple().unwrap();
            part.update(
                RatingType::from_char(rating_type.chars().next().unwrap()),
                rating_number.parse::<u32>().unwrap(),
            );
        }
        if matches!(
            run_recursive(&workflows, "in", &part),
            FunctionResult::Accepted
        ) {
            part_a += part.total_rating();
        }
    }

    dbg!(part_a);
}

fn run_recursive(
    workflows: &HashMap<&str, Workflow>,
    next_workflow: &str,
    part: &Part,
) -> FunctionResult {
    match workflows.get(next_workflow).unwrap().run(part) {
        FunctionResult::Jump(x) => run_recursive(workflows, &x, part),
        FunctionResult::Continue => panic!("Returned continue from workflow"),
        x => x,
    }
}
