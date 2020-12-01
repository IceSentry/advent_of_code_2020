use anyhow::Result;
use itertools::Itertools;

use crate::solver::{parse_line, Solver};

pub struct Day01 {}

impl Solver for Day01 {
    type Input = Vec<i32>;

    type Output = i32;

    fn parse_input(&self, input: String) -> Result<Self::Input> {
        input.lines().map(|l| parse_line(l)).collect()
    }

    fn solve_first(&self, input: &Vec<i32>) -> Self::Output {
        for (i, val1) in input.iter().enumerate() {
            for i2 in i..input.len() {
                let val2 = input.get(i2).unwrap();
                if val1 + val2 == 2020 {
                    return val1 * val2;
                }
            }
        }
        unreachable!()
    }

    fn solve_second(&self, input: &Vec<i32>) -> Self::Output {
        for c in input.into_iter().combinations(3) {
            match *c.as_slice() {
                [val1, val2, val3] => {
                    if val1 + val2 + val3 == 2020 {
                        println!("{:?}", c);

                        return val1 * val2 * val3;
                    }
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}
