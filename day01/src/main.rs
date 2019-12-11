use std;
use num;

fn calculate_fuel(mass: i64, include_fuel_mass: bool) -> i64 {
    let result = num::clamp((mass / 3) - 2, 0, std::i64::MAX);

    if result == 0 || !include_fuel_mass {
        result
    } else {
        result + calculate_fuel(result, true)
    }
}

fn main() {
    let result: i64 = std::fs::read_to_string("day01/src/input.txt").unwrap()
        .lines()
        .map(|l| calculate_fuel(l.parse().unwrap(), true))
        .sum();
    println!("{}", result);
}
