use anyhow::Result;

use crate::solver::Solver;

fn count_trees(input: &Vec<Vec<bool>>, slope: (usize, usize)) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    loop {
        if input[y][x] {
            count += 1;
        }

        x += slope.0;
        x = x % input[y].len();

        y += slope.1;
        if y >= input.len() {
            break;
        }
    }
    count
}

pub struct Day03 {}

impl Solver for Day03 {
    type Input = Vec<Vec<bool>>;
    type Output = i64;

    fn parse_input(&self, input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| if c == '#' { true } else { false })
                    .collect()
            })
            .collect())
    }

    fn solve_part1(&self, input: &Self::Input) -> Self::Output {
        count_trees(input, (3, 1))
    }

    fn solve_part2(&self, input: &Self::Input) -> Option<Self::Output> {
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

        let input = Day03 {}.parse_input(inputs);
        let result = Day03 {}.solve_part1(&input.unwrap());
        println!("result: {}", result);
        assert!(result == 7);
    }
}
