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

    fn parse_input(&self, input: String) -> Result<Self::Input>;

    fn solve_first(&self, input: &Self::Input) -> Self::Output;
    fn solve_second(&self, input: &Self::Input) -> Self::Output;

    fn solve(&self, input: String) {
        let input = self.parse_input(input).expect("failed to parse input");
        let first_result = self.solve_first(&input);
        let secondd_result = self.solve_second(&input);
        println!("{}", first_result);
        println!("{}", secondd_result);
    }
}
