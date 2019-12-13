enum Operation {
    Addition { in1: usize, in2: usize, out: usize },
    Halt,
    Mulitplication { in1: usize, in2: usize, out: usize },
}

pub struct Error {
    pub kind: ErrorKind,
    pub position: usize,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidOpcode,
    NotEnoughArguments,
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
        if self.state.is_empty() {
            return Ok(());
        }

        let mut pos: usize = 0;

        loop {
            match self.next_op(&mut pos) {
                Ok(Operation::Halt) => {
                    return Ok(());
                }
                Ok(Operation::Addition { in1, in2, out }) => {
                    self.state[out] = self.state[in1] + self.state[in2];
                }
                Ok(Operation::Mulitplication { in1, in2, out }) => {
                    self.state[out] = self.state[in1] * self.state[in2];
                }
                Err(err) => return Err(err),
            };
        }
    }

    fn next_op(&mut self, pos: &mut usize) -> Result<Operation, Error> {
        match self.state[*pos] {
            n @ 1 | n @ 2 => {
                *pos += 4;
                let range = 0..self.state.len();
                if range.contains(pos) {
                    let check_range = |p| {
                        if range.contains(&(self.state[p] as usize)) {
                            Ok(())
                        } else {
                            Err(Error {
                                kind: ErrorKind::PositionOutOfRange,
                                position: p,
                            })
                        }
                    };

                    check_range(*pos - 3)?;
                    check_range(*pos - 2)?;
                    check_range(*pos - 1)?;

                    let in1 = self.state[*pos - 3] as usize;
                    let in2 = self.state[*pos - 2] as usize;
                    let out = self.state[*pos - 1] as usize;

                    Ok(match n {
                        1 => Operation::Addition { in1, in2, out },
                        2 => Operation::Mulitplication { in1, in2, out },
                        _ => unreachable!()
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
    fn runs_first_example_program() {
        let instructions = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], &program.state[..]);
    }


    #[test]
    fn runs_second_example_program() {
        let instructions = [1, 0, 0, 0, 99];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 0, 0, 0, 99], &program.state[..]);
    }

    #[test]
    fn runs_third_example_program() {
        let instructions = [2, 3, 0, 3, 99];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 3, 0, 6, 99], &program.state[..]);
    }

    #[test]
    fn runs_fourth_example_program() {
        let instructions = [2, 4, 4, 5, 99, 0];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 4, 4, 5, 99, 9801], &program.state[..]);
    }

    #[test]
    fn runs_fifth_example_program() {
        let instructions = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[30, 1, 1, 4, 2, 5, 6, 0, 99], &program.state[..]);
    }

    #[test]
    fn solve_day2_part1() {
        let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
        let mut instructions: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        instructions[1] = 12;
        instructions[2] = 2;

        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(8017076, program.state[0]);
    }
}
