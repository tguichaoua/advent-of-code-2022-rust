use std::mem::take;
use std::str::FromStr;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut elves = Vec::new();
    let mut current_elf = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(take(&mut current_elf));
        } else {
            let value = u32::from_str(line).expect("read value");
            current_elf.push(value);
        }
    }
    if !current_elf.is_empty() {
        elves.push(current_elf);
    }
    elves
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let calories_by_elf = input.into_iter().map(|elf| elf.into_iter().sum());
    Some(calories_by_elf.max().expect("calories is empty"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let mut calories_by_elf = input
        .into_iter()
        .map(|elf| elf.into_iter().sum::<u32>())
        .collect_vec();
    calories_by_elf.sort_unstable();
    Some(calories_by_elf.into_iter().rev().take(3).sum())
}

fn main() {
    let input = &adventofcode::read_file("inputs", 1);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
