use serde_scan::scan;

type Data = Vec<String>;

pub fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| line.split(" contain ").map(|s| s.to_string()).collect())
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
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
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
