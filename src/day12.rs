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

fn rot(x: f32, y: f32, d: &f32) -> (f32, f32) {
    match *d as i32 {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => unreachable!(),
    }
}

pub fn part_2(input: &[Data]) -> f32 {
    let mut east = 0.0;
    let mut north = 0.0;
    let mut east_waypoint = 10.0;
    let mut north_waypoint = 1.0;
    for (dir, value) in input.iter() {
        match dir {
            'N' => north_waypoint += value,
            'S' => north_waypoint -= value,
            'E' => east_waypoint += value,
            'W' => east_waypoint -= value,
            'L' => {
                let (a, b) = rot(east_waypoint, north_waypoint, value);
                east_waypoint = a;
                north_waypoint = b;
            }
            'R' => {
                let (a, b) = rot(east_waypoint, north_waypoint, &(360.0 - value));
                east_waypoint = a;
                north_waypoint = b;
            }
            'F' => {
                east += east_waypoint * value;
                north += north_waypoint * value;
            }
            _ => unreachable!(),
        }
    }

    (east.abs() + north.abs()).round()
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
