use anyhow::{Context, Result};
use std::{error::Error, fmt::Display, str::FromStr};

pub fn parse_line<F: FromStr>(line: &str) -> Result<F>
where
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    line.parse::<F>()
        .with_context(|| format!("cannot parse {}", line))
}

pub trait Solver {
    type Input;
    type Output: Display;

    fn parse_input(&self, input: &str) -> Result<Self::Input>;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output;
    fn solve_part2(&self, input: &Self::Input) -> Self::Output;

    fn solve(&self, input: &str) {
        let input = self.parse_input(input).expect("failed to parse input");
        let first_result = self.solve_part1(&input);
        let secondd_result = self.solve_part2(&input);
        println!("part1: {}", first_result);
        println!("part2: {}", secondd_result);
    }
}
