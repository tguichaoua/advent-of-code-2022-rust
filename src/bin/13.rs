use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Item {
    Number(u32),
    List(Box<[Item]>),
}

mod parser {
    use std::str::FromStr;

    use super::Item;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, one_of},
        combinator::{map, map_res, recognize},
        multi::{many0, many1, separated_list0},
        sequence::{delimited, terminated},
        IResult,
    };

    pub fn item(input: &str) -> IResult<&str, Item> {
        alt((map(item_list, Item::List), map(item_number, Item::Number)))(input)
    }

    pub fn item_number(input: &str) -> IResult<&str, u32> {
        map_res(
            recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
            FromStr::from_str,
        )(input)
    }

    pub fn item_list(input: &str) -> IResult<&str, Box<[Item]>> {
        map(
            delimited(tag("["), separated_list0(tag(","), item), tag("]")),
            |items| items.into_boxed_slice(),
        )(input)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Item> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parser::item)
        .map(|x| x.unwrap().1)
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn cmp_list(a: &[Item], b: &[Item]) -> std::cmp::Ordering {
            if let Some(cmp) =
                a.iter().zip(b).map(|(a, b)| Ord::cmp(a, b)).find(|cmp| {
                    matches!(cmp, std::cmp::Ordering::Less | std::cmp::Ordering::Greater)
                })
            {
                cmp
            } else {
                Ord::cmp(&a.len(), &b.len())
            }
        }

        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.cmp(b),
            (a @ Item::Number(_), Item::List(b)) => cmp_list(std::slice::from_ref(a), b),
            (Item::List(a), b @ Item::Number(_)) => cmp_list(a, std::slice::from_ref(b)),
            (Item::List(a), Item::List(b)) => cmp_list(a, b),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let count = parse_input(input)
        .tuples()
        .map(|(a, b)| Ord::cmp(&a, &b))
        .enumerate()
        .filter(|(_, cmp)| matches!(cmp, std::cmp::Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum();

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let a = Item::List(vec![Item::List(vec![Item::Number(2)].into())].into());
    let b = Item::List(vec![Item::List(vec![Item::Number(6)].into())].into());

    let items = parse_input(input).chain([a.clone(), b.clone()]).sorted();

    let mut pos_a = None;
    let mut pos_b = None;

    for (i, item) in items.enumerate() {
        if pos_a.is_none() && a == item {
            pos_a = Some(i + 1);
        }
        if pos_b.is_none() && b == item {
            pos_b = Some(i + 1);
        }
        if pos_a.is_some() && pos_b.is_some() {
            break;
        }
    }

    Some(pos_a.unwrap() * pos_b.unwrap())
}

fn main() {
    let input = &adventofcode::read_file("inputs", 13);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
