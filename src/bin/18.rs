use std::collections::{HashSet, VecDeque};

use adventofcode::helpers::parser::decimal_value;
use glam::UVec3;
use nom::{character::complete::char, sequence::terminated};

fn parse_input(input: &str) -> impl Iterator<Item = UVec3> + '_ {
    input.lines().map(|line| {
        let (rest, x) = terminated(decimal_value, char(','))(line).unwrap();
        let (rest, y) = terminated(decimal_value, char(','))(rest).unwrap();
        let (_, z) = decimal_value(rest).unwrap();
        UVec3 { x, y, z }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let cubes: HashSet<_> = parse_input(input).collect();

    let mut visible_faces = 0;

    macro_rules! check_face {
        ($pos: expr) => {
            if !cubes.contains(&$pos) {
                visible_faces += 1;
            }
        };
    }

    for &UVec3 { x, y, z } in &cubes {
        check_face!(UVec3::new(x + 1, y, z));
        check_face!(UVec3::new(x, y + 1, z));
        check_face!(UVec3::new(x, y, z + 1));
        if let Some(x) = x.checked_sub(1) {
            check_face!(UVec3::new(x, y, z));
        } else {
            visible_faces += 1;
        }
        if let Some(y) = y.checked_sub(1) {
            check_face!(UVec3::new(x, y, z));
        } else {
            visible_faces += 1;
        }
        if let Some(z) = z.checked_sub(1) {
            check_face!(UVec3::new(x, y, z));
        } else {
            visible_faces += 1;
        }
    }

    Some(visible_faces)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cubes: HashSet<_> = parse_input(input).collect();

    let max_coordinates = cubes.iter().fold(UVec3::ZERO, |acc, &cur| acc.max(cur)) + UVec3::ONE;

    let mut visible_faces = 0;

    let is_connected_to_outside = |pos: UVec3| {
        let mut stack = VecDeque::new();
        stack.push_back(pos);
        let mut visisted = HashSet::new();

        while let Some(pos) = stack.pop_back() {
            visisted.insert(pos);

            if cubes.contains(&pos) {
                continue;
            }

            if pos.min_element() == 0
                || pos.x == max_coordinates.x
                || pos.y == max_coordinates.y
                || pos.z == max_coordinates.z
            {
                return true;
            }

            let mut push = |pos| {
                if !visisted.contains(&pos) {
                    stack.push_back(pos);
                }
            };

            push(UVec3::new(pos.x - 1, pos.y, pos.z));
            push(UVec3::new(pos.x + 1, pos.y, pos.z));
            push(UVec3::new(pos.x, pos.y - 1, pos.z));
            push(UVec3::new(pos.x, pos.y + 1, pos.z));
            push(UVec3::new(pos.x, pos.y, pos.z - 1));
            push(UVec3::new(pos.x, pos.y, pos.z + 1));
        }
        false
    };

    for &UVec3 { x, y, z } in &cubes {
        if is_connected_to_outside(UVec3::new(x + 1, y, z)) {
            visible_faces += 1;
        }
        if is_connected_to_outside(UVec3::new(x, y + 1, z)) {
            visible_faces += 1;
        }
        if is_connected_to_outside(UVec3::new(x, y, z + 1)) {
            visible_faces += 1;
        }

        if let Some(x) = x.checked_sub(1) {
            if is_connected_to_outside(UVec3::new(x, y, z)) {
                visible_faces += 1;
            }
        } else {
            visible_faces += 1;
        }
        if let Some(y) = y.checked_sub(1) {
            if is_connected_to_outside(UVec3::new(x, y, z)) {
                visible_faces += 1;
            }
        } else {
            visible_faces += 1;
        }
        if let Some(z) = z.checked_sub(1) {
            if is_connected_to_outside(UVec3::new(x, y, z)) {
                visible_faces += 1;
            }
        } else {
            visible_faces += 1;
        }
    }

    Some(visible_faces)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 18);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
