mod solutions;
mod solver;

use anyhow::{anyhow, Result};
use clap::Clap;
use solver::Solver;
use std::{fs, path::Path};

/// advent_of_code_2020
#[derive(Clap)]
#[clap(version = "1.0", author = "IceSentry")]
struct Opts {
    day: i32,
}

fn download_input(year: i32, day: i32) -> Result<()> {
    let file_path_string = &format!("./inputs/{}/{:02}.txt", year, day);
    let file_path = Path::new(file_path_string);
    println!("{:?}", file_path);
    if file_path.exists() {
        return Ok(());
    }
    println!("downloading inputs...");

    let path = &format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let session = std::env::var("COOKIE_SESSION")?;
    let resp = ureq::get(path)
        .set("COOKIE", &format!("session={}", session))
        .call();

    println!("inputs downloaded");

    if resp.ok() {
        fs::create_dir_all(&format!("./inputs/{}/", year))?;
        fs::write(file_path, resp.into_string()?)?;

        println!("file written succesfully");

        Ok(())
    } else {
        Err(anyhow!("{} Failed to download inputs", resp.status()))
    }
}

fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to load .env");
    let opts: Opts = Opts::parse();

    let year = 2020;
    download_input(year, opts.day)?;

    let input = fs::read_to_string(&format!("./inputs/{}/{:02}.txt", year, opts.day))?;

    match opts.day {
        1 => solutions::day01::Day01 {}.solve(&input),
        2 => solutions::day02::Day02 {}.solve(&input),
        day => println!("day {} not solved yet", day),
    };

    Ok(())
}
