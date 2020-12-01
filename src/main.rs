use std::fs;

use clap::Clap;

mod solutions;
mod solver;

use solver::Solver;

/// advent_of_code_2020
#[derive(Clap)]
#[clap(version = "1.0", author = "IceSentry")]
struct Opts {
    day: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    let filename = format!("inputs/{:02}.txt", opts.day);
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    match opts.day {
        1 => solutions::day01::Day01 {}.solve(input),
        day => println!("day {} not solved yet", day),
    }
}
