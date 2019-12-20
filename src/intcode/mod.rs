use std::io::{self, Read, Write};

enum ParameterMode {
    Position,
    Immediate,
}

enum Opcode {
    Addition {
        param1: ParameterMode,
        param2: ParameterMode,
    },
    Halt,
    Mulitplication {
        param1: ParameterMode,
        param2: ParameterMode,
    },
    Print {
        param: ParameterMode,
    },
    Store,
}

enum Operation {
    Addition { in1: i64, in2: i64, out: usize },
    Halt,
    Mulitplication { in1: i64, in2: i64, out: usize },
    Print { value: i64 },
    Store { value: i64, address: usize },
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: usize,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Input,
    InvalidOpcode,
    InvalidParameterMode,
    NotEnoughArguments,
    Output,
    PositionOutOfRange,
}

pub struct Program {
    pub state: Vec<i64>,
}

impl Program {
    pub fn new(init: Vec<i64>) -> Program {
        Program { state: init }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.run_with_io(Program::get_stdin, Program::print_stdout)
    }

    pub fn run_with_io<I: Fn() -> Result<i64, Error>, O: FnMut(i64) -> Result<(), Error>>(
        &mut self,
        input_func: I,
        mut output_func: O,
    ) -> Result<(), Error> {
        if self.state.is_empty() {
            return Ok(());
        }

        let mut pos: usize = 0;

        loop {
            match self.next_op(&mut pos, &input_func) {
                Ok(Operation::Halt) => {
                    return Ok(());
                }
                Ok(Operation::Addition { in1, in2, out }) => {
                    self.state[out] = in1 + in2;
                }
                Ok(Operation::Mulitplication { in1, in2, out }) => {
                    self.state[out] = in1 * in2;
                }
                Ok(Operation::Store { value, address }) => {
                    self.state[address] = value;
                }
                Ok(Operation::Print { value }) => {
                    output_func(value)?;
                }
                Err(err) => return Err(err),
            };
        }
    }

    fn next_op<F: Fn() -> Result<i64, Error>>(
        &mut self,
        pos: &mut usize,
        input_func: &F,
    ) -> Result<Operation, Error> {
        let opcode = Program::parse_opcode(self.state[*pos], *pos)?;
        match opcode {
            Opcode::Addition { .. } => {
                let (in1, in2, out) = self.get_three_args(pos)?;
                let (in1, in2) = self
                    .resolve_input_args(opcode, in1, in2)
                    .map_err(|p| Error {
                        kind: ErrorKind::PositionOutOfRange,
                        position: *pos - 4 + p as usize,
                    })?;
                Ok(Operation::Addition { in1, in2, out })
            }
            Opcode::Mulitplication { .. } => {
                let (in1, in2, out) = self.get_three_args(pos)?;
                let (in1, in2) = self
                    .resolve_input_args(opcode, in1, in2)
                    .map_err(|p| Error {
                        kind: ErrorKind::PositionOutOfRange,
                        position: *pos - 4 + p as usize,
                    })?;
                Ok(Operation::Mulitplication { in1, in2, out })
            }
            Opcode::Store => {
                let address = self.get_one_arg(pos)?;
                self.check_range(address).map_err(|_| Error {
                    kind: ErrorKind::PositionOutOfRange,
                    position: *pos - 1,
                })?;
                let value = input_func()?;
                Ok(Operation::Store { value, address })
            }
            Opcode::Print { .. } => {
                let address = self.get_one_arg(pos)?;
                let value = self.resolve_input_arg(opcode, address).map_err(|_| Error {
                    kind: ErrorKind::PositionOutOfRange,
                    position: *pos - 1,
                })?;
                Ok(Operation::Print { value })
            }
            Opcode::Halt => {
                *pos = 0;
                Ok(Operation::Halt)
            }
        }
    }

    fn get_three_args(&mut self, pos: &mut usize) -> Result<(usize, usize, usize), Error> {
        *pos += 4;
        self.check_range(*pos - 1).map_err(|_| Error {
            kind: ErrorKind::NotEnoughArguments,
            position: self.state.len() - 1,
        })?;

        // Check that the value addressed by the 3rd (output) parameter is, itself, a valid address.
        self.check_range(self.state[*pos - 1] as usize)
            .map_err(|_| Error {
                kind: ErrorKind::PositionOutOfRange,
                position: *pos - 1,
            })?;

        let in1 = self.state[*pos - 3] as usize;
        let in2 = self.state[*pos - 2] as usize;
        let out = self.state[*pos - 1] as usize;

        Ok((in1, in2, out))
    }

    fn resolve_input_args(
        &mut self,
        opcode: Opcode,
        in1: usize,
        in2: usize,
    ) -> Result<(i64, i64), u8> {
        match opcode {
            Opcode::Addition { param1, param2 } | Opcode::Mulitplication { param1, param2 } => {
                let i1 = match param1 {
                    ParameterMode::Position => {
                        self.check_range(in1).map_err(|_| 1)?;
                        self.state[in1]
                    }
                    ParameterMode::Immediate => in1 as i64,
                };
                let i2 = match param2 {
                    ParameterMode::Position => {
                        self.check_range(in2).map_err(|_| 2)?;
                        self.state[in2]
                    }
                    ParameterMode::Immediate => in2 as i64,
                };

                Ok((i1, i2))
            }
            _ => panic!("Only call resolve_input_args for Opcodes with two parameters"),
        }
    }

    fn get_one_arg(&mut self, pos: &mut usize) -> Result<usize, Error> {
        *pos += 2;
        self.check_range(*pos - 1).map_err(|_| Error {
            kind: ErrorKind::NotEnoughArguments,
            position: self.state.len() - 1,
        })?;
        Ok(self.state[*pos - 1] as usize)
    }

    fn resolve_input_arg(&mut self, opcode: Opcode, in1: usize) -> Result<i64, ()> {
        match opcode {
            Opcode::Print { param } => match param {
                ParameterMode::Position => {
                    self.check_range(in1)?;
                    Ok(self.state[in1])
                }
                ParameterMode::Immediate => Ok(in1 as i64),
            },
            _ => panic!("Only call resolve_input_arg for Opcodes with two parameters"),
        }
    }

    fn parse_opcode(opcode: i64, position: usize) -> Result<Opcode, Error> {
        match opcode % 100 {
            1 => Ok(Opcode::Addition {
                param1: Program::parse_parameter_mode(opcode, 1, position)?,
                param2: Program::parse_parameter_mode(opcode, 2, position)?,
            }),
            2 => Ok(Opcode::Mulitplication {
                param1: Program::parse_parameter_mode(opcode, 1, position)?,
                param2: Program::parse_parameter_mode(opcode, 2, position)?,
            }),
            3 => Ok(Opcode::Store),
            4 => Ok(Opcode::Print {
                param: Program::parse_parameter_mode(opcode, 1, position)?,
            }),
            99 => Ok(Opcode::Halt),
            _ => Err(Error {
                kind: ErrorKind::InvalidOpcode,
                position,
            }),
        }
    }

    fn parse_parameter_mode(
        value: i64,
        which: u32,
        position: usize,
    ) -> Result<ParameterMode, Error> {
        let place = 10_i64.checked_pow(which + 1).unwrap();
        match value / place {
            0 | 10 => Ok(ParameterMode::Position),
            1 | 11 => Ok(ParameterMode::Immediate),
            _ => Err(Error {
                kind: ErrorKind::InvalidParameterMode,
                position,
            }),
        }
    }

    fn check_range(&self, address: usize) -> Result<(), ()> {
        if self.range().contains(&address) {
            Ok(())
        } else {
            Err(())
        }
    }

    fn range(&self) -> std::ops::Range<usize> {
        0..self.state.len()
    }

    pub fn get_stdin() -> Result<i64, Error> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).map_err(|_| Error {
            kind: ErrorKind::Input,
            position: 0,
        })?;
        Ok(buffer.parse::<i64>().unwrap())
    }

    pub fn print_stdout(value: i64) -> Result<(), Error> {
        let buf = value.to_string();
        io::stdout().write_all(buf.as_bytes()).map_err(|_| Error {
            kind: ErrorKind::Output,
            position: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_an_empty_program() {
        let instructions = [];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&instructions[..], &program.state[..]);
    }

    #[test]
    fn fails_to_run_program_with_invalid_opcode() {
        let instructions = [1, 5, 6, 7, 5555, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::InvalidOpcode, err.kind);
        assert_eq!(4, err.position);
    }

    #[test]
    fn understands_halt() {
        let instructions = [99];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&instructions[..], &program.state[..]);
    }

    #[test]
    fn fails_add_when_first_input_position_is_out_of_range() {
        let instructions = [1, 5555, 6, 7, 99, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(1, err.position);
    }

    #[test]
    fn fails_add_when_second_input_position_is_out_of_range() {
        let instructions = [1, 5, 5555, 7, 99, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(2, err.position);
    }

    #[test]
    fn fails_add_when_output_position_is_out_of_range() {
        let instructions = [1, 5, 6, 5555, 99, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(3, err.position);
    }

    #[test]
    fn fails_add_when_there_are_not_enough_arguments() {
        let instructions = [1, 5, 6];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::NotEnoughArguments, err.kind);
        assert_eq!(2, err.position);
    }

    #[test]
    fn adds_when_parameters_are_in_position_mode() {
        let instructions = [1, 5, 6, 7, 99, 10, 20, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1, 5, 6, 7, 99, 10, 20, 30], &program.state[..]);
    }

    #[test]
    fn adds_when_first_parameter_is_in_immediate_mode() {
        let instructions = [101, 10, 5, 6, 99, 20, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[101, 10, 5, 6, 99, 20, 30], &program.state[..]);
    }

    #[test]
    fn adds_when_second_parameter_is_in_immediate_mode() {
        let instructions = [1001, 5, 20, 6, 99, 10, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1001, 5, 20, 6, 99, 10, 30], &program.state[..]);
    }

    #[test]
    fn adds_when_both_parameters_are_in_immediate_mode() {
        let instructions = [1101, 10, 20, 5, 99, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1101, 10, 20, 5, 99, 30], &program.state[..]);
    }

    #[test]
    fn adds_negative_parameter_in_immediate_mode() {
        let instructions = [1101,100,-1,4,0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1101,100,-1,4,99], &program.state[..]);
    }

    #[test]
    fn fails_multiply_when_first_input_position_is_out_of_range() {
        let instructions = [2, 5555, 6, 7, 99, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(1, err.position);
    }

    #[test]
    fn fails_mulitply_when_second_input_position_is_out_of_range() {
        let instructions = [2, 5, 5555, 7, 99, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(2, err.position);
    }

    #[test]
    fn fails_multiply_when_output_position_is_out_of_range() {
        let instructions = [2, 5, 6, 5555, 99, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(3, err.position);
    }

    #[test]
    fn fails_multiply_when_there_are_not_enough_arguments() {
        let instructions = [2, 5, 6];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::NotEnoughArguments, err.kind);
        assert_eq!(2, err.position);
    }

    #[test]
    fn multiplies_when_parameters_are_in_position_mode() {
        let instructions = [2, 5, 6, 7, 99, 10, 20, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 5, 6, 7, 99, 10, 20, 200], &program.state[..]);
    }

    #[test]
    fn multiplies_when_first_parameter_is_in_immediate_mode() {
        let instructions = [102, 10, 5, 6, 99, 20, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[102, 10, 5, 6, 99, 20, 200], &program.state[..]);
    }

    #[test]
    fn multiplies_when_second_parameter_is_in_immediate_mode() {
        let instructions = [1002, 5, 20, 6, 99, 10, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1002, 5, 20, 6, 99, 10, 200], &program.state[..]);
    }

    #[test]
    fn multiplies_when_both_parameters_are_in_immediate_mode() {
        let instructions = [1102, 10, 20, 5, 99, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1102, 10, 20, 5, 99, 200], &program.state[..]);
    }

    #[test]
    fn fails_store_when_output_position_is_out_of_range() {
        let instructions = [3, 5555, 99];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(1, err.position);
    }

    #[test]
    fn fails_store_when_there_are_not_enough_arguments() {
        let instructions = [3];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::NotEnoughArguments, err.kind);
        assert_eq!(0, err.position);
    }

    #[test]
    fn understands_store() {
        let instructions = [3, 3, 99, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run_with_io(|| Ok(77), |_| unreachable!());
        assert!(result.is_ok());
        assert_eq!(&[3, 3, 99, 77], &program.state[..]);
    }

    #[test]
    fn fails_print_when_input_position_is_out_of_range() {
        let instructions = [4, 5555, 99];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::PositionOutOfRange, err.kind);
        assert_eq!(1, err.position);
    }

    #[test]
    fn fails_print_when_there_are_not_enough_arguments() {
        let instructions = [4];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(ErrorKind::NotEnoughArguments, err.kind);
        assert_eq!(0, err.position);
    }

    #[test]
    fn prints_when_parameter_is_in_position_mode() {
        let instructions = [4, 3, 99, 77];
        let mut program = Program::new(instructions.to_vec());
        let input = || unreachable!();
        let output = |i| {
            assert_eq!(77, i);
            Ok(())
        };

        let result = program.run_with_io(input, output);
        assert!(result.is_ok());
        assert_eq!(&[4, 3, 99, 77], &program.state[..]);
    }

    #[test]
    fn prints_when_parameter_is_in_immediate_mode() {
        let instructions = [104, 77, 99];
        let mut program = Program::new(instructions.to_vec());
        let input = || unreachable!();
        let output = |i| {
            assert_eq!(77, i);
            Ok(())
        };

        let result = program.run_with_io(input, output);
        assert!(result.is_ok());
        assert_eq!(&[104, 77, 99], &program.state[..]);
    }
}
