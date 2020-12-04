mod solutions;
pub mod solver;

use anyhow::{anyhow, Result};
use clap::Clap;
use solver::Solver;
use std::{fs, path::Path};

/// advent_of_code_2020
#[derive(Clap)]
#[clap(version = "1.0", author = "IceSentry")]
struct Opts {
    day: u8,
    #[clap(short, long)]
    bench: bool,
}

fn filename(year: i32, day: u8) -> String {
    format!("./inputs/{}/{:02}.txt", year, day)
}

fn download_input(year: i32, day: u8) -> Result<()> {
    let filename = filename(year, day);
    let file_path = Path::new(&filename);
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

macro_rules! day {
    ( $d:expr, $input:expr ) => {
        paste::paste! {
            solve::<solutions::[<day $d>]::[<Day $d>]>($input)
        }
    };
}

fn bench<'a, S: Solver>(day: u8, input: &'a str) -> Result<()> {
    let mut criterion = criterion::Criterion::default().without_plots();
    let mut group = criterion.benchmark_group(format!("Day {}", day));

    group.bench_with_input("parser", &input, |b, i| {
        b.iter_with_large_drop(|| S::parse(i));
    });

    let input = S::parse(input)?;

    group.bench_with_input("part 1", &input, |b, i| {
        b.iter_batched(|| i, S::part1, criterion::BatchSize::SmallInput)
    });

    group.bench_with_input("part 2", &input, |b, i| {
        b.iter_batched(|| i, S::part2, criterion::BatchSize::SmallInput)
    });

    Ok(())
}

fn solve<'a, S: Solver>(input: &'a str) -> Result<()> {
    let opts: Opts = Opts::parse();
    if opts.bench {
        return bench::<S>(opts.day, input);
    }

    let input = S::parse(input)?;
    let first_result = S::part1(&input);
    println!("part1: {}", first_result);
    if let Some(second_result) = S::part2(&input) {
        println!("part2: {:?}", second_result);
    }
    Ok(())
}

fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to load .env");
    let opts: Opts = Opts::parse();

    let year = 2020;
    download_input(year, opts.day)?;

    let filename = filename(year, opts.day);
    let input = fs::read_to_string(&filename)?;

    match opts.day {
        1 => day!(01, &input)?,
        2 => day!(02, &input)?,
        3 => day!(03, &input)?,
        4 => day!(04, &input)?,
        day => println!("day {} not solved yet", day),
    };

    Ok(())
}
