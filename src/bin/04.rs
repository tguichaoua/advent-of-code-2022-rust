use std::{ops::RangeInclusive, str::FromStr};

use tuple::Map;

fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    line.split_once(',').unwrap().map(|range| {
        let (start, end) = range
            .split_once('-')
            .unwrap()
            .map(|x| FromStr::from_str(x).unwrap());
        start..=end
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let overlap_count = input
        .lines()
        .map(|line| {
            let (a, b) = parse_line(line);
            if (a.contains(b.start()) && a.contains(b.end()))
                || (b.contains(a.start()) && b.contains(a.end()))
            {
                1
            } else {
                0
            }
        })
        .sum();

    Some(overlap_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let overlap_count = input
        .lines()
        .map(|line| {
            let (a, b) = parse_line(line);
            if a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
            {
                1
            } else {
                0
            }
        })
        .sum();

    Some(overlap_count)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 4);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
