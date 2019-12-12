pub struct IntcodeComputer {
    pub state: Vec<i64>
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer {
            state: Vec::new()
        }
    }

    pub fn run(&mut self, program: impl Iterator<Item = i64>) {
        self.state = program.collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_first_opcode() {
        let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
        let program: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        assert_eq!(1, program[0]);
    }

    #[test]
    fn computer_understands_halt() {
        let program = [99];
        let mut computer = IntcodeComputer::new();
        computer.run(program.iter().cloned());
        assert_eq!(&program[..], &computer.state[..]);
    }
}
