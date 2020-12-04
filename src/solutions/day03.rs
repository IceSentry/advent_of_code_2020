use crate::solver::Solver;
use anyhow::Result;

fn count_trees(input: &Vec<Vec<char>>, slope: (usize, usize)) -> i64 {
    let mut x = 0;
    let mut count = 0;

    for line in input.iter().step_by(slope.1) {
        if line[x % line.len()] == '#' {
            count += 1;
        }
        x += slope.0;
    }
    count
}

pub struct Day03;

impl Solver for Day03 {
    type Input = Vec<Vec<char>>;
    type Output = i64;

    fn parse(input: &str) -> Result<Self::Input> {
        Ok(input.lines().map(|l| l.chars().collect()).collect())
    }

    fn part1(input: &Self::Input) -> Self::Output {
        count_trees(input, (3, 1))
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        Some(
            slopes
                .iter()
                .map(|s| count_trees(input, *s))
                .product::<i64>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Day03;
    use crate::solver::Solver;
    use indoc::indoc;

    #[test]
    fn part1() {
        let inputs = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "};

        let input = Day03::parse(inputs);
        let result = Day03::part1(&input.unwrap());
        println!("result: {}", result);
        assert!(result == 7);
    }
}
