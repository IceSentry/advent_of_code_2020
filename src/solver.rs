use anyhow::{Context, Result};
use std::{
    error::Error,
    fmt::{Debug, Display},
    str::FromStr,
};

pub fn parse_line<F: FromStr>(line: &str) -> Result<F>
where
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    line.parse::<F>().context(format!("cannot parse {}", line))
}

pub trait Solver {
    type Input;
    type Output: Display + Debug;

    fn parse(input: &str) -> Result<Self::Input>;

    fn part1(input: &Self::Input) -> Self::Output;
    fn part2(input: &Self::Input) -> Option<Self::Output>;
}
