use super::instruction::{self, Opcode};

pub struct Program<I, O>
where
    I: Fn() -> i64,
    O: FnMut(i64),
{
    pub memory: Vec<i64>,
    instruction_pointer: usize,
    input_fn: I,
    output_fn: O,
}

impl<I, O> Program<I, O>
where
    I: Fn() -> i64,
    O: FnMut(i64),
{
    pub fn with_io(init: &[i64], input_fn: I, output_fn: O) -> Self {
        Program {
            memory: init.to_vec(),
            instruction_pointer: 0,
            input_fn,
            output_fn,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if self.memory.is_empty() {
            return Ok(());
        }

        loop {
            let opcode = Opcode::parse(self.memory[self.instruction_pointer])
                .address(self.instruction_pointer)?;

            {
                let advance_address = match opcode {
                    Opcode::Addition { param1, param2 } => {
                        instruction::add(self, &[param1, param2])
                            .address(self.instruction_pointer)?
                    }
                    Opcode::Halt => return Ok(()),
                    Opcode::Multiplication { param1, param2 } => {
                        instruction::multiply(self, &[param1, param2])
                            .address(self.instruction_pointer)?
                    }
                    Opcode::Print { param } => {
                        instruction::print(self, param).address(self.instruction_pointer)?
                    }
                    Opcode::Store => instruction::store(self).address(self.instruction_pointer)?,
                };

                self.instruction_pointer = advance_address;
            }

            assert!(self.instruction_pointer <= self.memory.len());
            if self.instruction_pointer == self.memory.len() {
                return Ok(());
            }
        }
    }
}

#[macro_export]
macro_rules! new_program {
    ($mem:expr) => {
        Program::with_io($mem, || unimplemented!(), |_| unimplemented!())
    };
}

pub trait System {
    fn get_memory_len(&self) -> usize;
    fn read_memory(&self, address: usize) -> i64;
    fn write_memory(&mut self, address: usize, value: i64);
    fn read_instruction_pointer(&self) -> usize;
    fn write_instruction_pointer(&mut self, address: usize);
    fn read_input(&self) -> i64;
    fn write_output(&mut self, value: i64);
}

impl<I, O> System for Program<I, O>
where
    I: Fn() -> i64,
    O: FnMut(i64),
{
    fn get_memory_len(&self) -> usize {
        self.memory.len()
    }

    fn read_memory(&self, address: usize) -> i64 {
        self.memory[address]
    }

    fn write_memory(&mut self, address: usize, value: i64) {
        self.memory[address] = value;
    }

    fn read_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    fn write_instruction_pointer(&mut self, address: usize) {
        let _ = self.memory[address]; // panics if address is out of range
        self.instruction_pointer = address;
    }

    fn read_input(&self) -> i64 {
        (self.input_fn)()
    }

    fn write_output(&mut self, value: i64) {
        (self.output_fn)(value);
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
        let program = new_program!(&memory);
        assert_eq!(memory, program.memory[..]);
    }

    #[test]
    fn runs_an_empty_program() {
        let memory = [];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&memory, &program.memory[..]);
    }

    #[test]
    fn fails_to_run_program_with_invalid_opcode() {
        let memory = [1, 5, 6, 7, 5555, 3, 7, 0];
        let mut program = new_program!(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::InvalidOpcode, 4)),
            result
        );
    }

    #[test]
    fn fails_add_when_first_input_position_is_out_of_range() {
        let memory = [1, 5555, 6, 7, 99, 3, 7, 0];
        let mut program = new_program!(&memory);
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
        let mut program = new_program!(&memory);
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
        let mut program = new_program!(&memory);
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
        let mut program = new_program!(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            result
        );
    }

    #[test]
    fn adds_when_both_parameters_are_in_position_mode() {
        let memory = [1, 5, 6, 0, 99, 10, 20];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 5, 6, 0, 99, 10, 20], &program.memory[..]);
    }

    #[test]
    fn adds_when_first_parameter_is_in_immediate_mode() {
        let memory = [101, 10, 5, 0, 99, 20];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 10, 5, 0, 99, 20], &program.memory[..]);
    }

    #[test]
    fn adds_when_second_parameter_is_in_immediate_mode() {
        let memory = [1001, 5, 20, 0, 99, 10];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 5, 20, 0, 99, 10], &program.memory[..]);
    }

    #[test]
    fn adds_when_both_parameters_are_in_immediate_mode() {
        let memory = [1101, 10, 20, 0];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[30, 10, 20, 0], &program.memory[..]);
    }

    #[test]
    fn adds_negative_parameter_in_immediate_mode() {
        let memory = [1101, 100, -1, 0];
        let mut program = new_program!(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[99, 100, -1, 0], &program.memory[..]);
    }

    #[test]
    fn fails_multiply_when_first_input_position_is_out_of_range() {
        let memory = [2, 5555, 6, 7, 99, 3, 7, 0];
        let mut program = new_program!(&memory);
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
        let mut program = new_program!(&memory);
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
        let mut program = new_program!(&memory);
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
        let mut program = new_program!(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            result
        );
    }

    #[test]
    fn multiplies_when_both_parameters_are_in_position_mode() {
        let memory = [2, 5, 6, 0, 99, 10, 20];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 5, 6, 0, 99, 10, 20], &program.memory[..]);
    }

    #[test]
    fn multiplies_when_first_parameter_is_in_immediate_mode() {
        let memory = [102, 10, 5, 0, 99, 20];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 10, 5, 0, 99, 20], &program.memory[..]);
    }

    #[test]
    fn multiplies_when_second_parameter_is_in_immediate_mode() {
        let memory = [1002, 5, 20, 0, 99, 10];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 5, 20, 0, 99, 10], &program.memory[..]);
    }

    #[test]
    fn multiplies_when_both_parameters_are_in_immediate_mode() {
        let memory = [1102, 10, 20, 0];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[200, 10, 20, 0], &program.memory[..]);
    }

    #[test]
    fn fails_store_when_output_position_is_out_of_range() {
        let memory = [3, 5555];
        let mut program = Program::with_io(&memory, || 0, |_| unreachable!());
        assert_eq!(
            Err(Error::new(
                instruction::ErrorKind::AddressOutOfRange(5555),
                0
            )),
            program.run()
        );
    }

    #[test]
    fn fails_store_when_there_are_not_enough_parameters() {
        let memory = [3];
        let mut program = Program::with_io(&memory, || 0, |_| unreachable!());
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            program.run()
        );
    }

    #[test]
    fn understands_store() {
        let memory = [3, 3, 99, 0];
        let mut program = Program::with_io(&memory, || 77, |_| unreachable!());
        assert!(program.run().is_ok());
        assert_eq!(&[3, 3, 99, 77], &program.memory[..]);
    }

    #[test]
    fn understands_halt() {
        let memory = [99, 1101, 10, 20, 0];
        let mut program = new_program!(&memory);
        assert!(program.run().is_ok());
        assert_eq!(&[99, 1101, 10, 20, 0], &program.memory[..]);
    }

    #[test]
    fn fails_print_when_input_position_is_out_of_range() {
        let memory = [4, 5555];
        let mut program = new_program!(&memory);
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
        let mut program = new_program!(&memory);
        let result = program.run();
        assert_eq!(
            Err(Error::new(instruction::ErrorKind::NotEnoughParameters, 0)),
            result
        );
    }

    #[test]
    fn prints_when_parameter_is_in_position_mode() {
        let memory = [4, 3, 99, 77];
        let mut program = Program::with_io(&memory, || unreachable!(), |i| assert_eq!(77, i));
        assert!(program.run().is_ok());
        assert_eq!(&[4, 3, 99, 77], &program.memory[..]);
    }

    #[test]
    fn prints_when_parameter_is_in_immediate_mode() {
        let memory = [104, 77];
        let mut program = Program::with_io(&memory, || unreachable!(), |i| assert_eq!(77, i));
        assert!(program.run().is_ok());
        assert_eq!(&[104, 77], &program.memory[..]);
    }

    #[test]
    fn system_returns_memory_len() {
        let memory = [5, 4, 3];
        let program = new_program!(&memory);
        assert_eq!(memory.len(), program.get_memory_len());
    }

    #[test]
    fn system_returns_memory_at_address() {
        let memory = [5, 4, 3];
        let program = new_program!(&memory);
        assert_eq!(4, program.read_memory(1));
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 55")]
    fn system_panics_when_requested_memory_address_is_out_of_range() {
        let memory = [5, 4, 3];
        let program = new_program!(&memory);
        program.read_memory(55);
    }

    #[test]
    fn system_writes_memory_at_address() {
        let memory = [5, 4, 3];
        let mut program = new_program!(&memory);
        program.write_memory(1, 7);
        assert_eq!(&[5, 7, 3], &program.memory[..]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 4")]
    fn system_panics_when_asked_to_write_memory_out_of_range() {
        let memory = [5, 4, 3];
        let mut program = new_program!(&memory);
        program.write_memory(4, 0);
    }

    #[test]
    fn system_updates_instruction_pointer() {
        let memory = [5, 4, 3];
        let mut program = new_program!(&memory);
        program.write_instruction_pointer(2);
        assert_eq!(2, program.read_instruction_pointer());
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn system_panics_if_updated_instruction_pointer_is_out_of_range() {
        let memory = [5, 4, 3];
        let mut program = new_program!(&memory);
        program.write_instruction_pointer(memory.len());
    }

    #[test]
    fn system_reads_a_value_from_input() {
        let program = Program::with_io(&[], || 5_i64, |_| unimplemented!());
        assert_eq!(5, program.read_input());
    }

    #[test]
    fn system_writes_a_value_to_output() {
        let mut actual: i64 = 0;
        let mut program = Program::with_io(&[], || unimplemented!(), |x| actual = x);
        program.write_output(5);
        assert_eq!(5, actual);
    }
}
