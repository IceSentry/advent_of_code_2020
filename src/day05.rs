use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Seat {
    row: u8,
    col: u8,
    id: i32,
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse(input: &str) -> Vec<Seat> {
    let mut vec: Vec<Seat> = input
        .lines()
        .map(|l| {
            let row = &l[..7].replace('F', "0").replace('B', "1");
            let col = &l[7..].replace('L', "0").replace('R', "1");
            let row = u8::from_str_radix(row, 2).unwrap();
            let col = u8::from_str_radix(col, 2).unwrap();
            Seat {
                row,
                col,
                id: row as i32 * 8 + col as i32,
            }
        })
        .collect();
    vec.sort();
    vec
}

#[allow(clippy::ptr_arg)]
pub fn part_1(input: &Vec<Seat>) -> i32 {
    input.iter().max().unwrap().id
}

#[allow(clippy::ptr_arg)]
pub fn part_2(input: &Vec<Seat>) -> i32 {
    let mut it = input.iter().peekable();
    while let Some(seat) = it.next() {
        if it.peek().unwrap().id != seat.id + 1 {
            return seat.id + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        FBFBBFFRLR
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL
    "};

    const FILE_INPUTS: &str = include_str!("../inputs/2020/05.txt");

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        println!("{:#?}", input);

        let result = super::part_1(&input);
        assert_eq!(result, 820);
    }

    #[test]
    fn file() {
        let input = super::parse(FILE_INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 953);

        let result = super::part_2(&input);
        assert_eq!(result, 615);
    }
}
