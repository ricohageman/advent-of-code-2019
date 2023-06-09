use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let minimum = 284639;
    let maximum = 748759;

    Some(
        (minimum..=maximum)
            .filter(|number| {
                let digits = number
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect::<Vec<_>>();

                // Check if the numbers are monotonic increasing.
                if digits.iter().tuple_windows().any(|(a, b)| b < a) {
                    return false;
                }

                // Check if there is at least one double number
                digits.iter().tuple_windows().any(|(a, b)| a == b)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let minimum = 284639;
    let maximum = 748759;

    Some(
        (minimum..=maximum)
            .filter(|number| {
                let digits = number
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect::<Vec<_>>();

                // Check if the numbers are monotonic increasing.
                if digits.iter().tuple_windows().any(|(a, b)| b < a) {
                    return false;
                }

                // Check if there is at least one sequence of similar numbers of size 2
                digits
                    .iter()
                    .group_by(|a| **a)
                    .into_iter()
                    .any(|(_, group)| group.into_iter().count() == 2)
            })
            .count(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
