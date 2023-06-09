/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */
use std::cmp::Ordering;
use std::iter;

#[derive(Clone)]
pub struct Intcode {
    code: Vec<isize>,
    index: isize,
}

impl Intcode {
    pub fn from_input(input: &str) -> Self {
        let code = input
            .split(",")
            .map(|element| element.parse::<isize>().unwrap())
            .chain(iter::repeat(0))
            .take(10000)
            .collect::<Vec<isize>>();

        Self { code, index: 0 }
    }

    pub fn set(&mut self, index: usize, value: isize) {
        self.code[index] = value;
    }

    pub fn get(&self, index: isize) -> isize {
        self.code[index as usize]
    }

    pub fn next(&mut self, mode: ParameterMode, relative_base: isize) -> isize {
        let value = self.get(self.index);
        self.index += 1;

        match mode {
            ParameterMode::Position => self.get(value),
            ParameterMode::Immediate => value,
            ParameterMode::Relative => self.get(value + relative_base),
        }
    }

    pub fn next_target(&mut self, mode: ParameterMode, relative_base: isize) -> isize {
        let value = self.get(self.index);
        self.index += 1;

        match mode {
            ParameterMode::Position => value,
            ParameterMode::Relative => value + relative_base,
            ParameterMode::Immediate => panic!(),
        }
    }

    pub fn jump_to(&mut self, target: isize) {
        self.index = target;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Add,
    Multiply,
    Break,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    ChangeRelativeBase,
}

impl Opcode {
    pub fn convert(value: usize) -> Self {
        match value {
            99 => Opcode::Break,
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            9 => Opcode::ChangeRelativeBase,
            _ => panic!("Undefined opcode: '{value}'"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative
}

impl ParameterMode {
    fn convert(value: isize) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode: '{value}'"),
        }
    }
}

#[derive(Debug)]
pub struct ParsedOpcode {
    pub code: Opcode,
    parameter_mode: Vec<ParameterMode>,
    index: usize,
}

impl ParsedOpcode {
    pub fn new(value: isize) -> Self {
        let binding = value.to_string();
        let mut digits = binding.chars().map(|char| char.to_digit(10).unwrap()).rev();

        let opcode = digits.next().unwrap() + digits.next().unwrap_or(0) * 10;
        let code = Opcode::convert(opcode as usize);

        let parameter_mode: Vec<ParameterMode> = digits
            .map(|element| ParameterMode::convert(element as isize))
            .chain(iter::repeat(ParameterMode::Position))
            .take(3)
            .collect();

        Self {
            code,
            parameter_mode,
            index: 0,
        }
    }

    pub fn next_mode(&mut self) -> ParameterMode {
        let mode = self
            .parameter_mode
            .get(self.index)
            .copied()
            .unwrap();

        self.index += 1;
        mode
    }
}

pub struct Input {
    values: Vec<usize>,
    index: usize,
}

impl Input {
    pub fn none() -> Self {
        Self {
            values: vec![],
            index: 0,
        }
    }

    pub fn new(values: Vec<usize>) -> Self {
        Self { values, index: 0 }
    }

    pub fn next(&mut self) -> usize {
        let value = self.values[self.index];
        self.index += 1;
        value
    }
}

pub struct IntcodeComputer {
    pub code: Intcode,
    relative_base: isize,
}

impl IntcodeComputer {
    pub fn new(code: Intcode) -> Self {
        Self { code, relative_base: 0 }
    }

    pub fn simulate_without_input(&mut self) -> Vec<isize> {
        self.simulate(Input::none())
    }

    pub fn simulate(&mut self, input: Input) -> Vec<isize> {
        let mut input = input;
        let mut output = Vec::new();

        loop {
            let value = self.code.next(ParameterMode::Immediate, self.relative_base);
            let mut opcode = ParsedOpcode::new(value);

            match opcode.code {
                Opcode::Break => break,
                Opcode::Add => {
                    let source_1 = self.code.next(opcode.next_mode(), self.relative_base);
                    let source_2 = self.code.next(opcode.next_mode(), self.relative_base);
                    let target = self.code.next_target(opcode.next_mode(), self.relative_base);

                    self.code.set(target as usize, source_1 + source_2);
                }
                Opcode::Multiply => {
                    let source_1 = self.code.next(opcode.next_mode(), self.relative_base);
                    let source_2 = self.code.next(opcode.next_mode(), self.relative_base);
                    let target = self.code.next_target(opcode.next_mode(), self.relative_base);

                    self.code.set(target as usize, source_1 * source_2);
                }
                Opcode::Input => {
                    let target = self.code.next_target(opcode.next_mode(), self.relative_base);
                    let value = input.next();
                    self.code.set(target as usize, value as isize);
                }
                Opcode::Output => {
                    let value = self.code.next(opcode.next_mode(), self.relative_base);
                    output.push(value);
                }
                Opcode::JumpIfTrue => {
                    let value = self.code.next(opcode.next_mode(), self.relative_base);
                    let target = self.code.next(opcode.next_mode(), self.relative_base);

                    if value == 0 {
                        continue;
                    }

                    self.code.jump_to(target);
                }
                Opcode::JumpIfFalse => {
                    let value = self.code.next(opcode.next_mode(), self.relative_base);
                    let target = self.code.next(opcode.next_mode(), self.relative_base);

                    if value != 0 {
                        continue;
                    }

                    self.code.jump_to(target);
                }
                Opcode::LessThan => {
                    let source_1 = self.code.next(opcode.next_mode(), self.relative_base);
                    let source_2 = self.code.next(opcode.next_mode(), self.relative_base);
                    let target = self.code.next_target(opcode.next_mode(), self.relative_base);

                    let result = match source_1.cmp(&source_2) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 0,
                    };

                    self.code.set(target as usize, result);
                }
                Opcode::Equals => {
                    let source_1 = self.code.next(opcode.next_mode(), self.relative_base);
                    let source_2 = self.code.next(opcode.next_mode(), self.relative_base);
                    let target = self.code.next_target(opcode.next_mode(), self.relative_base);

                    let result = match source_1.cmp(&source_2) {
                        Ordering::Less => 0,
                        Ordering::Equal => 1,
                        Ordering::Greater => 0,
                    };

                    self.code.set(target as usize, result);
                },
                Opcode::ChangeRelativeBase => {
                    let amount = self.code.next(opcode.next_mode(), self.relative_base);
                    self.relative_base += amount;
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_add() {
        let intcode = Intcode::from_input("1,0,0,0,99");
        let mut computer = IntcodeComputer::new(intcode);
        computer.simulate_without_input();

        assert_eq!(&computer.code.code[0..5], &[2, 0, 0, 0, 99])
    }

    #[test]
    fn test_day_2_multiply() {
        let intcode = Intcode::from_input("2,3,0,3,99");
        let mut computer = IntcodeComputer::new(intcode);
        computer.simulate_without_input();

        assert_eq!(&computer.code.code[0..5], &[2, 3, 0, 6, 99])
    }

    #[test]
    fn test_day_2_multiply_2() {
        let intcode = Intcode::from_input("2,4,4,5,99,0");
        let mut computer = IntcodeComputer::new(intcode);
        computer.simulate_without_input();

        assert_eq!(&computer.code.code[0..6], &[2, 4, 4, 5, 99, 9801])
    }

    #[test]
    fn test_day_2_large() {
        let intcode = Intcode::from_input("1,1,1,4,99,5,6,0,99");
        let mut computer = IntcodeComputer::new(intcode);
        computer.simulate_without_input();

        assert_eq!(&computer.code.code[0..9], &[30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn test_day_5_negative_number() {
        let intcode = Intcode::from_input("1101,100,-1,4,0");
        let mut computer = IntcodeComputer::new(intcode);
        computer.simulate_without_input();

        assert_eq!(&computer.code.code[0..5], &[1101, 100, -1, 4, 99])
    }

    #[test]
    fn test_day_9_copy() {
        let intcode = Intcode::from_input("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut computer = IntcodeComputer::new(intcode);
        let output = computer.simulate_without_input();

        assert_eq!(output, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99])
    }

    #[test]
    fn test_day_9_large_output() {
        let intcode = Intcode::from_input("1102,34915192,34915192,7,4,7,99,0");
        let mut computer = IntcodeComputer::new(intcode);
        let output = computer.simulate_without_input();

        assert_eq!(output, vec![1219070632396864])
    }

    #[test]
    fn test_day_9_large_output_2() {
        let intcode = Intcode::from_input("104,1125899906842624,99");
        let mut computer = IntcodeComputer::new(intcode);
        let output = computer.simulate_without_input();

        assert_eq!(output, vec![1125899906842624])
    }
}
