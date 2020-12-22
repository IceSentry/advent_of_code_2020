type Data = (char, f32);

pub fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let dir = chars.next().unwrap();
            let value = chars.collect::<String>().parse().unwrap();
            (dir, value)
        })
        .collect()
}

pub fn part_1(input: &[Data]) -> f32 {
    let mut orientation = 90.0;
    let mut east = 0.0;
    let mut north = 0.0;
    for (dir, value) in input.iter() {
        match dir {
            'N' => north += value,
            'S' => north -= value,
            'E' => east += value,
            'W' => east -= value,
            'L' => orientation -= value,
            'R' => orientation += value,
            'F' => {
                if orientation == 90.0 {
                    east += value;
                } else if orientation == 180.0 {
                    north -= value;
                } else if orientation == 270.0 {
                    east -= value;
                } else if orientation > 180.0 && orientation < 360.0 {
                    north -= value * (orientation * std::f32::consts::PI / 180.0).cos();
                    east -= value * (orientation * std::f32::consts::PI / 180.0).sin();
                } else {
                    north += value * (orientation * std::f32::consts::PI / 180.0).cos();
                    east += value * (orientation * std::f32::consts::PI / 180.0).sin();
                };
            }
            _ => unreachable!(),
        }
    }

    (east.abs() + north.abs()).round()
}

pub fn part_2(input: &[Data]) -> f32 {
    0.0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        F10
        N3
        F7
        R90
        F11
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 25.0);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 286.0);
    }
}
