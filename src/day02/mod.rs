#[cfg(test)]
mod tests {
    use crate::intcode::program::Program;

    #[test]
    fn runs_first_example_program() {
        let memory = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            &program.memory[..]
        );
    }

    #[test]
    fn runs_second_example_program() {
        let memory = [1, 0, 0, 0, 99];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 0, 0, 0, 99], &program.memory[..]);
    }

    #[test]
    fn runs_third_example_program() {
        let memory = [2, 3, 0, 3, 99];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 3, 0, 6, 99], &program.memory[..]);
    }

    #[test]
    fn runs_fourth_example_program() {
        let memory = [2, 4, 4, 5, 99, 0];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[2, 4, 4, 5, 99, 9801], &program.memory[..]);
    }

    #[test]
    fn runs_fifth_example_program() {
        let memory = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[30, 1, 1, 4, 2, 5, 6, 0, 99], &program.memory[..]);
    }

    #[test]
    fn solve_day2_part1() {
        let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
        let mut memory: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        memory[1] = 12;
        memory[2] = 2;

        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(8017076, program.memory[0]);
    }

    #[test]
    fn solve_day2_part2() {
        let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
        let init: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        // for i in 0..99 {
        //     for j in 0..99 {
        //         let mut memory = init.clone();
        //         memory[1] = i;
        //         memory[2] = j;

        //         let mut program = Program::new(memory);
        //         let result = program.run();
        //         assert!(result.is_ok());

        //         if program.memory[0] == 19690720 {
        //             // to see this output: `cargo test -- --nocapture`
        //             println!("### VALUES: [1] = {}, [2] = {}", i, j);
        //             break;
        //         }
        //     }
        // }

        let mut memory = init.clone();
        memory[1] = 31;
        memory[2] = 46;

        let mut program = Program::new(&memory);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(19690720, program.memory[0]);
    }
}
