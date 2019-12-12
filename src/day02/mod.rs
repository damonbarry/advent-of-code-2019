#[cfg(test)]
mod tests {
    #[test]
    fn can_read_first_opcode() {
        let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
        let program: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        assert_eq!(1, program[0]);
    }
}
