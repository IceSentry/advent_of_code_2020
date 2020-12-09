use serde_scan::scan;

pub fn parse(input: &str) -> Vec<(i32, i32, char, String)> {
    input
        .lines()
        .map(|line| scan!("{}-{} {}:{}" <- line).unwrap())
        .collect()
}

pub fn part_1(input: &[(i32, i32, char, String)]) -> usize {
    input
        .iter()
        .filter(|(lowest, highest, letter, password)| {
            let count = password.matches(*letter).count() as i32;
            count >= *lowest && count <= *highest
        })
        .count()
}

pub fn part_2(input: &[(i32, i32, char, String)]) -> usize {
    input
        .iter()
        .filter(|(lowest, highest, letter, password)| {
            let mut chars = password.chars();
            let a = chars.nth(*lowest as usize - 1);
            let b = chars.nth(*highest as usize - *lowest as usize - 2);
            (a == Some(*letter)) != (b == Some(*letter))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
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
