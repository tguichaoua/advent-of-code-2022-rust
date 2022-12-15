use std::ops::Add;

use adventofcode::helpers::parser::decimal_value;
use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn distance_to(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    position: Pos,
    nearest_beacon: Pos,
}

fn pos(input: &str) -> IResult<&str, Pos> {
    map(
        preceded(
            tag("x="),
            separated_pair(decimal_value, tag(", y="), decimal_value),
        ),
        |(x, y)| Pos { x, y },
    )(input)
}

fn parse_input(input: &str) -> impl Iterator<Item = Sensor> + '_ {
    input.lines().map(move |line| {
        let (rest, position) = preceded(tag("Sensor at "), pos)(line).unwrap();
        let (_, nearest_beacon) = preceded(tag(": closest beacon is at "), pos)(rest).unwrap();
        Sensor {
            position,
            nearest_beacon,
        }
    })
}

pub fn part_one(input: &str, target_line: i32) -> Option<u32> {
    let sensors = parse_input(input).collect_vec();

    let x_range = {
        let MinMaxResult::MinMax(min, max) = sensors
        .iter()
        .flat_map(
            |Sensor {
                 position,
                 nearest_beacon,
             }| {
                let d = position.distance_to(nearest_beacon);
                [
                    position.x.checked_sub_unsigned(d).unwrap(),
                    position.x.checked_add_unsigned(d).unwrap()
                ]
            },
        )
        .minmax() else{ panic!("Expected two values") };
        min..=max
    };

    let beacons = sensors
        .iter()
        .map(|Sensor { nearest_beacon, .. }| nearest_beacon)
        .copied()
        .filter(|pos| pos.y == target_line)
        .collect_vec();

    let sensors = sensors
        .iter()
        .map(
            |Sensor {
                 position,
                 nearest_beacon,
             }| {
                let distance = position.distance_to(nearest_beacon);
                (*position, distance)
            },
        )
        .collect_vec();

    let mut beacon_cannot_be_here_count = 0;

    let y = target_line;
    for x in x_range {
        let pos = Pos { x, y };
        if beacons.contains(&pos) {
            continue;
        }
        for (sensor_pos, beacon_distance) in &sensors {
            let distance = sensor_pos.distance_to(&pos);
            if distance <= *beacon_distance {
                beacon_cannot_be_here_count += 1;
                break;
            }
        }
    }

    Some(beacon_cannot_be_here_count)
}

pub fn part_two(input: &str, max_coordinate: i32) -> Option<i64> {
    fn manhattan_circle(radius: u32) -> impl Iterator<Item = Pos> {
        [
            Pos {
                x: -(radius as i32),
                y: 0,
            },
            Pos {
                x: radius as i32,
                y: 0,
            },
            Pos {
                x: 0,
                y: -(radius as i32),
            },
            Pos {
                x: 0,
                y: radius as i32,
            },
        ]
        .into_iter()
        .chain((1..radius).flat_map(move |x| {
            let y = radius - x;

            [
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                Pos {
                    x: x as i32,
                    y: -(y as i32),
                },
                Pos {
                    x: -(x as i32),
                    y: y as i32,
                },
                Pos {
                    x: -(x as i32),
                    y: -(y as i32),
                },
            ]
        }))
    }

    let sensors = parse_input(input)
        .map(
            |Sensor {
                 position,
                 nearest_beacon,
             }| {
                let distance = position.distance_to(&nearest_beacon);
                (position, distance)
            },
        )
        .collect_vec();

    let coordinate_range = 0..=max_coordinate;
    let mut distress_beacon_pos = None;

    'outer: for &(sensor_pos, beacon_distance) in &sensors {
        let pois = manhattan_circle(beacon_distance + 1)
            .map(|p| p + sensor_pos)
            .filter(|Pos { x, y }| coordinate_range.contains(x) && coordinate_range.contains(y));
        'poi: for p in pois {
            for &(sensor_pos, beacon_distance) in &sensors {
                let distance = sensor_pos.distance_to(&p);
                if distance <= beacon_distance {
                    continue 'poi;
                }
            }
            distress_beacon_pos = Some(p);
            break 'outer;
        }
    }

    let Pos { x, y } = distress_beacon_pos.unwrap();

    Some((x as i64) * 4_000_000 + (y as i64))
}

fn main() {
    let input = &adventofcode::read_file("inputs", 15);

    let part_one = |input: &str| part_one(input, 2_000_000);
    let part_two = |input: &str| part_two(input, 4_000_000);

    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 15);
        assert_eq!(part_one(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 15);
        assert_eq!(part_two(&input, 20), Some(56000011));
    }
}
