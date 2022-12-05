use std::str::FromStr;

use adventofcode::helpers::GetMut;
use itertools::Itertools;
use tuple::Map;

#[derive(Debug, Default, Clone)]
struct Stack {
    crates: Vec<u8>,
}

impl Stack {
    fn add_one(&mut self, crate_: u8) {
        self.crates.push(crate_)
    }

    fn add(&mut self, crates: impl IntoIterator<Item = u8>) {
        self.crates.extend(crates);
    }

    fn take(&mut self, amount: usize) -> std::vec::Drain<u8> {
        self.crates.drain(self.crates.len() - amount..)
    }

    fn top(&self) -> u8 {
        *self.crates.last().unwrap()
    }
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Stack>, impl Iterator<Item = Instruction> + '_) {
    let mut lines = input.lines();

    let crate_lines = lines
        .by_ref()
        // The crate and instruction sections are separated by an empty line
        .take_while(|line| !line.is_empty())
        // The last line of crate section is the index of the stacks
        // this is useless, so we drop this line.
        .filter(|line| line.as_bytes()[1] != b'1')
        .map(|line| {
            line.as_bytes()
                .iter()
                .copied()
                .skip(1)
                .step_by(4)
                .map(|letter| (letter != b' ').then_some(letter))
        })
        // Collect to a vec to be able to iterate from the last element
        .collect_vec();

    let mut stacks = {
        let stack_count = crate_lines.first().unwrap().len();
        vec![Stack::default(); stack_count]
    };

    for line in crate_lines.into_iter().rev() {
        for (i, letter) in line.enumerate() {
            if let Some(letter) = letter {
                stacks[i].add_one(letter);
            }
        }
    }

    let instructions = lines.map(|line| {
        let (amount, from, to) = line
            .split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .collect_tuple::<(_, _, _)>()
            .unwrap()
            .map(|x| FromStr::from_str(x).unwrap());
        Instruction {
            amount,
            // in input, `from` and `to` are base 1
            from: from - 1,
            to: to - 1,
        }
    });

    (stacks, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, instructions) = parse_input(input);
    for Instruction { amount, from, to } in instructions {
        let (from, to) = stacks.get_two_mut(from, to);
        to.add(from.take(amount).rev());
    }
    Some(String::from_iter(
        stacks.into_iter().map(|stack| stack.top() as char),
    ))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, instructions) = parse_input(input);
    for Instruction { amount, from, to } in instructions {
        let (from, to) = stacks.get_two_mut(from, to);
        to.add(from.take(amount));
    }
    Some(String::from_iter(
        stacks.into_iter().map(|stack| stack.top() as char),
    ))
}

fn main() {
    let input = &adventofcode::read_file("inputs", 5);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
