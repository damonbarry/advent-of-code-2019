#[cfg(test)]
mod tests {
    use crate::intcode::program::Program;

    #[test]
    fn solve_day5_part1() {
        let mut output = Vec::<i64>::new();
        let input = std::fs::read_to_string("src/day05/input.txt").unwrap();
        let memory: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        let mut program = Program::with_io(&memory, || 1, |i| output.push(i));
        assert!(program.run().is_ok());
        let (left, right) = output.split_at(output.len() - 1);
        assert!(left.iter().all(|i| *i == 0));
        assert_eq!(12428642, *right.last().unwrap());
    }

    #[test]
    fn test_day5_part2_example1() {
        let mut output = Vec::<i64>::new();
        let memory = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let mut program = Program::with_io(&memory, || 0, |i| output.push(i));
        assert!(program.run().is_ok());

        let mut program = Program::with_io(&memory, || 1, |i| output.push(i));
        assert!(program.run().is_ok());

        assert_eq!(&[0, 1], &output[..]);
    }

    #[test]
    fn test_day5_part2_example2() {
        let mut output = Vec::<i64>::new();
        let memory = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let mut program = Program::with_io(&memory, || 0, |i| output.push(i));
        assert!(program.run().is_ok());

        let mut program = Program::with_io(&memory, || 1, |i| output.push(i));
        assert!(program.run().is_ok());

        assert_eq!(&[0, 1], &output[..]);
    }

    #[test]
    fn test_day5_part2_example3() {
        let mut output = Vec::<i64>::new();
        let memory = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let mut program = Program::with_io(&memory, || 7, |i| output.push(i));
        assert!(program.run().is_ok());

        let mut program = Program::with_io(&memory, || 8, |i| output.push(i));
        assert!(program.run().is_ok());

        let mut program = Program::with_io(&memory, || 9, |i| output.push(i));
        assert!(program.run().is_ok());

        assert_eq!(&[999, 1000, 1001], &output[..]);
    }

    #[test]
    fn solve_day5_part2() {
        let mut output = Vec::<i64>::new();
        let input = std::fs::read_to_string("src/day05/input.txt").unwrap();
        let memory: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        let mut program = Program::with_io(&memory, || 5, |i| output.push(i));
        assert!(program.run().is_ok());
        let (left, right) = output.split_at(output.len() - 1);
        assert!(left.iter().all(|i| *i == 0));
        assert_eq!(918655, *right.last().unwrap());
    }

}
