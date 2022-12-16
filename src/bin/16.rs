use std::{
    collections::{BTreeSet, HashMap, HashSet},
    mem,
};

use adventofcode::helpers::{parser::decimal_value, IteratorExt};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

struct Valve {
    flow_rate: u32,
    neighbor_valves: Vec<String>,
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|line| {
            fn valve_name(input: &str) -> IResult<&str, String> {
                map(take(2usize), ToString::to_string)(input)
            }

            let (rest, name) = preceded(tag("Valve "), valve_name)(line).unwrap();
            let (rest, flow_rate) = preceded(tag(" has flow rate="), decimal_value)(rest).unwrap();
            let (_, neighbor_valves) = preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), valve_name),
            )(rest)
            .unwrap();

            (
                name,
                Valve {
                    flow_rate,
                    neighbor_valves,
                },
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = parse_input(input);

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct Universe {
        open_valves: BTreeSet<String>,
        released_pressure: u32,
        location: String,
        visited: BTreeSet<String>,
    }

    let mut universes = HashSet::with_capacity(1);
    universes.insert(Universe {
        // Mark all valves with 0 flow as already open.
        open_valves: valves
            .iter()
            .filter(|(_, valve)| valve.flow_rate == 0)
            .map(|(name, _)| name)
            .cloned()
            .collect(),
        released_pressure: 0,
        location: "AA".to_string(),
        visited: BTreeSet::new(),
    });

    let mut new_universes = HashSet::new();

    let mut max_released_pressure = 0;

    for time_left in (0..30).rev() {
        for universe in universes.drain() {
            let current_valve = valves.get(&universe.location).unwrap();
            let can_open_valve = !universe.open_valves.contains(&universe.location);

            if can_open_valve {
                let (total, count): (u32, _) = valves
                    .iter()
                    .filter(|(name, _)| !universe.open_valves.contains(*name))
                    .map(|(_, valve)| valve.flow_rate)
                    .sum_count();

                let mean = total / count as u32;

                if current_valve.flow_rate * 4 >= mean * 3 {
                    let released_pressure =
                        universe.released_pressure + time_left * current_valve.flow_rate;
                    if max_released_pressure < released_pressure {
                        max_released_pressure = released_pressure;
                    }

                    let all_open = universe.open_valves.len() + 1 == valves.len();

                    if !all_open {
                        let mut open_valves = universe.open_valves.clone();
                        open_valves.insert(universe.location.clone());
                        let location = universe.location.clone();
                        new_universes.insert(Universe {
                            open_valves,
                            released_pressure,
                            location,
                            visited: BTreeSet::new(),
                        });
                    } else {
                        continue;
                    }
                }
            }

            let moves = current_valve
                .neighbor_valves
                .iter()
                .filter(|&new_loc| !universe.visited.contains(new_loc))
                .cloned()
                .map(|new_location| {
                    let mut new_universe = universe.clone();
                    let old_location = mem::replace(&mut new_universe.location, new_location);
                    new_universe.visited.insert(old_location);
                    new_universe
                });

            new_universes.extend(moves);
        }

        mem::swap(&mut universes, &mut new_universes);
    }

    Some(max_released_pressure)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &adventofcode::read_file("inputs", 16);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
        // assert_eq!(part_two(&input), Some(1707));
    }
}
