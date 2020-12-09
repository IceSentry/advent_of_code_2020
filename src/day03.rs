fn count_trees(input: &[Vec<char>], slope: (usize, usize)) -> i64 {
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

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_1(input: &[Vec<char>]) -> i64 {
    count_trees(input, (3, 1))
}

pub fn part_2(input: &[Vec<char>]) -> i64 {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|s| count_trees(input, *s))
        .product::<i64>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    pub fn part_1() {
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

        let input = super::parse(inputs);
        let result = super::part_1(&input);
        println!("result: {}", result);
        assert!(result == 7);
    }
}
