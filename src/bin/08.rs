use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;

/// Convert a digit character from '0'..='9' to its numerical value.
fn fast_parse_digit(x: u8) -> u8 {
    debug_assert!((b'0'..=b'9').contains(&x));
    x - b'0'
}

struct Map {
    trees: Box<[u8]>,
    width: usize,
}

impl Map {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.trees.len() / self.width
    }
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> u8 {
        debug_assert!((0..self.width()).contains(&x));
        debug_assert!((0..self.height()).contains(&y));
        let idx = y * self.width() + x;
        // Safety: safety is upheld by the caller.
        unsafe { *self.trees.get_unchecked(idx) }
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().as_bytes().len();
        let trees = s
            .lines()
            .flat_map(|line| line.bytes().map(fast_parse_digit))
            .collect_vec()
            .into_boxed_slice();
        Ok(Map { trees, width })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();
    // edge trees are always visible.
    let mut visible_tree_count = 2 * map.width() + 2 * (map.height() - 2);

    for y in 1..map.height() - 1 {
        for x in 1..map.width() - 1 {
            // Safety: x and y are in bounds of map.
            let tree = unsafe { map.get_unchecked(x, y) };

            {
                let visible_by_left = (0..x)
                    .map(|x| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .all(|other| other < tree);
                if visible_by_left {
                    visible_tree_count += 1;
                    continue;
                }
            }
            {
                let visible_by_right = (x + 1..map.width())
                    .map(|x| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .all(|other| other < tree);
                if visible_by_right {
                    visible_tree_count += 1;
                    continue;
                }
            }
            {
                let visible_by_top = (0..y)
                    .map(|y| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .all(|other| other < tree);
                if visible_by_top {
                    visible_tree_count += 1;
                    continue;
                }
            }
            {
                let visible_by_bottom = (y + 1..map.height())
                    .map(|y| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .all(|other| other < tree);
                if visible_by_bottom {
                    visible_tree_count += 1;
                    continue;
                }
            }
        }
    }

    Some(visible_tree_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from_str(input).unwrap();

    let max_score = (1..map.width() - 1)
        .cartesian_product(1..map.height() - 1)
        .map(|(x, y)| {
            // Safety: x and y are in bounds of map.
            let tree = unsafe { map.get_unchecked(x, y) };
            let mut score = 1;

            score *= {
                let range = (0..x).rev();
                let range_len = range.len();
                let visible_by_left = range
                    .map(|x| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .take_while(|other| other < &tree)
                    .count();
                if visible_by_left != range_len {
                    visible_by_left + 1
                } else {
                    visible_by_left
                }
            };
            score *= {
                let range = x + 1..map.width();
                let range_len = range.len();
                let visible_by_right = range
                    .map(|x| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .take_while(|other| other < &tree)
                    .count();
                if visible_by_right != range_len {
                    visible_by_right + 1
                } else {
                    visible_by_right
                }
            };
            score *= {
                let range = (0..y).rev();
                let range_len = range.len();
                let visible_by_top = range
                    .map(|y| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .take_while(|other| other < &tree)
                    .count();
                if visible_by_top != range_len {
                    visible_by_top + 1
                } else {
                    visible_by_top
                }
            };
            score *= {
                let range = y + 1..map.height();
                let range_len = range.len();
                let visible_by_bottom = range
                    .map(|y| {
                        // Safety: x and y are in bounds of map.
                        unsafe { map.get_unchecked(x, y) }
                    })
                    .take_while(|other| other < &tree)
                    .count();
                if visible_by_bottom != range_len {
                    visible_by_bottom + 1
                } else {
                    visible_by_bottom
                }
            };

            score
        })
        .max()
        .unwrap();

    Some(max_score)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 8);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
