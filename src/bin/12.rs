use std::collections::{HashSet, VecDeque};

use adventofcode::helpers::{grid_neighbors, Grid, GridIndex};
use itertools::Itertools;

fn parse_input(input: &str) -> (Grid<u8>, GridIndex, GridIndex) {
    let width = input.lines().next().unwrap().len();

    let mut start = None;
    let mut end = None;

    let heightmap = input
        .lines()
        .flat_map(|line| line.bytes())
        .enumerate()
        .map(|(i, elt)| match elt {
            b'S' => {
                start = Some(i);
                0
            }
            b'E' => {
                end = Some(i);
                b'z' - b'a'
            }
            elt => elt - b'a',
        })
        .collect_vec();
    let heightmap = Grid::new_from_width(heightmap, width).unwrap();
    let start = heightmap.unflat_index(start.unwrap());
    let end = heightmap.unflat_index(end.unwrap());

    (heightmap, start, end)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (heightmap, start, end) = parse_input(input);

    let mut deque = VecDeque::with_capacity(1);
    deque.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((current, distance)) = deque.pop_front() {
        let highness = *heightmap.get(current).unwrap();
        let neighbors = grid_neighbors(current, heightmap.width(), heightmap.height())
            .into_iter()
            .flatten()
            .filter(|&n| {
                let n = *heightmap.get(n).unwrap();
                n <= highness + 1
            });

        for n in neighbors {
            if n == end {
                return Some(distance + 1);
            }
            if visited.contains(&n) {
                continue;
            }
            visited.insert(n);
            deque.push_back((n, distance + 1));
        }
    }

    panic!("no solution");
}

pub fn part_two(input: &str) -> Option<usize> {
    let (heightmap, _, end) = parse_input(input);

    let mut deque = VecDeque::with_capacity(1);
    deque.push_back((end, 0));

    let mut visited = HashSet::new();
    visited.insert(end);

    while let Some((current, distance)) = deque.pop_front() {
        let highness = *heightmap.get(current).unwrap();
        let neighbors = grid_neighbors(current, heightmap.width(), heightmap.height())
            .into_iter()
            .flatten()
            .map(|index| (index, *heightmap.get(index).unwrap()))
            .filter(|&(_, n)| highness == n + 1 || n >= highness);

        for (index, highness) in neighbors {
            if highness == 0 {
                return Some(distance + 1);
            }
            if visited.contains(&index) {
                continue;
            }
            visited.insert(index);
            deque.push_back((index, distance + 1));
        }
    }

    panic!("no solution");
}

fn main() {
    let input = &adventofcode::read_file("inputs", 12);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
