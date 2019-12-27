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

        let mut program = Program::new(&memory);
        let result = program.run_with_io(
            || Ok(1),
            |i| {
                output.push(i);
                Ok(())
            },
        );
        assert!(result.is_ok());
        let (left, right) = output.split_at(output.len() - 1);
        assert!(left.iter().all(|i| *i == 0));
        assert_eq!(12428642, *right.last().unwrap());
    }
}
