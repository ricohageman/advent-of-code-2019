fn simulate(intcode: &mut [usize], noun: usize, verb: usize) -> usize {
    intcode[1] = noun;
    intcode[2] = verb;

    let mut current_index = 0;

    loop {
        match intcode[current_index] {
            99 => break,
            1 => {
                let source_1 = intcode[current_index + 1];
                let source_2 = intcode[current_index + 2];
                let target = intcode[current_index + 3];

                intcode[target] = intcode[source_1] + intcode[source_2];
            }
            2 => {
                let source_1 = intcode[current_index + 1];
                let source_2 = intcode[current_index + 2];
                let target = intcode[current_index + 3];

                intcode[target] = intcode[source_1] * intcode[source_2];
            }
            _ => panic!("{}", intcode[current_index]),
        }

        current_index += 4;
    }

    intcode[0]
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut intcode = input
        .split(",")
        .map(|element| element.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    Some(simulate(&mut intcode, 12, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let intcode = input
        .split(",")
        .map(|element| element.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (0..=99)
        .flat_map(|noun| (0..=99).map(move |verb| (noun, verb)))
        .filter(|(noun, verb)| simulate(&mut intcode.clone(), *noun, *verb) == 19690720)
        .next()
        .map(|(noun, verb)| 100 * noun + verb)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
