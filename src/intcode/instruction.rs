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

fn process_parameters<T: super::program::System>(
    system: &T,
    param_types: &[ParameterType],
    read_modes: &[ParameterMode],
) -> Result<(Vec<i64>, Vec<usize>), ErrorKind> {
    let instruction_size = 1 + param_types.len();
    let count_read_params = param_types
        .iter()
        .filter(|ty| **ty == ParameterType::Read)
        .count();

    if read_modes.len() != count_read_params {
        return Err(ErrorKind::ReadModeMismatch(
            count_read_params,
            read_modes.len(),
        ));
    }

    let address = system.read_instruction_pointer();
    if address + instruction_size > system.get_memory_len() {
        return Err(ErrorKind::NotEnoughParameters);
    }

    let address = address + 1; // skip over opcode to the 1st param

    let mut read_iter = read_modes.iter();
    let mut read_values = Vec::<i64>::new();
    let mut write_addrs = Vec::<usize>::new();
    for (index, param) in param_types.iter().enumerate() {
        match param {
            ParameterType::Read => {
                let mode = *read_iter
                    .next()
                    .expect("Read modes don't align with actual parameters");
                match mode {
                    ParameterMode::Position => {
                        let address = system.read_memory(address + index) as usize;
                        if address > system.get_memory_len() {
                            return Err(ErrorKind::AddressOutOfRange(address));
                        } else {
                            read_values.push(system.read_memory(address));
                        }
                    }
                    ParameterMode::Immediate => {
                        read_values.push(system.read_memory(address + index))
                    }
                };
            }
            ParameterType::Write => {
                let address = system.read_memory(address + index) as usize;
                if address > system.get_memory_len() {
                    return Err(ErrorKind::AddressOutOfRange(address));
                }
                write_addrs.push(address);
            }
        }
    }

    Ok((read_values, write_addrs))
}

pub fn add<T: super::program::System>(
    system: &mut T,
    read_modes: &[ParameterMode],
) -> Result<usize, ErrorKind> {
    const INSTRUCTION_SIZE: usize = 4;
    let (read_values, write_addrs) = process_parameters(
        system,
        &[
            ParameterType::Read,
            ParameterType::Read,
            ParameterType::Write,
        ],
        read_modes,
    )?;
    system.write_memory(write_addrs[0], read_values[0] + read_values[1]);
    Ok(system.read_instruction_pointer() + INSTRUCTION_SIZE)
}

pub fn multiply<T: super::program::System>(
    system: &mut T,
    read_modes: &[ParameterMode],
) -> Result<usize, ErrorKind> {
    const INSTRUCTION_SIZE: usize = 4;
    let (read_values, write_addrs) = process_parameters(
        system,
        &[
            ParameterType::Read,
            ParameterType::Read,
            ParameterType::Write,
        ],
        read_modes,
    )?;
    system.write_memory(write_addrs[0], read_values[0] * read_values[1]);
    Ok(system.read_instruction_pointer() + INSTRUCTION_SIZE)
}

pub fn print<T: super::program::System>(
    system: &mut T,
    read_mode: ParameterMode,
) -> Result<usize, ErrorKind> {
    const INSTRUCTION_SIZE: usize = 2;
    let (read_values, _) = process_parameters(system, &[ParameterType::Read], &[read_mode])?;
    system.write_output(read_values[0]);
    Ok(system.read_instruction_pointer() + INSTRUCTION_SIZE)
}

pub fn store<T: super::program::System>(system: &mut T) -> Result<usize, ErrorKind> {
    const INSTRUCTION_SIZE: usize = 2;
    let (_, write_addrs) = process_parameters(system, &[ParameterType::Write], &[])?;
    system.write_memory(write_addrs[0], system.read_input());
    Ok(system.read_instruction_pointer() + INSTRUCTION_SIZE)
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
