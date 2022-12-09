#![feature(inline_const)]

use std::collections::HashSet;

use adventofcode::helpers::StrExt;
use byte_set::ByteSet;
use itertools::Itertools;

#[inline]
fn item_priority(item: u8) -> u32 {
    debug_assert!((b'A'..=b'Z').contains(&item) || (b'a'..=b'z').contains(&item));
    let value = if item < b'a' {
        item - const { b'A' - 27 }
    } else {
        item - const { b'a' - 1 }
    };
    value as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let value = input
        .lines()
        .map(|line| {
            let item_count = line.len();
            debug_assert_eq!(item_count % 2, 0);
            let (a, b) = line.split_at(item_count / 2);
            debug_assert_eq!(a.len(), b.len());
            let a = HashSet::<_>::from_iter(a.as_bytes());
            let b = HashSet::from_iter(b.as_bytes());

            let item = a.intersection(&b).exactly_one().unwrap();

            item_priority(**item)
        })
        .sum();

    Some(value)
}

pub fn part_one_2(input: &str) -> Option<u32> {
    let value = input
        .lines()
        .map(|line| {
            let item_count = line.len();
            debug_assert_eq!(item_count % 2, 0);
            let mid = item_count / 2;
            // SAFETY: `mid` is on a char boundary : the input only contains
            // 1 byte length character ('a' through 'z' and 'A' through 'Z');
            let (a, b) = unsafe { line.split_at_unchecked(mid) };
            debug_assert_eq!(a.len(), b.len());
            let a = HashSet::<_>::from_iter(a.as_bytes());
            let b = HashSet::from_iter(b.as_bytes());

            let item = a.intersection(&b).exactly_one().unwrap();

            item_priority(**item)
        })
        .sum();

    Some(value)
}

pub fn part_one_3(input: &str) -> Option<u32> {
    let value = input
        .lines()
        .map(|line| {
            let item_count = line.len();
            debug_assert_eq!(item_count % 2, 0);
            let mid = item_count / 2;
            // SAFETY: `mid` is on a char boundary : the input only contains
            // 1 byte length character ('a' through 'z' and 'A' through 'Z');
            let (a, b) = unsafe { line.split_at_unchecked(mid) };
            debug_assert_eq!(a.len(), b.len());
            let a = HashSet::<_>::from_iter(a.as_bytes().iter().copied());
            let b = HashSet::from_iter(b.as_bytes().iter().copied());

            let item = a.intersection(&b).exactly_one().unwrap();

            item_priority(*item)
        })
        .sum();

    Some(value)
}

pub fn part_one_4(input: &str) -> Option<u32> {
    let value = input
        .lines()
        .map(|line| {
            let item_count = line.len();
            debug_assert_eq!(item_count % 2, 0);
            let mid = item_count / 2;
            // SAFETY: `mid` is on a char boundary : the input only contains
            // 1 byte length character ('a' through 'z' and 'A' through 'Z');
            let (a, b) = unsafe { line.split_at_unchecked(mid) };
            debug_assert_eq!(a.len(), b.len());

            let a = ByteSet::from_iter(a.bytes());
            let b = ByteSet::from_iter(b.bytes());

            let intersection = a.intersection(b);
            debug_assert_eq!(intersection.len(), 1);
            let item = intersection.first().unwrap();

            item_priority(item)
        })
        .sum();

    Some(value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let value = input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let a = HashSet::<_>::from_iter(a.as_bytes());
            let b = HashSet::from_iter(b.as_bytes());
            let c = HashSet::from_iter(c.as_bytes());

            let a_b_common = a.intersection(&b).copied().collect::<HashSet<_>>();
            let common_item = a_b_common.intersection(&c).exactly_one().unwrap();

            item_priority(**common_item)
        })
        .sum();

    Some(value)
}

pub fn part_two_2(input: &str) -> Option<u32> {
    let value = input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let a = HashSet::<_>::from_iter(a.as_bytes().iter().copied());
            let b = HashSet::from_iter(b.as_bytes().iter().copied());
            let c = HashSet::from_iter(c.as_bytes().iter().copied());

            let a_b_common = a.intersection(&b).copied().collect::<HashSet<_>>();
            let common_item = a_b_common.intersection(&c).exactly_one().unwrap();

            item_priority(*common_item)
        })
        .sum();

    Some(value)
}

pub fn part_two_3(input: &str) -> Option<u32> {
    let value = input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let a = ByteSet::from_iter(a.bytes());
            let b = ByteSet::from_iter(b.bytes());
            let c = ByteSet::from_iter(c.bytes());

            let common_item = a.intersection(b).intersection(c);
            debug_assert_eq!(common_item.len(), 1);
            let item = common_item.first().unwrap();

            item_priority(item)
        })
        .sum();

    Some(value)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 3);
    // {
    //     // HashSet does some stuff when used for the first time
    //     // that took ~300 µs (on my machine).
    //     // Create a HashSet here to not impact the solutions performance.
    //     let mut dummy = HashSet::with_capacity(1);
    //     dummy.insert(0u8);
    // }
    // adventofcode::solve!(1, part_one, input);
    // adventofcode::solve!(1, part_one_2, input);
    // adventofcode::solve!(1, part_one_3, input);
    adventofcode::solve!(1, part_one_4, input);
    // adventofcode::solve!(2, part_two, input);
    // adventofcode::solve!(2, part_two_2, input);
    adventofcode::solve!(2, part_two_3, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_one_2() {
        let input = adventofcode::read_file("examples", 3);
        assert_eq!(part_one_2(&input), Some(157));
    }

    #[test]
    fn test_part_one_3() {
        let input = adventofcode::read_file("examples", 3);
        assert_eq!(part_one_3(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_part_two_2() {
        let input = adventofcode::read_file("examples", 3);
        assert_eq!(part_two_2(&input), Some(70));
    }
}
