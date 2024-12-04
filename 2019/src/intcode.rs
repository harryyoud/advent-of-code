use types::*;

mod types {
    #[derive(Copy, Clone, Debug)]
    pub enum InstructionType {
        Add,
        Multiply,
        Input,
        Output,
        JumpIfTrue,
        JumpIfFalse,
        LessThan,
        Equals,
        AdjustBase,
        Stop,
    }

    impl InstructionType {
        pub fn parse(number: i64) -> InstructionType {
            use InstructionType::*;
            match number % 100 {
                1 => Add,
                2 => Multiply,
                3 => Input,
                4 => Output,
                5 => JumpIfTrue,
                6 => JumpIfFalse,
                7 => LessThan,
                8 => Equals,
                9 => AdjustBase,
                99 => Stop,
                n => panic!("Invalid instruction type: {n}"),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum OperandType {
        Position,
        Immediate,
        Relative,
    }

    #[derive(Debug)]
    pub struct Instruction {
        pub modes: Vec<OperandType>,
        pub r#type: InstructionType,
    }
}

fn extract_operand_type(number: i64, position: usize) -> OperandType {
    match (number as usize / 10_usize.pow((position + 2) as u32)) % 10 {
        0 => OperandType::Position,
        1 => OperandType::Immediate,
        2 => OperandType::Relative,
        n => panic!("Invalid OperandType: {n}"),
    }
}

fn extract_operand_types(mut number: i64) -> Vec<OperandType> {
    number /= 100;
    let mut types = vec![];

    while number > 0 {
        types.push(match number % 10 {
            0 => OperandType::Position,
            1 => OperandType::Immediate,
            2 => OperandType::Relative,
            n => panic!("Invalid OperandType: {n}"),
        });
        number /= 10;
    }

    types
}

pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|num| num.parse().expect("Expected valid integer"))
        .collect()
}

#[derive(Copy, Clone, Debug)]
pub enum MachineState {
    NotStarted,
    Running,
    NeedMoreInput,
    HitStopInstruction,
}

#[derive(Clone)]
pub struct Machine {
    pub program: Vec<i64>,
    program_cursor: usize,
    pub inputs: Vec<i64>,
    input_cursor: usize,
    pub state: MachineState,
    pub outputs: Vec<i64>,
    pub debug: bool,
    output_debug: Vec<(usize, InstructionType, Vec<OperandType>, i64, i64)>,
    relative_base: i64,
}

impl Machine {
    pub fn new(instructions: Vec<i64>) -> Self {
        Self {
            program: instructions,
            program_cursor: 0,
            inputs: vec![],
            input_cursor: 0,
            state: MachineState::NotStarted,
            outputs: vec![],
            debug: false,
            output_debug: vec![],
            relative_base: 0,
        }
    }

    fn read_instruction(&self) -> Instruction {
        Instruction {
            r#type: InstructionType::parse(self.program[self.program_cursor]),
            modes: extract_operand_types(self.program[self.program_cursor]),
        }
    }

    fn get_value(&self, parameter_index: usize) -> i64 {
        let arg_offset = self.program_cursor + parameter_index + 1;
        match extract_operand_type(self.program[self.program_cursor], parameter_index) {
            OperandType::Position => *self
                .program
                .get(self.program[arg_offset] as usize)
                .unwrap_or(&0),
            OperandType::Immediate => self.program[arg_offset],
            OperandType::Relative => *self
                .program
                .get((self.program[arg_offset] + (self.relative_base)) as usize)
                .unwrap_or(&0),
        }
    }

    fn set_value(&mut self, parameter_index: usize, value: i64) {
        let arg_offset = self.program_cursor + parameter_index + 1;
        match extract_operand_type(self.program[self.program_cursor], parameter_index) {
            OperandType::Position => {
                let offset = self.program[arg_offset];
                if self.program.len() <= offset as usize {
                    self.program.resize(offset as usize + 1, 0);
                }
                self.program[offset as usize] = value;
            }
            OperandType::Immediate => panic!("Can not set immediate mode value"),
            OperandType::Relative => {
                let offset = self.program[arg_offset];
                let offset = (offset + self.relative_base) as usize;
                if self.program.len() <= offset {
                    self.program.resize(offset + 1, 0);
                }
                self.program[offset] = value;
            }
        }
    }

    pub fn run(&mut self) -> MachineState {
        use MachineState::*;
        loop {
            match self.step() {
                NotStarted => continue,
                Running => continue,
                NeedMoreInput => break,
                HitStopInstruction => {
                    if self.debug && self.outputs.len() > 1 {
                        println!("Checks failed:");
                        for (
                            cursor,
                            instruction_type,
                            operand_type,
                            instruction_argument,
                            resolved_output,
                        ) in self.output_debug[..self.output_debug.len() - 1].iter()
                        {
                            if *resolved_output == 0 {
                                continue;
                            }
                            println!("----------------------------------------");
                            println!("offset of current instruction: {cursor}");
                            println!("instruction type: {instruction_type:?}");
                            println!("instruction argument: {instruction_argument} (in {operand_type:?} mode)");
                            println!("output = {resolved_output}");
                            println!("----------------------------------------");
                        }
                    }
                    break;
                }
            }
        }
        self.state
    }

    fn step(&mut self) -> MachineState {
        self.state = MachineState::Running;

        let instruction = self.read_instruction();
        match instruction.r#type {
            InstructionType::Add => {
                let (left, right) = (self.get_value(0), self.get_value(1));
                self.set_value(2, left + right);
                self.program_cursor += 4;
            }
            InstructionType::Multiply => {
                let (left, right) = (self.get_value(0), self.get_value(1));
                self.set_value(2, left * right);
                self.program_cursor += 4;
            }
            InstructionType::JumpIfTrue => {
                if self.get_value(0) != 0 {
                    self.program_cursor = self.get_value(1) as usize;
                } else {
                    self.program_cursor += 3;
                }
            }
            InstructionType::JumpIfFalse => {
                if self.get_value(0) == 0 {
                    self.program_cursor = self.get_value(1) as usize;
                } else {
                    self.program_cursor += 3;
                }
            }
            InstructionType::LessThan => {
                self.set_value(
                    2,
                    if self.get_value(0) < self.get_value(1) {
                        1
                    } else {
                        0
                    },
                );
                self.program_cursor += 4;
            }
            InstructionType::Equals => {
                self.set_value(
                    2,
                    if self.get_value(0) == self.get_value(1) {
                        1
                    } else {
                        0
                    },
                );
                self.program_cursor += 4;
            }
            InstructionType::Input => {
                if self.inputs.get(self.input_cursor).is_none() {
                    self.state = MachineState::NeedMoreInput;
                    return self.state;
                }
                self.set_value(0, self.inputs[self.input_cursor]);
                self.program_cursor += 2;
                self.input_cursor += 1;
            }
            InstructionType::Output => {
                let output = self.get_value(0);
                self.output_debug.push((
                    self.program_cursor,
                    instruction.r#type,
                    instruction.modes,
                    self.program[self.program_cursor],
                    output,
                ));
                self.outputs.push(output);
                self.program_cursor += 2;
            }
            InstructionType::AdjustBase => {
                self.relative_base += self.get_value(0);
                self.program_cursor += 2;
            }
            InstructionType::Stop => {
                self.state = MachineState::HitStopInstruction;
                // don't advance cursor to make sure we continually hit stop to avoid overrunning
            }
        }
        self.state
    }
}
