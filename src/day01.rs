use itertools::Itertools;
use serde_scan::scan;

fn find_sum(input: &[i32], n: usize) -> Vec<i32> {
    input
        .iter()
        .copied()
        .combinations(n)
        .find(|x| x.iter().sum::<i32>() == 2020)
        .expect("There should be a valid combination")
}

pub fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| scan!("{}" <- l).unwrap()).collect()
}

#[allow(clippy::ptr_arg)]
pub fn part_1(input: &Vec<i32>) -> i32 {
    let sums = find_sum(input, 2);
    sums.iter().product::<i32>()
}

#[allow(clippy::ptr_arg)]
pub fn part_2(input: &Vec<i32>) -> i32 {
    let sums = find_sum(input, 3);
    sums.iter().product::<i32>()
}

#[cfg(test)]
mod tests {
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
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 514579);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 241861950);
    }
}
