fn required_fuel_for_mass(mass: usize) -> usize {
    let partial = mass / 3;

    if partial < 2 {
        return 0;
    }

    partial - 2
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input.lines()
            .map(|line| line.parse::<usize>().unwrap())
            .map(required_fuel_for_mass)
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input.lines()
            .map(|line| line.parse::<usize>().unwrap())
            .map(|mass| {
                let mut total_fuel = required_fuel_for_mass(mass);
                let mut fuel = total_fuel;

                while fuel > 0 {
                    fuel = required_fuel_for_mass(fuel);
                    total_fuel += fuel;
                }

                total_fuel
            })
            .sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(34241));
    }
}
