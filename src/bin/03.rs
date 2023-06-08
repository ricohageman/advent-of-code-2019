use itertools::Itertools;
use std::cmp::{max, min};

type Unit = isize;

enum Direction {
    Right(Unit),
    Left(Unit),
    Up(Unit),
    Down(Unit),
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    x: Unit,
    y: Unit,
}

impl Point {
    fn new(x: Unit, y: Unit) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum LineSegment {
    Horizontal { y: Unit, x_range: (Unit, Unit) },
    Vertical { x: Unit, y_range: (Unit, Unit) },
}

impl LineSegment {
    fn new(tuple: (Point, Point)) -> Self {
        if tuple.0.x == tuple.1.x {
            return LineSegment::Vertical {
                x: tuple.0.x,
                y_range: (tuple.0.y, tuple.1.y),
            };
        }

        if tuple.0.y == tuple.1.y {
            return LineSegment::Horizontal {
                y: tuple.0.y,
                x_range: (tuple.0.x, tuple.1.x),
            };
        }

        panic!();
    }

    fn range(&self) -> (Unit, Unit) {
        match self {
            LineSegment::Horizontal { x_range, .. } => *x_range,
            LineSegment::Vertical { y_range, .. } => *y_range,
        }
    }

    fn fixed(&self) -> Unit {
        match self {
            LineSegment::Horizontal { y, .. } => *y,
            LineSegment::Vertical { x, .. } => *x,
        }
    }

    fn directionless_range(&self) -> (Unit, Unit) {
        let range = self.range();
        (min(range.0, range.1), max(range.0, range.1))
    }

    fn length(&self) -> Unit {
        let directionless_range = self.directionless_range();
        directionless_range.1 - directionless_range.0
    }

    fn distance_to(&self, point: &Point) -> Unit {
        match self {
            LineSegment::Horizontal { x_range, y } => {
                assert_eq!(point.y, *y);
                point.x - x_range.0
            }
            LineSegment::Vertical { y_range, x } => {
                assert_eq!(point.x, *x);
                point.y - y_range.0
            }
        }
    }

    fn intersection(self, other: Self) -> Option<Point> {
        match (self, other) {
            (LineSegment::Vertical { x, .. }, LineSegment::Horizontal { y, .. }) => {
                let x_range = other.directionless_range();
                if !(x_range.0..=x_range.1).contains(&x) {
                    return None;
                }

                let y_range = self.directionless_range();
                if !(y_range.0..=y_range.1).contains(&y) {
                    return None;
                }

                Some(Point::new(x, y))
            }
            (LineSegment::Horizontal { y, x_range }, LineSegment::Vertical { x, y_range }) => {
                let x_range = self.directionless_range();
                if !(x_range.0..=x_range.1).contains(&x) {
                    return None;
                }

                let y_range = other.directionless_range();
                if !(y_range.0..=y_range.1).contains(&y) {
                    return None;
                }

                Some(Point::new(x, y))
            }
            _ => return None,
        }
    }
}

fn apply(point: Point, direction: Direction) -> Point {
    match direction {
        Direction::Right(amount) => Point::new(point.x + amount, point.y),
        Direction::Left(amount) => Point::new(point.x - amount, point.y),
        Direction::Up(amount) => Point::new(point.x, point.y + amount),
        Direction::Down(amount) => Point::new(point.x, point.y - amount),
    }
}

fn parse_input(input: &str) -> (Vec<LineSegment>, Vec<LineSegment>) {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|element| {
                    let amount = element[1..element.len()].parse().unwrap();

                    match element.chars().next().unwrap() {
                        'R' => Direction::Right(amount),
                        'L' => Direction::Left(amount),
                        'U' => Direction::Up(amount),
                        'D' => Direction::Down(amount),
                        _ => panic!(),
                    }
                })
                .fold(vec![Point::new(0, 0)], |mut visited_points, direction| {
                    let updated_point = apply(*visited_points.last().unwrap(), direction);
                    visited_points.push(updated_point);

                    visited_points
                })
                .into_iter()
                .tuple_windows()
                .map(LineSegment::new)
                .collect::<Vec<LineSegment>>()
        })
        .collect_tuple()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<isize> {
    let (first_wire, second_wire) = parse_input(input);

    first_wire
        .into_iter()
        .cartesian_product(second_wire)
        .filter_map(|(first, second)| first.intersection(second))
        .filter(|intersection| *intersection != Point::new(0, 0))
        .map(|intersection| intersection.x.abs() + intersection.y.abs())
        .min()
}

pub fn part_two(input: &str) -> Option<isize> {
    let (first_wire, second_wire) = parse_input(input);
    let first_wire_length = first_wire.iter().fold(vec![0], |mut lengths, segment| {
        lengths.push(lengths.last().unwrap() + segment.length());
        lengths
    });

    let second_wire_length = second_wire.iter().fold(vec![0], |mut lengths, segment| {
        lengths.push(lengths.last().unwrap() + segment.length());
        lengths
    });

    first_wire
        .into_iter()
        .enumerate()
        .cartesian_product(second_wire.into_iter().enumerate())
        .filter_map(|((i, first), (j, second))| {
            let intersection = first.intersection(second)?;

            if intersection.x == 0 && intersection.y == 0 {
                return None;
            }

            Some(
                first_wire_length[i]
                    + second_wire_length[j]
                    + first.distance_to(&intersection)
                    + second.distance_to(&intersection),
            )
        })
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_intersection() {
        let a = LineSegment::new((Point::new(7, 6), Point::new(7, 2)));
        let b = LineSegment::new((Point::new(3, 5), Point::new(8, 5)));

        assert_eq!(a.intersection(b), Some(Point::new(7, 5)))
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(159));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(610));
    }
}
