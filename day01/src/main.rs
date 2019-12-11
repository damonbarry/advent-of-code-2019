use num;
use std;

fn sum_fuel(masses: impl Iterator<Item = i64>, include_fuel_mass: bool) -> i64 {
    masses.map(|m| calculate_fuel(m, include_fuel_mass)).sum()
}

fn calculate_fuel(mass: i64, include_fuel_mass: bool) -> i64 {
    let result = num::clamp((mass / 3) - 2, 0, std::i64::MAX);

    if result == 0 || !include_fuel_mass {
        result
    } else {
        result + calculate_fuel(result, true)
    }
}

fn main() {
    let input = std::fs::read_to_string("day01/src/input.txt").unwrap();
    let masses = input.lines().map(|l| l.parse().unwrap());
    let result = sum_fuel(masses, true);
    println!("{}", result);
}
