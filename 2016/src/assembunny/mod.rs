use std::{collections::HashMap, marker::PhantomData};

use itertools::Itertools;


mod private {
    pub trait Sealed {}
}

pub struct PreExecution;
pub struct Executed;
pub trait MachineState: private::Sealed {}
impl private::Sealed for PreExecution {}
impl MachineState for PreExecution {}
impl private::Sealed for Executed {}
impl MachineState for Executed {}

pub enum MachineResult {
    OutOfInstructions,
    PushedOut,
    Ok,
}

pub struct AssembunnyMachine<T: MachineState> {
    cursor: usize,
    instructions: Vec<Instruction>,
    registers: HashMap<RegisterName, u32>,
    out: Vec<i32>,
    _state: PhantomData<T>,
}

impl AssembunnyMachine<PreExecution> {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        AssembunnyMachine {
            instructions,
            cursor: 0,
            registers: HashMap::new(),
            out: vec![],
            _state: PhantomData,
        }
    }

    pub fn set_register(&mut self, name: RegisterName, value: u32) {
        self.registers.insert(name, value);
    }

    pub fn execute(mut self) -> AssembunnyMachine<Executed> {
        loop {
            match self.step() {
                MachineResult::OutOfInstructions => break,
                MachineResult::Ok => continue,
                MachineResult::PushedOut => continue,
            }
        }
        AssembunnyMachine {
            cursor: self.cursor,
            instructions: self.instructions,
            registers: self.registers,
            out: vec![],
            _state: PhantomData,
        }
    }

    pub fn step(&mut self) -> MachineResult {
        let Some(instruction) = self.instructions.get(self.cursor) else {
            return MachineResult::OutOfInstructions;
        };
        match instruction {
            Instruction::Copy(value, register) => {
                let value = get_value(&self.registers, value);
                self.registers.insert(*register, value as u32);
                self.cursor += 1;
            },
            Instruction::JumpIfNotZero(value, relative_jump) => {
                let value = get_value(&self.registers, value);
                let relative_jump = get_value(&self.registers, relative_jump);
                if value == 0 {
                    self.cursor += 1;
                } else {
                    self.cursor = self.cursor.saturating_add_signed(relative_jump as isize);
                }
            },
            Instruction::Increment(register) => {
                self.registers.entry(*register)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                self.cursor += 1;
            },
            Instruction::Decrement(register) => {
                self.registers.entry(*register)
                    .and_modify(|x| *x -= 1)
                    .or_insert(1);
                self.cursor += 1;
            },
            Instruction::Toggle(value) => {
                let offset = get_value(&self.registers, value);
                if let Some(i) = self.instructions.get_mut(self.cursor.saturating_add_signed(offset as isize)) {
                    *i = i.toggle();
                };
                self.cursor += 1;
            },
            Instruction::Out(value) => {
                self.out.push(get_value(&self.registers, value));
                self.cursor += 1;
                return MachineResult::PushedOut;
            }
            Instruction::Noop => {
                self.cursor += 1;
            },
        }
        MachineResult::Ok
    }

    pub fn pop_out(&mut self) -> Option<i32> {
        self.out.pop()
    }
}

impl AssembunnyMachine<Executed> {
    pub fn get_register(&self, name: RegisterName) -> u32 {
        *self.registers.get(&name).unwrap_or(&0)
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect_vec()
}

pub fn get_value(registers: &HashMap<RegisterName, u32>, value: &Value) -> i32 {
    match value {
        Value::Number(x) => *x,
        Value::Register(x) => *registers.get(x).unwrap_or(&0) as i32,
    }
}

pub fn parse_instruction(line: &str) -> Instruction {
    let mut split = line.split_whitespace();

    match split.next().unwrap() {
        "cpy" => Instruction::Copy(
            parse_value(split.next().unwrap()),
            split.next().unwrap().chars().next().unwrap().into(),
        ),
        "jnz" => Instruction::JumpIfNotZero(
            parse_value(split.next().unwrap()),
            parse_value(split.next().unwrap()),
        ),
        "inc" => Instruction::Increment(
            split.next().unwrap().chars().next().unwrap().into(),
        ),
        "dec" => Instruction::Decrement(
            split.next().unwrap().chars().next().unwrap().into(),
        ),
        "tgl" => Instruction::Toggle(
            parse_value(split.next().unwrap()),
        ),
        "out" => Instruction::Out(
            parse_value(split.next().unwrap()),
        ),
        s => panic!("Invalid instruction: {s} (from line: {line})")
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Copy(Value, RegisterName),
    JumpIfNotZero(Value, Value),
    Increment(RegisterName),
    Decrement(RegisterName),
    Toggle(Value),
    Out(Value),
    Noop,
}

impl Instruction {
    pub fn toggle(&self) -> Self {
        match self {
            Instruction::Copy(a, b) => {
                Instruction::JumpIfNotZero(*a, Value::Register(*b))
            },
            Instruction::JumpIfNotZero(a, b) => {
                match b {
                    Value::Number(_x) => Instruction::Noop,
                    Value::Register(x) => Instruction::Copy(*a, *x),
                }
            },
            Instruction::Increment(x) => {
                Instruction::Decrement(*x)
            },
            Instruction::Decrement(x) => {
                Instruction::Increment(*x)
            },
            Instruction::Toggle(val) => {
                match val {
                    Value::Number(_x) => Instruction::Noop,
                    Value::Register(x) => Instruction::Increment(*x),
                }
            },
            Instruction::Out(_) => self.clone(),
            Instruction::Noop => Instruction::Noop
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Value {
    Number(i32),
    Register(RegisterName),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct RegisterName(char);

impl From<char> for RegisterName {
    fn from(value: char) -> Self {
        Self(value)
    }
}

pub fn parse_value(slice: &str) -> Value {
    if let Ok(x) = slice.parse::<i32>() {
        return Value::Number(x);
    }
    if slice.len() > 1 {
        panic!("Register length can be one character only: {slice}");
    }
    return Value::Register(slice.chars().next().unwrap().into());
}
