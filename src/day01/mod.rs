use num;
use std;

pub fn sum_fuel(masses: impl Iterator<Item = i64>, include_fuel_mass: bool) -> i64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_fuel_for_module_mass_including_fuel_mass() {
        let input = std::fs::read_to_string("src/day01/input.txt").unwrap();
        let masses = input.lines().map(|l| l.parse().unwrap());
        let result = sum_fuel(masses, true);
        assert_eq!(5018888, result);
    }

    #[test]
    fn can_calculate_fuel_for_module_mass_without_fuel_mass() {
        let input = std::fs::read_to_string("src/day01/input.txt").unwrap();
        let masses = input.lines().map(|l| l.parse().unwrap());
        let result = sum_fuel(masses, false);
        assert_eq!(3347838, result);
    }

    #[test]
    fn module_with_mass_14_requires_2_fuel() {
        let result = sum_fuel([14].iter().cloned(), true);
        assert_eq!(2, result);
    }

    #[test]
    fn module_with_mass_1969_requires_966_fuel() {
        let result = sum_fuel([1969].iter().cloned(), true);
        assert_eq!(966, result);
    }

    #[test]
    fn module_with_mass_100756_requires_966_fuel() {
        let result = sum_fuel([100756].iter().cloned(), true);
        assert_eq!(50346, result);
    }
}
