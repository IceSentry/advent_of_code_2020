use anyhow::Context;
use anyhow::Result;
use regex::Regex;

use crate::solver::Solver;

#[derive(Debug)]
pub struct PasswordPolicy {
    lowest: i32,
    highest: i32,
    letter: char,
}

pub struct Day02 {}

impl Solver for Day02 {
    type Input = Vec<(i32, i32, char, String)>;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Result<Self::Input> {
        input
            .lines()
            .map(|l| {
                let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]*)")?;
                let captures = re.captures(l).with_context(|| "failed to match")?;

                Ok((
                    captures[1].parse()?,
                    captures[2].parse()?,
                    captures[3].parse()?,
                    captures[4].parse()?,
                ))
            })
            .collect()
    }

    fn solve_part1(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|(lowest, highest, letter, password)| {
                let count = password.matches(*letter).count() as i32;
                count >= *lowest && count <= *highest
            })
            .count()
    }

    fn solve_part2(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .filter(|(lowest, highest, letter, password)| {
                    match (
                        password.chars().nth(*lowest as usize - 1),
                        password.chars().nth(*highest as usize - 1),
                    ) {
                        (Some(a), Some(b)) => (a == *letter) != (b == *letter),
                        _ => false,
                    }
                })
                .count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Day02;
    use crate::solver::Solver;
    use indoc::indoc;

    #[test]
    fn part1() {
        let inputs = indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "};

        let input = Day02 {}.parse_input(inputs);
        let result = Day02 {}.solve_part1(&input.unwrap());
        println!("result: {}", result);
        assert!(result == 2);
    }

    #[test]
    fn part2() {
        let inputs = indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "};

        let input = Day02 {}.parse_input(inputs);
        let result = Day02 {}.solve_part2(&input.unwrap());
        assert!(result.unwrap() == 1);
    }
}
