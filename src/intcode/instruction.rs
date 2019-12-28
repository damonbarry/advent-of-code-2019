#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Addition {
        param1: ParameterMode,
        param2: ParameterMode,
    },
    Halt,
    Multiplication {
        param1: ParameterMode,
        param2: ParameterMode,
    },
    Print {
        param: ParameterMode,
    },
    Store,
}

impl Opcode {
    const FIRST: u32 = 1;
    const SECOND: u32 = 2;

    pub fn parse(value: i64) -> Result<Self, ErrorKind> {
        match value % 100 {
            1 => Ok(Opcode::Addition {
                param1: Self::parse_parameter_mode(value, Self::FIRST)?,
                param2: Self::parse_parameter_mode(value, Self::SECOND)?,
            }),
            2 => Ok(Opcode::Multiplication {
                param1: Self::parse_parameter_mode(value, Self::FIRST)?,
                param2: Self::parse_parameter_mode(value, Self::SECOND)?,
            }),
            3 => Ok(Opcode::Store),
            4 => Ok(Opcode::Print {
                param: Self::parse_parameter_mode(value, Self::FIRST)?,
            }),
            99 => Ok(Opcode::Halt),
            _ => Err(ErrorKind::InvalidOpcode),
        }
    }

    fn parse_parameter_mode(value: i64, which: u32) -> Result<ParameterMode, ErrorKind> {
        let place = 10_i64.checked_pow(which + 1).unwrap();
        match value / place {
            0 | 10 => Ok(ParameterMode::Position),
            1 | 11 => Ok(ParameterMode::Immediate),
            _ => Err(ErrorKind::InvalidParameterMode(which as usize)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "Parameter at offset {} is out of range", _0)]
    AddressOutOfRange(usize),
    #[fail(display = "Encountered invalid opcode")]
    InvalidOpcode,
    #[fail(display = "Encountered invalid mode for parameter at offset {}", _0)]
    InvalidParameterMode(usize),
    #[fail(display = "Not enough parameters in memory to interpret instruction")]
    NotEnoughParameters,
    #[fail(
        display = "Instruction has {} read parameters, but {} ParameterMode values were given",
        _0, _1
    )]
    ReadModeMismatch(usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterType {
    Read,
    Write,
}

macro_rules! instruction {
    (
        name: $name:ident,
        parameters: [ $( $param:path ),* ],
        execute: $execute:expr,
    ) => {
        pub fn $name(
            memory: &mut [i64],
            address: usize,
            read_modes: &[ParameterMode],
        ) -> Result<usize, ErrorKind> {
            const PARAMETER_TYPES: &[ParameterType] = &[ $($param),* ];
            const INSTRUCTION_SIZE: usize = 1 + PARAMETER_TYPES.len();

            let count_read_params = PARAMETER_TYPES.iter().filter(|ty| **ty == ParameterType::Read).count();
            if read_modes.len() != count_read_params {
                return Err(ErrorKind::ReadModeMismatch(count_read_params, read_modes.len()));
            }

            if address + INSTRUCTION_SIZE > memory.len() {
                return Err(ErrorKind::NotEnoughParameters);
            }

            let advance_address = address + INSTRUCTION_SIZE;
            let address = address + 1; // skip over opcode to the 1st param

            let mut read_iter = read_modes.iter();
            let mut read_params = Vec::<i64>::new();
            let mut write_addrs = Vec::<usize>::new();
            for (index, param) in PARAMETER_TYPES.iter().enumerate() {
                match param {
                    ParameterType::Read => {
                        let mode = *read_iter.next().expect("Read modes don't align with actual parameters");
                        match mode {
                            ParameterMode::Position => {
                                let address = memory[address + index] as usize;
                                if address > memory.len() {
                                    return Err(ErrorKind::AddressOutOfRange(address));
                                } else {
                                    read_params.push(memory[address]);
                                }
                            }
                            ParameterMode::Immediate => read_params.push(memory[address + index]),
                        };
                    }
                    ParameterType::Write => {
                        let address = memory[address + index] as usize;
                        if address > memory.len() {
                            return Err(ErrorKind::AddressOutOfRange(address));
                        }
                        write_addrs.push(address);
                    }
                }
            }

            let mut results = vec![0; write_addrs.len()];
            $execute(read_params, &mut results);

            for (address, value) in write_addrs.iter().zip(results) {
                memory[*address] = value;
            }

            Ok(advance_address)
        }
    };
}

instruction! {
    name: add,
    parameters: [ParameterType::Read, ParameterType::Read, ParameterType::Write],
    execute: |read_params: Vec<i64>, write_params: &mut Vec<i64>| write_params[0] = read_params[0] + read_params[1],
}

instruction! {
    name: multiply,
    parameters: [ParameterType::Read, ParameterType::Read, ParameterType::Write],
    execute: |read_params: Vec<i64>, write_params: &mut Vec<i64>| write_params[0] = read_params[0] * read_params[1],
}

pub fn print(
    memory: &mut [i64],
    address: usize,
    read_mode: ParameterMode,
    out_value: &mut i64,
) -> Result<usize, ErrorKind> {
    const INSTRUCTION_SIZE: usize = 2;

    if address + INSTRUCTION_SIZE > memory.len() {
        return Err(ErrorKind::NotEnoughParameters);
    }

    let advance_address = address + INSTRUCTION_SIZE;
    let address = address + 1;

    *out_value = match read_mode {
        ParameterMode::Position => {
            let address = memory[address] as usize;
            if address > memory.len() {
                Err(ErrorKind::AddressOutOfRange(address))
            } else {
                Ok(memory[address])
            }
        }
        ParameterMode::Immediate => Ok(memory[address]),
    }?;

    Ok(advance_address)
}

pub fn store(memory: &mut [i64], address: usize, input: i64) -> Result<usize, ErrorKind> {
    const INSTRUCTION_SIZE: usize = 2;

    if address + INSTRUCTION_SIZE > memory.len() {
        return Err(ErrorKind::NotEnoughParameters);
    }

    let advance_address = address + INSTRUCTION_SIZE;
    let address = address + 1;

    let address = memory[address] as usize;
    if address > memory.len() {
        return Err(ErrorKind::AddressOutOfRange(address));
    }

    memory[address] = input;
    Ok(advance_address)
}

#[cfg(test)]
mod tests {
    mod opcode {
        use super::super::*;

        #[test]
        fn can_parse_addition() {
            assert_eq!(
                Opcode::Addition {
                    param1: ParameterMode::Position,
                    param2: ParameterMode::Position
                },
                Opcode::parse(1).unwrap()
            );
        }

        #[test]
        fn can_parse_addition_with_first_immediate_parameter() {
            assert_eq!(
                Opcode::Addition {
                    param1: ParameterMode::Immediate,
                    param2: ParameterMode::Position
                },
                Opcode::parse(101).unwrap()
            );
        }

        #[test]
        fn can_parse_addition_with_second_immediate_parameter() {
            assert_eq!(
                Opcode::Addition {
                    param1: ParameterMode::Position,
                    param2: ParameterMode::Immediate,
                },
                Opcode::parse(1001).unwrap()
            );
        }

        #[test]
        fn can_parse_addition_with_immediate_parameters() {
            assert_eq!(
                Opcode::Addition {
                    param1: ParameterMode::Immediate,
                    param2: ParameterMode::Immediate,
                },
                Opcode::parse(1101).unwrap()
            );
        }
    }
}
