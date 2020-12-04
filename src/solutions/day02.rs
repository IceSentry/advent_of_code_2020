use crate::solver::Solver;
use anyhow::Context;
use anyhow::Result;
use regex::Regex;
use serde_scan::scan;

#[allow(dead_code)]
fn parse_regex(line: &str) -> Result<(i32, i32, char, String)> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]*)")?;
    let captures = re.captures(line).context("failed to match")?;

    Ok((
        captures[1].parse()?,
        captures[2].parse()?,
        captures[3].parse()?,
        captures[4].parse()?,
    ))
}

pub struct Day02 {}

impl Solver for Day02 {
    type Input = Vec<(i32, i32, char, String)>;
    type Output = usize;

    fn parse(input: &str) -> Result<Self::Input> {
        input
            .lines()
            .map(|line| Ok(scan!("{}-{} {}:{}" <- line)?))
            .collect()
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|(lowest, highest, letter, password)| {
                let count = password.matches(*letter).count() as i32;
                count >= *lowest && count <= *highest
            })
            .count()
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .filter(|(lowest, highest, letter, password)| {
                    let mut chars = password.chars();
                    let a = chars.nth(*lowest as usize - 1);
                    let b = chars.nth(*highest as usize - *lowest as usize - 2);
                    (a == Some(*letter)) != (b == Some(*letter))
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

    const INPUTS: &str = indoc! {"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "};

    #[test]
    fn part1() {
        let input = Day02::parse(INPUTS);
        let result = Day02::part1(&input.unwrap());
        assert!(result == 2);
    }

    #[test]
    fn part2() {
        let input = Day02::parse(INPUTS);
        let result = Day02::part2(&input.unwrap());
        assert!(result.unwrap() == 1);
    }
}
