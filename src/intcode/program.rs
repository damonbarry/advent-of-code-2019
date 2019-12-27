use super::instruction::{self, Opcode};

pub struct Program {
    pub memory: Vec<i64>,
    instruction_pointer: usize,
}

impl Program {
    pub fn new(init: &[i64]) -> Self {
        Program {
            memory: init.to_vec(),
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.run_with_io(|| unimplemented!(), |_| unimplemented!())
    }

    pub fn run_with_io<I: Fn() -> Result<i64, Error>, O: FnMut(i64) -> Result<(), Error>>(
        &mut self,
        input_func: I,
        mut output_func: O,
    ) -> Result<(), Error> {
        if self.memory.is_empty() {
            return Ok(());
        }

        loop {
            let opcode = Opcode::parse(self.memory[self.instruction_pointer])
                .address(self.instruction_pointer)?;

            {
                let advance = match opcode {
                    Opcode::Addition { param1, param2 } => instruction::add(
                        &mut self.memory,
                        self.instruction_pointer,
                        &[param1, param2],
                    )
                    .address(self.instruction_pointer)?,
                    Opcode::Halt => return Ok(()),
                    Opcode::Multiplication { param1, param2 } => instruction::multiply(
                        &mut self.memory,
                        self.instruction_pointer,
                        &[param1, param2],
                    )
                    .address(self.instruction_pointer)?,
                    Opcode::Print { param } => {
                        let mut value: i64 = 0;
                        let advance = instruction::print(
                            &mut self.memory,
                            self.instruction_pointer,
                            param,
                            &mut value,
                        )
                        .address(self.instruction_pointer)?;
                        output_func(value).address(self.instruction_pointer)?;
                        advance
                    }
                    Opcode::Store => instruction::store(
                        &mut self.memory,
                        self.instruction_pointer,
                        input_func().address(self.instruction_pointer)?,
                    )
                    .address(self.instruction_pointer)?,
                };

                self.instruction_pointer += advance;
            }

            assert!(self.instruction_pointer <= self.memory.len());
            if self.instruction_pointer == self.memory.len() {
                return Ok(());
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
#[fail(
    display = "Encountered an error at address {:?} while running the program",
    address
)]
pub struct Error {
    #[cause]
    pub kind: instruction::ErrorKind,
    pub address: Option<usize>,
}

impl Error {
    pub fn new(kind: instruction::ErrorKind, address: usize) -> Self {
        Error {
            kind,
            address: Some(address),
        }
    }
}

impl From<instruction::ErrorKind> for Error {
    fn from(kind: instruction::ErrorKind) -> Self {
        Error {
            kind,
            address: None,
        }
    }
}

pub trait ResultExt<T> {
    fn address(self, address: usize) -> Result<T, Error>;
}

impl<T> ResultExt<T> for Result<T, Error> {
    fn address(self, address: usize) -> Result<T, Error> {
        self.map_err(|e| Error::new(e.kind, address))
    }
}

impl<T> ResultExt<T> for Result<T, instruction::ErrorKind> {
    fn address(self, address: usize) -> Result<T, Error> {
        self.map_err(|e| Error::new(e, address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_initialize_program_with_memory() {
        let memory = [1, 2, 3];
        let program = Program::new(&memory);
        assert_eq!(memory, program.memory[..]);
    }

    #[test]
    fn runs_an_empty_program() {
        let memory = [];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&memory, &program.memory[..]);
    }

    #[test]
    fn fails_to_run_program_with_invalid_opcode() {
        let memory = [1, 5, 6, 7, 5555, 3, 7, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::InvalidOpcode, 4)),
            result
        );
    }

    #[test]
    fn fails_add_when_first_input_position_is_out_of_range() {
        let memory = [1, 5555, 6, 7, 99, 3, 7, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_add_when_second_input_position_is_out_of_range() {
        let memory = [1, 5, 5555, 7, 99, 3, 7, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_add_when_output_position_is_out_of_range() {
        let memory = [1, 5, 6, 5555, 99, 3, 7, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_add_when_there_are_not_enough_parameters() {
        let memory = [1, 5, 6];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            result
        );
    }

    #[test]
    fn adds_when_both_parameters_are_in_position_mode() {
        let memory = [1, 5, 6, 0, 99, 10, 20];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 5, 6, 0, 99, 10, 20], &program.memory[..]);
    }

    #[test]
    fn adds_when_first_parameter_is_in_immediate_mode() {
        let memory = [101, 10, 5, 0, 99, 20];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 10, 5, 0, 99, 20], &program.memory[..]);
    }

    #[test]
    fn adds_when_second_parameter_is_in_immediate_mode() {
        let memory = [1001, 5, 20, 0, 99, 10];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 5, 20, 0, 99, 10], &program.memory[..]);
    }

    #[test]
    fn adds_when_both_parameters_are_in_immediate_mode() {
        let memory = [1101, 10, 20, 0];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 10, 20, 0], &program.memory[..]);
    }

    #[test]
    fn adds_negative_parameter_in_immediate_mode() {
        let memory = [1101, 100, -1, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[99, 100, -1, 0], &program.memory[..]);
    }

    #[test]
    fn fails_multiply_when_first_input_position_is_out_of_range() {
        let memory = [2, 5555, 6, 7, 99, 3, 7, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_mulitply_when_second_input_position_is_out_of_range() {
        let memory = [2, 5, 5555, 7, 99, 3, 7, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_multiply_when_output_position_is_out_of_range() {
        let memory = [2, 5, 6, 5555, 99, 3, 7, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_multiply_when_there_are_not_enough_parameters() {
        let memory = [2, 5, 6];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            result
        );
    }

    #[test]
    fn multiplies_when_both_parameters_are_in_position_mode() {
        let memory = [2, 5, 6, 0, 99, 10, 20];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 5, 6, 0, 99, 10, 20], &program.memory[..]);
    }

    #[test]
    fn multiplies_when_first_parameter_is_in_immediate_mode() {
        let memory = [102, 10, 5, 0, 99, 20];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 10, 5, 0, 99, 20], &program.memory[..]);
    }

    #[test]
    fn multiplies_when_second_parameter_is_in_immediate_mode() {
        let memory = [1002, 5, 20, 0, 99, 10];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 5, 20, 0, 99, 10], &program.memory[..]);
    }

    #[test]
    fn multiplies_when_both_parameters_are_in_immediate_mode() {
        let memory = [1102, 10, 20, 0];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 10, 20, 0], &program.memory[..]);
    }

    #[test]
    fn fails_store_when_output_position_is_out_of_range() {
        let memory = [3, 5555];
        let mut program = Program::new(&memory);
        let result = program.run_with_io(|| Ok(0), |_| unreachable!());
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_store_when_there_are_not_enough_parameters() {
        let memory = [3];
        let mut program = Program::new(&memory);
        let result = program.run_with_io(|| Ok(0), |_| unreachable!());
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            result
        );
    }

    #[test]
    fn understands_store() {
        let memory = [3, 3, 99, 0];
        let mut program = Program::new(&memory);
        assert!(program.run_with_io(|| Ok(77), |_| unreachable!()).is_ok());
        assert_eq!(&[3, 3, 99, 77], &program.memory[..]);
    }

    #[test]
    fn understands_halt() {
        let memory = [99, 1101, 10, 20, 0];
        let mut program = Program::new(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[99, 1101, 10, 20, 0], &program.memory[..]);
    }

    #[test]
    fn fails_print_when_input_position_is_out_of_range() {
        let memory = [4, 5555];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            result
        );
    }

    #[test]
    fn fails_print_when_there_are_not_enough_parameters() {
        let memory = [4];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            result
        );
    }

    #[test]
    fn prints_when_parameter_is_in_position_mode() {
        let memory = [4, 3, 99, 77];
        let mut program = Program::new(&memory);
        let out = |i| {
            assert_eq!(77, i);
            Ok(())
        };

        assert!(program.run_with_io(|| unreachable!(), out).is_ok());
        assert_eq!(&[4, 3, 99, 77], &program.memory[..]);
    }

    #[test]
    fn prints_when_parameter_is_in_immediate_mode() {
        let memory = [104, 77];
        let mut program = Program::new(&memory);
        let out = |i| {
            assert_eq!(77, i);
            Ok(())
        };

        assert!(program.run_with_io(|| unreachable!(), out).is_ok());
        assert_eq!(&[104, 77], &program.memory[..]);
    }
}
