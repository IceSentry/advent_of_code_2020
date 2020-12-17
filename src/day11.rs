type Grid = Vec<Vec<u8>>;

pub fn parse(input: &str) -> Grid {
    input.lines().map(|l| l.bytes().collect()).collect()
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    println!();
    for line in grid {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
    println!();
}

fn check_position(grid: &Grid, x: i32, y: i32, c: u8) -> bool {
    if x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32 {
        grid[y as usize][x as usize] == c
    } else {
        false
    }
}

fn count_occupied(grid: &Grid, x: i32, y: i32, use_loop: bool) -> i32 {
    let mut count = 0;
    for dy in [-1, 0, 1].iter() {
        for dx in [-1, 0, 1].iter() {
            if dx == &0 && dy == &0 {
                continue;
            }
            let mut x = x + dx;
            let mut y = y + dy;
            while use_loop && check_position(grid, x, y, b'.') {
                x += dx;
                y += dy;
            }
            if check_position(grid, x, y, b'#') {
                count += 1;
            }
        }
    }
    count
}

fn run(grid: &Grid, limit: i32, use_loop: bool) -> usize {
    let mut grid = grid.clone();
    loop {
        let mut updated_grid = grid.clone();
        let mut changed = false;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let occupied = count_occupied(&grid, x as i32, y as i32, use_loop);
                match grid[y][x] {
                    b'L' if occupied == 0 => {
                        updated_grid[y][x] = b'#';
                        changed = true;
                    }
                    b'#' if occupied >= limit => {
                        updated_grid[y][x] = b'L';
                        changed = true;
                    }
                    b'.' => continue,
                    _ => (),
                }
            }
        }
        grid = updated_grid;

        if !changed {
            return grid.iter().flatten().filter(|c| **c == b'#').count();
        }
    }
}

pub fn part_1(input: &Grid) -> usize {
    run(input, 4, false)
}

pub fn part_2(input: &Grid) -> usize {
    run(input, 5, true)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 37);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 26);
    }

    #[test]
    pub fn count_occupied() {
        let grid = super::parse(indoc! {"
            ...
            .L.
            ...
        "});
        assert_eq!(super::count_occupied(&grid, 1, 1, false), 0);

        let grid = super::parse(indoc! {"
            LLL
            LLL
            LLL
        "});
        assert_eq!(super::count_occupied(&grid, 1, 1, false), 0);

        let grid = super::parse(indoc! {"
            L#L
            #L#
            L#L
        "});
        assert_eq!(super::count_occupied(&grid, 1, 1, false), 4);

        let grid = super::parse(indoc! {"
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
        "});
        assert_eq!(super::count_occupied(&grid, 3, 4, true), 8);

        let grid = super::parse(indoc! {"
            .............
            .L.L.#.#.#.#.
            .............
        "});
        assert_eq!(super::count_occupied(&grid, 1, 1, true), 0);

        let grid = super::parse(indoc! {"
            .##.##.
            #.#.#.#
            ##...##
            ...L...
            ##...##
            #.#.#.#
            .##.##.
        "});
        assert_eq!(super::count_occupied(&grid, 3, 3, true), 0);
    }
}
