use itertools::Itertools;

type Data = u64;

pub fn parse(input: &str) -> Vec<Data> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn find_wrong_value(input: &[Data], offset: usize) -> Data {
    for (i, w) in input.windows(offset).enumerate().skip(offset) {
        let target = input[i + offset];
        if !w
            .iter()
            .tuple_combinations()
            .any(|(a, b)| *a + *b == target)
        {
            return target;
        }
    }

    unreachable!()
}

fn find_contiguous_combinations(input: &[Data], target: Data) -> Vec<Data> {
    let mut acc = 0;
    let mut index = 0;
    let mut last_index = 0;
    let mut values = vec![];
    while acc != target {
        index += 1;
        if index >= input.len() {
            last_index += 1;
            index = last_index;
            values.clear();
            acc = 0;
        }
        acc += input[index];
        values.push(input[index]);
    }
    values.sort();
    values
}

pub fn part_1(input: &[Data]) -> Data {
    find_wrong_value(input, 25)
}

pub fn part_2(input: &[Data]) -> Data {
    let target = find_wrong_value(input, 25);
    let result = find_contiguous_combinations(&input, target);
    let first = result.first().unwrap();
    let last = result.last().unwrap();
    first + last
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::find_wrong_value(&input, 5);
        assert_eq!(result, 127);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let target = super::find_wrong_value(&input, 5);
        let result = super::find_contiguous_combinations(&input, target);
        let first = result.first().unwrap();
        let last = result.last().unwrap();
        assert_eq!(first, &15);
        assert_eq!(last, &47);
        assert_eq!(first + last, 62);
    }
}
