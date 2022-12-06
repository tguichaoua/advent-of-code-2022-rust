use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    for (i, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Some(i + 4);
        }
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Option<usize> {
    for (i, x) in input.as_bytes().windows(14).enumerate() {
        if HashSet::<_>::from_iter(x).len() == 14 {
            return Some(14 + i);
        }
    }
    unreachable!()
}

fn main() {
    let input = &adventofcode::read_file("inputs", 6);
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
        let input = adventofcode::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
