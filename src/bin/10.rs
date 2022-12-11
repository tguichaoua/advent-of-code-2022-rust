use std::str::FromStr;

enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_intstruction(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| match line.as_bytes()[0] {
        b'n' => Instruction::Noop,
        b'a' => {
            let value = FromStr::from_str(&line[5..]).unwrap();
            Instruction::Addx(value)
        }
        _ => unreachable!(),
    })
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = parse_intstruction(input);
    let mut signal_strengths = Vec::with_capacity(6);
    let mut x = 1;
    let mut cycle = 1;

    macro_rules! increment_cycle {
        () => {{
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                signal_strengths.push(cycle * x);
            }
        }};
    }

    for inst in instructions {
        match inst {
            Instruction::Noop => increment_cycle!(),
            Instruction::Addx(value) => {
                increment_cycle!();
                x += value;
                increment_cycle!();
            }
        }
    }

    debug_assert_eq!(signal_strengths.len(), 6);

    Some(signal_strengths.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<String> {
    const SCREEN_PIXELS_COUNT: usize = 40 * 6;

    let instructions = parse_intstruction(input);
    let mut x = 1;
    let mut cycle = 0;

    let mut screen = String::with_capacity(SCREEN_PIXELS_COUNT);
    macro_rules! increment_cycle {
        () => {{
            let pixel = if (x - 1..=x + 1).contains(&(cycle % 40)) {
                '#'
            } else {
                '.'
            };
            screen.push(pixel);
            cycle += 1;
        }};
    }

    for inst in instructions {
        match inst {
            Instruction::Noop => increment_cycle!(),
            Instruction::Addx(value) => {
                increment_cycle!();
                increment_cycle!();
                x += value;
            }
        }
    }

    debug_assert_eq!(screen.len(), SCREEN_PIXELS_COUNT);

    screen.reserve(5);

    for i in 0..5 {
        screen.insert((i + 1) * 40 + i, '\n');
    }

    Some(screen)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 10);
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 10);
        assert_eq!(
            part_two(&input).as_deref(),
            Some(
                r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
            )
        );
    }
}
