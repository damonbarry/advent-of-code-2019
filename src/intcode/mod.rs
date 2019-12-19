use std::io::{self, Read, Write};

enum Operation {
    Addition { in1: usize, in2: usize, out: usize },
    Halt,
    Mulitplication { in1: usize, in2: usize, out: usize },
    Print { address: usize },
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

    pub fn run_with_io<I: Fn() -> Result<i64, Error>, O: Fn(i64) -> Result<(), Error>>(
        &mut self,
        input_func: I,
        output_func: O,
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
                    self.state[out] = self.state[in1] + self.state[in2];
                }
                Ok(Operation::Mulitplication { in1, in2, out }) => {
                    self.state[out] = self.state[in1] * self.state[in2];
                }
                Ok(Operation::Store { value, address }) => {
                    self.state[address] = value;
                }
                Ok(Operation::Print { address }) => {
                    return output_func(self.state[address]);
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
        match self.state[*pos] {
            n @ 1 | n @ 2 => {
                *pos += 4;
                if self.range().contains(&(*pos - 1)) {
                    self.check_range(*pos - 3)?;
                    self.check_range(*pos - 2)?;
                    self.check_range(*pos - 1)?;

                    let in1 = self.state[*pos - 3] as usize;
                    let in2 = self.state[*pos - 2] as usize;
                    let out = self.state[*pos - 1] as usize;

                    Ok(match n {
                        1 => Operation::Addition { in1, in2, out },
                        2 => Operation::Mulitplication { in1, in2, out },
                        _ => unreachable!(),
                    })
                } else {
                    Err(Error {
                        kind: ErrorKind::NotEnoughArguments,
                        position: self.state.len() - 1,
                    })
                }
            }
            n @ 3 | n @ 4 => {
                *pos += 2;
                if self.range().contains(&(*pos - 1)) {
                    self.check_range(*pos - 1)?;

                    Ok(match n {
                        3 => {
                            let value = input_func()?;
                            Operation::Store {
                                value,
                                address: self.state[*pos - 1] as usize,
                            }
                        }
                        4 => Operation::Print {
                            address: self.state[*pos - 1] as usize,
                        },
                        _ => unreachable!(),
                    })
                } else {
                    Err(Error {
                        kind: ErrorKind::NotEnoughArguments,
                        position: self.state.len() - 1,
                    })
                }
            }
            99 => {
                *pos = 0;
                Ok(Operation::Halt)
            }
            _ => Err(Error {
                kind: ErrorKind::InvalidOpcode,
                position: *pos,
            }),
        }
    }

    fn check_range(&self, p: usize) -> Result<(), Error> {
        if self.range().contains(&(self.state[p] as usize)) {
            Ok(())
        } else {
            Err(Error {
                kind: ErrorKind::PositionOutOfRange,
                position: p,
            })
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
    fn understands_add() {
        let instructions = [1, 5, 6, 7, 99, 3, 7, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1, 5, 6, 7, 99, 3, 7, 10], &program.state[..]);
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
    fn understands_multiply() {
        let instructions = [2, 5, 6, 7, 99, 3, 7, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 5, 6, 7, 99, 3, 7, 21], &program.state[..]);
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
    fn understands_print() {
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
}
