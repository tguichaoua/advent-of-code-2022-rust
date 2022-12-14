use std::collections::HashSet;

use adventofcode::helpers::range;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn down(self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

mod parser {
    use super::Pos;

    use adventofcode::helpers::parser::decimal_value;
    use nom::{
        bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
        IResult,
    };

    pub fn line(input: &str) -> IResult<&str, Vec<Pos>> {
        separated_list1(tag(" -> "), pos)(input)
    }

    fn pos(input: &str) -> IResult<&str, Pos> {
        map(
            separated_pair(decimal_value, tag(","), decimal_value),
            |(x, y)| Pos { x, y },
        )(input)
    }
}

fn parse_input(input: &str) -> HashSet<Pos> {
    input
        .lines()
        .map(parser::line)
        .map(Result::unwrap)
        .map(|(_, x)| x)
        .flat_map(|points| {
            points
                .into_iter()
                .tuple_windows()
                .flat_map(|(a, b)| -> Box<dyn Iterator<Item = Pos>> {
                    if a.x == b.x {
                        let x = a.x;
                        Box::new(range(a.y, b.y).map(move |y| Pos { x, y }))
                    } else {
                        let y = a.y;
                        Box::new(range(a.x, b.x).map(move |x| Pos { x, y }))
                    }
                })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut walls = parse_input(input);

    let void_y = walls.iter().map(|Pos { x: _, y }| y).max().unwrap() + 1;

    const SAND_SOURCE_POS: Pos = Pos { x: 500, y: 0 };

    let mut rested_sand = 0;

    'outer: loop {
        let mut sand_pos = SAND_SOURCE_POS;
        loop {
            if sand_pos.y == void_y {
                break 'outer;
            }
            if !walls.contains(&sand_pos.down()) {
                sand_pos = sand_pos.down();
                continue;
            }
            if !walls.contains(&sand_pos.down_left()) {
                sand_pos = sand_pos.down_left();
                continue;
            }
            if !walls.contains(&sand_pos.down_right()) {
                sand_pos = sand_pos.down_right();
                continue;
            }
            walls.insert(sand_pos);
            rested_sand += 1;
            break;
        }
    }

    Some(rested_sand)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut walls = parse_input(input);

    let floor_y = walls.iter().map(|Pos { x: _, y }| y).max().unwrap() + 1;

    const SAND_SOURCE_POS: Pos = Pos { x: 500, y: 0 };

    let mut rested_sand = 0;

    'outer: loop {
        let mut sand_pos = SAND_SOURCE_POS;
        loop {
            if sand_pos.y != floor_y {
                if !walls.contains(&sand_pos.down()) {
                    sand_pos = sand_pos.down();
                    continue;
                }
                if !walls.contains(&sand_pos.down_left()) {
                    sand_pos = sand_pos.down_left();
                    continue;
                }
                if !walls.contains(&sand_pos.down_right()) {
                    sand_pos = sand_pos.down_right();
                    continue;
                }
            }

            rested_sand += 1;

            if sand_pos == SAND_SOURCE_POS {
                break 'outer;
            }

            walls.insert(sand_pos);
            break;
        }
    }

    Some(rested_sand)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 14);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
