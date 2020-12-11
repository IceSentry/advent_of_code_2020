use std::collections::{HashMap, HashSet};

type Data = HashMap<String, Vec<(u32, String)>>;

pub fn parse<'a>(input: &'a str) -> Data {
    // TODO regex would probably be much cleaner here
    input
        .lines()
        .map(
            |line| match *line.split(" bags contain ").collect::<Vec<_>>().as_slice() {
                [container, bags] => (
                    container.to_string(),
                    bags.split(',')
                        .map(|s| {
                            s.trim()
                                .replace("bags", "")
                                .replace("bag", "")
                                .trim_end_matches('.')
                                .trim()
                                .to_string()
                        })
                        .filter(|bag| bag != "no other")
                        .map(|bag| {
                            let mut chars = bag.chars();
                            (
                                chars.nth(0).unwrap().to_digit(10).unwrap(),
                                chars.as_str().trim().to_string(),
                            )
                        })
                        .collect(),
                ),
                _ => unreachable!(),
            },
        )
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    fn contains_bag(curr: &str, target: &str, data: &Data) -> bool {
        curr == target
            || data[curr]
                .iter()
                .any(|(_qty, color)| contains_bag(color, target, data))
    }

    input
        .keys()
        .filter(|bag| contains_bag(bag, "shiny gold", input))
        .count()
        - 1
}

pub fn part_1_cache(input: &Data) -> usize {
    fn contains_bag<'a>(curr: &'a str, data: &'a Data, cache: &mut HashSet<&'a str>) -> bool {
        if !cache.contains(curr) {
            let result = data[curr]
                .iter()
                .any(|(_qty, color)| contains_bag(color, data, cache));
            if result {
                cache.insert(curr);
            }
            result
        } else {
            true
        }
    }

    let mut cache = HashSet::new();
    cache.insert("shiny gold");
    input
        .keys()
        .filter(|bag| contains_bag(bag, input, &mut cache))
        .count()
        - 1
}

pub fn part_2(input: &Data) -> u32 {
    fn count_bags(container: &str, data: &Data) -> u32 {
        data[container]
            .iter()
            .map(|(qty, color)| *qty * count_bags(color, data))
            .sum::<u32>()
            + 1
    }

    count_bags("shiny gold", input) - 1
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    pub fn part_1() {
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

        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 4);
    }

    #[test]
    pub fn part_2() {
        const INPUTS: &str = indoc! {"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        "};

        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 126);
    }
}
