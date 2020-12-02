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
    type Input = Vec<(PasswordPolicy, String)>;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Result<Self::Input> {
        input
            .lines()
            .map(|l| {
                let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]*)")?;
                let captures = re.captures(l).with_context(|| "failed to match")?;

                Ok((
                    PasswordPolicy {
                        lowest: captures[1].parse()?,
                        highest: captures[2].parse()?,
                        letter: captures[3].parse()?,
                    },
                    captures[4].parse::<String>()?,
                ))
            })
            .collect()
    }

    fn solve_part1(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|(policy, password)| {
                let count = password.matches(policy.letter).count() as i32;
                count >= policy.lowest && count <= policy.highest
            })
            .count()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|(policy, password)| {
                let mut chars = password.chars();
                match (
                    chars.nth(policy.lowest as usize - 1),
                    chars.nth(policy.highest as usize - policy.lowest as usize - 1),
                ) {
                    (Some(a), Some(b)) => a != b && (a == policy.letter || b == policy.letter),
                    _ => false,
                }
            })
            .count()
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
        println!("result: {}", result);
        assert!(result == 1);
    }
}
