#![feature(get_many_mut)]

use std::{collections::HashSet, str::FromStr};

enum Move {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse_moves(input: &str) -> impl Iterator<Item = (Move, u32)> + '_ {
    input.lines().map(|line| {
        let (direction, distance) = line.split_at(1);
        // remove the whitespace
        let (_, distance) = distance.split_at(1);

        let direction = match direction.as_bytes()[0] {
            b'U' => Move::Up,
            b'D' => Move::Down,
            b'R' => Move::Right,
            b'L' => Move::Left,
            _ => unreachable!(),
        };

        let distance = FromStr::from_str(distance).unwrap();

        (direction, distance)
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let moves = parse_moves(input);

    let mut head = Pos::default();
    let mut tail = Pos::default();
    let mut visited = HashSet::new();
    visited.insert(tail);

    for (dir, dist) in moves {
        for _ in 0..dist {
            match dir {
                Move::Up => head.y += 1,
                Move::Down => head.y -= 1,
                Move::Right => head.x += 1,
                Move::Left => head.x -= 1,
            }

            if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
                match dir {
                    Move::Up => {
                        tail.x = head.x;
                        tail.y = head.y - 1;
                    }
                    Move::Down => {
                        tail.x = head.x;
                        tail.y = head.y + 1;
                    }
                    Move::Right => {
                        tail.x = head.x - 1;
                        tail.y = head.y;
                    }
                    Move::Left => {
                        tail.x = head.x + 1;
                        tail.y = head.y;
                    }
                }
                visited.insert(tail);
            }
        }
    }

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let moves = parse_moves(input);

    let mut knots = [Pos::default(); 10];
    let mut visited = HashSet::new();
    visited.insert(knots[9]);

    for (dir, dist) in moves {
        for _ in 0..dist {
            match dir {
                Move::Up => knots[0].y += 1,
                Move::Down => knots[0].y -= 1,
                Move::Right => knots[0].x += 1,
                Move::Left => knots[0].x -= 1,
            }

            let mut tail_moved = true;
            for i in 0..knots.len() - 1 {
                let [a, b] = knots.get_many_mut([i, i + 1]).unwrap();

                let dx = a.x - b.x;
                let dy = a.y - b.y;

                if dx.abs() < 2 && dy.abs() < 2 {
                    tail_moved = false;
                    break;
                }

                b.x += dx.signum();
                b.y += dy.signum();
            }

            if tail_moved {
                visited.insert(knots[9]);
            }
        }
    }

    Some(visited.len())
}

fn main() {
    let input = &adventofcode::read_file("inputs", 9);
    {
        // HashSet does some stuff when used for the first time
        // that took ~300 µs (on my machine).
        // Create a HashSet here to not impact the solutions performance.
        let mut dummy = HashSet::with_capacity(1);
        dummy.insert(0u8);
    }
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = {
            let cwd = std::env::current_dir().unwrap();
            let filepath = cwd.join("src").join("examples").join(format!("09_bis.txt"));
            let f = std::fs::read_to_string(filepath);
            f.expect("could not open input file")
        };
        assert_eq!(part_two(&input), Some(36));
    }
}
