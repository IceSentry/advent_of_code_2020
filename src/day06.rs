use std::collections::HashMap;

type FormGroup = (HashMap<char, usize>, usize);

pub fn parse(input: &str) -> Vec<FormGroup> {
    input
        .split("\n\n")
        .map(|group| {
            let mut map = HashMap::new();
            for s in group.split_whitespace() {
                for c in s.chars() {
                    *map.entry(c).or_insert(0) += 1;
                }
            }
            (map, group.lines().count())
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
pub fn part_1(input: &Vec<FormGroup>) -> usize {
    input.iter().map(|(group, _)| group.len()).sum()
}

#[allow(clippy::ptr_arg)]
pub fn part_2(input: &Vec<FormGroup>) -> usize {
    input
        .iter()
        .map(|(group, count)| group.iter().filter(|(_, v)| *v == count).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 6);
    }
}
