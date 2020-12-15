use std::collections::HashMap;

type Data = i32;

pub fn parse(input: &str) -> Vec<Data> {
    let mut data: Vec<Data> = input.lines().map(|l| l.parse().unwrap()).collect();
    data.push(0);
    data.sort_unstable();
    data
}

pub fn part_1(input: &[Data]) -> usize {
    let mut map = HashMap::new();
    for adapter in input.windows(2) {
        let delta = adapter[1] - adapter[0];
        *map.entry(delta).or_insert(1) += 1;
    }
    map.get(&1).unwrap() * map.get(&3).unwrap()
}

fn count_paths(input: &[Data], cache: &mut HashMap<usize, u64>, i: usize) -> u64 {
    if i >= input.len() - 1 {
        1
    } else if cache.contains_key(&i) {
        cache[&i]
    } else {
        let mut answer = 0;
        for j in i + 1..input.len() {
            if input[j] - input[i] <= 3 {
                answer += count_paths(input, cache, j);
            }
        }
        cache.insert(i, answer);
        answer
    }
}

pub fn part_2(input: &[Data]) -> u64 {
    let mut cache = HashMap::new();
    count_paths(input, &mut cache, 0)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "};

    const INPUTS2: &str = indoc! {"
        28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS2);
        let result = super::part_1(&input);
        assert_eq!(result, 220);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 8);
    }
}
