use std::collections::HashMap;

pub type RegisterName = char;
pub type Registers = HashMap<RegisterName, isize>;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Send(Operand),
    Set(RegisterName, Operand),
    AddAssign(RegisterName, Operand),
    SubtractAssign(RegisterName, Operand),
    MultiplyAssign(RegisterName, Operand),
    ModuloAssign(RegisterName, Operand),
    Recover(Operand),
    Receive(RegisterName),
    JumpIfGreaterThanZero(Operand, Operand),
    JumpIfNotZero(Operand, Operand),
}

impl Instruction {
    pub fn try_as_receive(&self) -> Result<Self, ()> {
        match self {
            Instruction::Recover(x) => match x {
                Operand::RegisterName(x) => Ok(Instruction::Receive(*x)),
                Operand::Value(_) => Err(()),
            },
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    RegisterName(RegisterName),
    Value(isize),
}

impl Operand {
    pub fn eval(&self, registers: &Registers) -> isize {
        match self {
            Operand::RegisterName(x) => registers.get(x).copied().unwrap_or(0),
            Operand::Value(x) => *x,
        }
    }
}

pub mod parsing {
    use std::str::FromStr;

    use itertools::Itertools;

    use super::{Instruction, Operand};

    #[derive(Debug)]
    pub struct InvalidRegisterName;

    impl FromStr for Operand {
        type Err = InvalidRegisterName;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Ok(x) = s.parse::<isize>() {
                return Ok(Operand::Value(x));
            }
            if s.len() != 1 {
                return Err(InvalidRegisterName);
            }
            Ok(Operand::RegisterName(parse_register_name(s)?))
        }
    }

    fn parse_register_name(input: &str) -> Result<char, InvalidRegisterName> {
        input.chars().next().ok_or(InvalidRegisterName)
    }

    #[derive(Debug)]
    pub enum InstructionParseError {
        InvalidInstructionType,
        OperandParseError(InvalidRegisterName),
    }

    impl From<InvalidRegisterName> for InstructionParseError {
        fn from(value: InvalidRegisterName) -> Self {
            InstructionParseError::OperandParseError(value)
        }
    }

    impl FromStr for Instruction {
        type Err = InstructionParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let words = s.split_whitespace().collect_vec();
            let instruction = words[0];

            assert!(words.len() >= 2);

            Ok(match instruction {
                "snd" => Instruction::Send(Operand::from_str(words[1])?),
                "set" => {
                    Instruction::Set(parse_register_name(words[1])?, Operand::from_str(words[2])?)
                }
                "add" => Instruction::AddAssign(
                    parse_register_name(words[1])?,
                    Operand::from_str(words[2])?,
                ),
                "mul" => Instruction::MultiplyAssign(
                    parse_register_name(words[1])?,
                    Operand::from_str(words[2])?,
                ),
                "mod" => Instruction::ModuloAssign(
                    parse_register_name(words[1])?,
                    Operand::from_str(words[2])?,
                ),
                "rcv" => Instruction::Recover(Operand::from_str(words[1])?),
                "jgz" => Instruction::JumpIfGreaterThanZero(
                    Operand::from_str(words[1])?,
                    Operand::from_str(words[2])?,
                ),
                "jnz" => Instruction::JumpIfNotZero(
                    Operand::from_str(words[1])?,
                    Operand::from_str(words[2])?,
                ),
                "sub" => Instruction::SubtractAssign(
                    parse_register_name(words[1])?,
                    Operand::from_str(words[2])?,
                ),
                _ => return Err(InstructionParseError::InvalidInstructionType),
            })
        }
    }

    pub fn parse_instructions(input: &str) -> Vec<Result<Instruction, InstructionParseError>> {
        input.lines().map(Instruction::from_str).collect_vec()
    }
}
