#![allow(clippy::identity_op)] // for the sake of consistency ;)

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|play| match play {
            ("A", "X") => 3 + 1,
            ("A", "Y") => 6 + 2,
            ("A", "Z") => 0 + 3,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 6 + 1,
            ("C", "Y") => 0 + 2,
            ("C", "Z") => 3 + 3,
            _ => unreachable!(),
        })
        .sum();
    Some(score)
}

pub fn part_one_2(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| match line {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => unreachable!(),
        })
        .sum();
    Some(score)
}

pub fn part_one_3(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| (line.as_bytes()[0] as char, line.as_bytes()[2] as char))
        .map(|line| match line {
            ('A', 'X') => 3 + 1,
            ('A', 'Y') => 6 + 2,
            ('A', 'Z') => 0 + 3,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 6 + 1,
            ('C', 'Y') => 0 + 2,
            ('C', 'Z') => 3 + 3,
            _ => unreachable!(),
        })
        .sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|play| match play {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            _ => unreachable!(),
        })
        .sum();
    Some(score)
}

pub fn part_two_2(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent, outcome)| {
            let opponent_sign_score = match opponent {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => unreachable!(),
            };
            match outcome {
                "X" => 0 + 1 + (opponent_sign_score + 2) % 3,
                "Y" => 3 + 1 + opponent_sign_score,
                "Z" => 6 + 1 + (opponent_sign_score + 1) % 3,
                _ => unreachable!(),
            }
        })
        .sum();
    Some(score)
}

pub fn part_two_3(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| (line.as_bytes()[0] as char, line.as_bytes()[2] as char))
        .map(|(opponent, outcome)| {
            let opponent_sign_score = match opponent {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => unreachable!(),
            };
            match outcome {
                'X' => 0 + 1 + (opponent_sign_score + 2) % 3,
                'Y' => 3 + 1 + opponent_sign_score,
                'Z' => 6 + 1 + (opponent_sign_score + 1) % 3,
                _ => unreachable!(),
            }
        })
        .sum();
    Some(score)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 2);
    // adventofcode::solve!(1, part_one, input);
    // adventofcode::solve!(1, part_one_2, input);
    adventofcode::solve!(1, part_one_3, input);
    // adventofcode::solve!(2, part_two, input);
    // adventofcode::solve!(2, part_two_2, input);
    adventofcode::solve!(2, part_two_3, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_one_2() {
        let input = adventofcode::read_file("examples", 2);
        assert_eq!(part_one_2(&input), Some(15));
    }

    #[test]
    fn test_part_one_3() {
        let input = adventofcode::read_file("examples", 2);
        assert_eq!(part_one_3(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_part_two_2() {
        let input = adventofcode::read_file("examples", 2);
        assert_eq!(part_two_2(&input), Some(12));
    }

    #[test]
    fn test_part_two_3() {
        let input = adventofcode::read_file("examples", 2);
        assert_eq!(part_two_3(&input), Some(12));
    }
}
