use crate::solver::Solver;
use anyhow::Result;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Eq, Debug, Copy, Clone)]
pub struct Seat {
    row: u8,
    col: u8,
    id: i32,
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

pub struct Day05 {}
impl Solver for Day05 {
    type Input = Vec<Seat>;
    type Output = i32;

    fn parse(input: &str) -> Result<Self::Input> {
        let mut vec: Self::Input = input
            .lines()
            .map(|l| {
                let row = &l[..7].replace('F', "0").replace('B', "1");
                let col = &l[7..].replace('L', "0").replace('R', "1");
                let row = u8::from_str_radix(row, 2).unwrap();
                let col = u8::from_str_radix(col, 2).unwrap();
                Seat {
                    row,
                    col,
                    id: row as i32 * 8 + col as i32,
                }
            })
            .collect();
        vec.sort();
        Ok(vec)
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().max().unwrap().id
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        let mut it = input.iter().peekable();
        while let Some(seat) = it.next() {
            if it.peek().unwrap().id != seat.id + 1 {
                return Some(seat.id + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Day05;
    use crate::solver::Solver;
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        FBFBBFFRLR
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL
    "};

    const FILE_INPUTS: &str = include_str!("../../inputs/2020/05.txt");

    #[test]
    fn part1() {
        let input = Day05::parse(INPUTS);
        println!("{:#?}", input);

        let result = Day05::part1(&input.unwrap());
        assert_eq!(result, 820);
    }

    #[test]
    fn file() {
        let input = Day05::parse(FILE_INPUTS).unwrap();
        let result = Day05::part1(&input);
        assert_eq!(result, 953);

        let result = Day05::part2(&input);
        assert_eq!(result, Some(615));
    }
}
