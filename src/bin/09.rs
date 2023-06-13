use advent_of_code::helpers::{Input, Intcode, IntcodeComputer};

pub fn part_one(input: &str) -> Option<usize> {
    let intcode = Intcode::from_input(input);
    let mut computer = IntcodeComputer::new(intcode);

    let input = Input::new(vec![1]);
    let output = computer.simulate(input);

    assert_eq!(output.len(), 1);
    Some(*output.last().unwrap() as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let intcode = Intcode::from_input(input);
    let mut computer = IntcodeComputer::new(intcode);

    let input = Input::new(vec![2]);
    let output = computer.simulate(input);

    assert_eq!(output.len(), 1);
    Some(*output.last().unwrap() as usize)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
