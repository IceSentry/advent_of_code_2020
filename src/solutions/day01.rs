use anyhow::Result;
use itertools::Itertools;

use crate::solver::{parse_line, Solver};

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

    fn parse_input(&self, input: &str) -> Result<Self::Input> {
        input.lines().map(|l| parse_line(l)).collect()
    }

    fn solve_part1(&self, input: &Self::Input) -> Self::Output {
        let sums = find_sum(input.clone(), 2 as usize);
        sums.iter().product::<i32>()
    }

    fn solve_part2(&self, input: &Self::Input) -> Option<Self::Output> {
        let sums = find_sum(input.clone(), 3 as usize);
        Some(sums.iter().product::<i32>())
    }
}
