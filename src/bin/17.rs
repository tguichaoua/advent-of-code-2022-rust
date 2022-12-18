#![feature(inline_const)]

use std::collections::HashSet;

use glam::UVec2;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Horizontal,
    Plus,
    J,
    Vertical,
    Square,
}

impl Shape {
    fn height(self) -> u32 {
        match self {
            Shape::Horizontal => 1,
            Shape::Plus => 3,
            Shape::J => 3,
            Shape::Vertical => 4,
            Shape::Square => 2,
        }
    }
    fn width(self) -> u32 {
        match self {
            Shape::Horizontal => 4,
            Shape::Plus => 3,
            Shape::J => 3,
            Shape::Vertical => 1,
            Shape::Square => 2,
        }
    }
    fn points(self) -> &'static [UVec2] {
        match self {
            Shape::Horizontal => {
                const {
                    &[
                        UVec2::new(0, 0),
                        UVec2::new(1, 0),
                        UVec2::new(2, 0),
                        UVec2::new(3, 0),
                    ]
                }
            }
            Shape::Plus => {
                const {
                    &[
                        UVec2::new(1, 0),
                        UVec2::new(0, 1),
                        UVec2::new(1, 1),
                        UVec2::new(2, 1),
                        UVec2::new(1, 2),
                    ]
                }
            }
            Shape::J => {
                const {
                    &[
                        UVec2::new(2, 2),
                        UVec2::new(2, 1),
                        UVec2::new(0, 0),
                        UVec2::new(1, 0),
                        UVec2::new(2, 0),
                    ]
                }
            }
            Shape::Vertical => {
                const {
                    &[
                        UVec2::new(0, 0),
                        UVec2::new(0, 1),
                        UVec2::new(0, 2),
                        UVec2::new(0, 3),
                    ]
                }
            }
            Shape::Square => {
                const {
                    &[
                        UVec2::new(0, 0),
                        UVec2::new(1, 0),
                        UVec2::new(0, 1),
                        UVec2::new(1, 1),
                    ]
                }
            }
        }
    }
}

static SHAPE_ORDER: [Shape; 5] = [
    Shape::Horizontal,
    Shape::Plus,
    Shape::J,
    Shape::Vertical,
    Shape::Square,
];

#[derive(Debug)]
enum Push {
    Left,
    Right,
}

fn parse_input(input: &str) -> impl Iterator<Item = Push> + Clone + '_ {
    input.bytes().filter_map(|b| match b {
        b'<' => Some(Push::Left),
        b'>' => Some(Push::Right),
        _ => None,
    })
}

const CHAMBER_WIDTH: u32 = 7;

pub fn part_one(input: &str) -> Option<u32> {
    let mut moves = parse_input(input).cycle();
    let shapes = SHAPE_ORDER.into_iter().cycle().take(2022);

    let mut stopped_rocks = HashSet::new();
    let mut toppest = 0;

    for shape in shapes {
        let width = shape.width();
        let height = shape.height();

        // The coordinate of the bottom-left corner of the shape.
        let mut pos = UVec2::new(2, toppest + 3);

        let has_collision = |pos: UVec2| {
            shape
                .points()
                .iter()
                .map(|&p| p + pos)
                .any(|p| stopped_rocks.contains(&p))
        };

        loop {
            let dx = moves.next().unwrap();
            let new_x = match dx {
                Push::Left if pos.x != 0 => Some(pos.x - 1),
                Push::Right if pos.x + width != CHAMBER_WIDTH => Some(pos.x + 1),
                _ => None,
            };

            // If we move horizontaly,
            if let Some(new_x) = new_x {
                // and if there is no collision with another rock,
                if !has_collision(UVec2::new(new_x, pos.y)) {
                    // apply the move.
                    pos.x = new_x;
                }
            }

            // If we don't reach the floor, try move down.
            if pos.y != 0 {
                let new_y = pos.y - 1;
                if !has_collision(UVec2::new(pos.x, new_y)) {
                    pos.y = new_y;
                    continue;
                }
            }

            // If we reach the floor, or a collision happen, stop the rock.
            stopped_rocks.extend(shape.points().iter().map(|&p| p + pos));
            let top = pos.y + height;
            if top > toppest {
                toppest = top;
            }
            break;
        }
    }

    Some(toppest)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn main() {
    let input = &adventofcode::read_file("inputs", 17);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 17);
        // assert_eq!(part_two(&input), Some(1514285714288));
        assert_eq!(part_two(&input), None);
    }
}
