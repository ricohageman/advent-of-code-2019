use advent_of_code::helpers::{Intcode, IntcodeComputer};

fn simulate(intcode: Intcode, noun: usize, verb: usize) -> usize {
    let mut intcode = intcode;
    intcode.set(1, noun);
    intcode.set(2, verb);

    let mut computer = IntcodeComputer::new(intcode);
    computer.simulate_without_input();
    computer.code.get(0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let intcode = Intcode::from_input(input);

    Some(simulate(intcode, 12, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let intcode = Intcode::from_input(input);

    (0..=99)
        .flat_map(|noun| (0..=99).map(move |verb| (noun, verb)))
        .filter(|(noun, verb)| simulate(intcode.clone(), *noun, *verb) == 19690720)
        .next()
        .map(|(noun, verb)| 100 * noun + verb)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
