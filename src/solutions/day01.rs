use crate::solver::{parse_line, Solver};
use anyhow::Result;
use itertools::Itertools;

pub struct Day01 {}

fn find_sum(input: Vec<i32>, n: usize) -> Vec<i32> {
    input
        .into_iter()
        .combinations(n)
        .find(|x| x.iter().sum::<i32>() == 2020)
        .expect("There should be a valid combination")
}

impl Solver for Day01 {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse(input: &str) -> Result<Self::Input> {
        input.lines().map(|l| parse_line(l)).collect()
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let sums = find_sum(input.clone(), 2 as usize);
        sums.iter().product::<i32>()
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        let sums = find_sum(input.clone(), 3 as usize);
        Some(sums.iter().product::<i32>())
    }
}

#[cfg(test)]
mod tests {
    use super::Day01;
    use crate::solver::Solver;
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        1721
        979
        366
        299
        675
        1456
    "};

    #[test]
    fn part1() {
        let input = Day01::parse(INPUTS);
        let result = Day01::part1(&input.unwrap());
        assert!(result == 514579);
    }

    #[test]
    fn part2() {
        let input = Day01::parse(INPUTS);
        let result = Day01::part2(&input.unwrap());
        assert!(result.unwrap() == 241861950);
    }
}
