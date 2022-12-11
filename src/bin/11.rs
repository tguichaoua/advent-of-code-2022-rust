use std::{cmp::Reverse, mem::transmute, ops::Mul, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct Item(u64);

enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    const STARTING_ITEMS: &'static str = "  Starting items: ";
    const OPERATION: &'static str = "  Operation: new = old ";
    const TEST: &'static str = "  Test: divisible by ";
    const IF_TRUE: &'static str = "    If true: throw to monkey ";
    const IF_FALSE: &'static str = "    If false: throw to monkey ";

    input
        .lines()
        .filter(|line| !line.is_empty())
        .tuples()
        .map(|(_, starting_items, operation, test, if_true, if_false)| {
            let items = starting_items[STARTING_ITEMS.len()..]
                .split(", ")
                .map(FromStr::from_str)
                .map(Result::unwrap)
                .map(Item)
                .collect_vec();

            let operation = {
                let operator = operation.as_bytes()[OPERATION.len()];
                let rhs = &operation[OPERATION.len() + 2..];
                match FromStr::from_str(rhs) {
                    Ok(value) => match operator {
                        b'+' => Operation::Add(value),
                        b'*' => Operation::Mul(value),
                        _ => unreachable!(),
                    },
                    Err(_) => Operation::Square,
                }
            };

            let test = FromStr::from_str(&test[TEST.len()..]).unwrap();
            let if_true = FromStr::from_str(&if_true[IF_TRUE.len()..]).unwrap();
            let if_false = FromStr::from_str(&if_false[IF_FALSE.len()..]).unwrap();

            Monkey {
                items,
                operation,
                test,
                if_true,
                if_false,
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);
    let mut inspect_counts = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // Safety: `i` is in bounds of `monkeys`.
            // Safety: We garentees the reference is unique.
            let monkey: &mut Monkey = unsafe { transmute(monkeys.get_unchecked_mut(i)) };

            // Safety: `i` is in bounds of `inspect_counts`.
            *unsafe { inspect_counts.get_unchecked_mut(i) } += monkey.items.len();

            for item in monkey.items.drain(..) {
                let item = match monkey.operation {
                    Operation::Add(x) => item.0 + x,
                    Operation::Mul(x) => item.0 * x,
                    Operation::Square => item.0 * item.0,
                };
                let item = item / 3;
                let recipient = if item % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                // Safety: `recipient` is in bounds of monkeys (otherwise `input` is invalid).
                let recipient = unsafe { monkeys.get_unchecked_mut(recipient) };
                recipient.items.push(Item(item));
            }
        }
    }

    inspect_counts.sort_unstable_by_key(|&num| Reverse(num));

    // Safety: there is at least 2 monkeys.
    let business = unsafe { inspect_counts.get_unchecked(0) * inspect_counts.get_unchecked(1) };
    Some(business)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);
    let worry_max = monkeys
        .iter()
        .map(|monkey| monkey.test)
        .reduce(Mul::mul)
        .unwrap();
    let mut inspect_counts = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            // Safety: `i` is in bounds of `monkeys`.
            // Safety: We garentees the reference is unique.
            let monkey: &mut Monkey = unsafe { transmute(monkeys.get_unchecked_mut(i)) };

            // Safety: `i` is in bounds of `inspect_counts`.
            *unsafe { inspect_counts.get_unchecked_mut(i) } += monkey.items.len();

            for item in monkey.items.drain(..) {
                let item = match monkey.operation {
                    Operation::Add(x) => item.0 + x,
                    Operation::Mul(x) => item.0 * x,
                    Operation::Square => item.0 * item.0,
                };
                let item = item % worry_max;
                let recipient = if item % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                // Safety: `recipient` is in bounds of monkeys (otherwise `input` is invalid).
                let recipient = unsafe { monkeys.get_unchecked_mut(recipient) };
                recipient.items.push(Item(item));
            }
        }
    }

    inspect_counts.sort_unstable_by_key(|&num| Reverse(num));

    // Safety: there is at least 2 monkeys.
    let business = unsafe { inspect_counts.get_unchecked(0) * inspect_counts.get_unchecked(1) };
    Some(business)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 11);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
