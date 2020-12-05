use crate::solver::Solver;
use anyhow::Result;

pub struct Day0X {}
impl Solver for Day0X {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse(input: &str) -> Result<Self::Input> {
        input.lines().map(|l| todo!()).collect()
    }

    fn part1(input: &Self::Input) -> Self::Output {
        todo!()
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Day0X;
    use crate::solver::Solver;
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
    "};

    #[test]
    fn part1() {
        let input = Day0X::parse(INPUTS);
        let result = Day0X::part1(&input.unwrap());
        assert!(result == 0);
    }

    #[test]
    fn part2() {
        let input = Day0X::parse(INPUTS);
        let result = Day0X::part2(&input.unwrap());
        assert!(result.unwrap() == 0);
    }
}
