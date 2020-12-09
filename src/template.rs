use serde_scan::scan;

type Data = i32;

pub fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| scan!("{}" <- line).unwrap())
        .collect()
}

pub fn part_1(input: &[Data]) -> usize {
    0
}

pub fn part_2(input: &[Data]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert!(result == 2);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert!(result == 1);
    }
}
