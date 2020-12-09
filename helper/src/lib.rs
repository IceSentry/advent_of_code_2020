use std::cmp::min;
use std::iter;
use std::time::Duration;

use clap::Clap;
use colored::*;

pub use clap;
pub use criterion;
pub use dotenv;

pub mod input;

const DISPLAY_WIDTH: usize = 40;

pub fn print_with_duration(line: &str, output: Option<&str>, duration: Duration) {
    let duration = format!("({:.2?})", duration);
    print!("  - {} {}", line, duration.dimmed());

    if let Some(output) = output {
        let width = "  - ".len() + line.chars().count() + 1 + duration.chars().count();
        let dots = DISPLAY_WIDTH - min(DISPLAY_WIDTH - 5, width) - 2;
        let dots: String = iter::repeat('.').take(dots).collect();
        print!(" {}", dots.dimmed());

        if output.contains('\n') {
            println!();

            for line in output.trim_matches('\n').lines() {
                println!("    {}", line.bold());
            }
        } else {
            println!(" {}", output.bold());
        }
    } else {
        println!()
    }
}

#[derive(Clap)]
#[clap(version = "1.0", author = "IceSentry")]
pub struct Opts {
    pub day: String,
    #[clap(short, long)]
    pub bench: bool,
    #[clap(short, long)]
    pub download: bool,
}

#[macro_export]
macro_rules! main {
    (
        year $year: expr;
        $( $day: ident $( : $parser: ident )? => $( $solution: ident ),+ );+
        $( ; )?
    ) => {
        use std::fs::read_to_string;
        use std::io::Read;
        use std::time::Instant;

        use $crate::clap::Clap;

        const YEAR: u16 = $year;
        const DAYS: &[&str] = &[$(stringify!($day)),*];

        fn main() {
            $crate::dotenv::dotenv().expect("Failed to load .env");

            let mut opt = $crate::Opts::parse();
            let module_name = format!("day{}", opt.day);
            let day: u8 = opt.day.parse().expect("Day is not a number");

            if opt.bench {
                bench::run_bench(module_name);
                return
            }

            if opt.download {
                $crate::input::get_input($year, day).unwrap();
            }


            if !DAYS.contains(&module_name.as_str()) {
                eprintln!(
                    "Module `{}` was not registered, available are: {}",
                    module_name,
                    DAYS.join(", "),
                );
            }

            $(
                if stringify!($day) == module_name {
                    println!("Day {}", day);

                    let data = $crate::input::get_input(YEAR, day).expect("could not fetch input");
                    let input = data.as_str();

                    $(
                        let start = Instant::now();
                        let input = $day::$parser(&data);
                        let elapsed = start.elapsed();
                        $crate::print_with_duration("parser", None, elapsed);
                    )?

                    $({
                        let start = Instant::now();
                        let response = $day::$solution(&input);
                        let elapsed = start.elapsed();

                        $crate::print_with_duration(
                            stringify!($solution),
                            Some(&format!("{}", response)),
                            elapsed,
                        );
                    })+
                }
            )+
        }


        mod bench {
            use $crate::criterion::{BatchSize, Criterion};

            pub fn run_bench(module_name: String) {
                $(
                    if module_name == stringify!($day) {
                        $day();
                    }
                )+
            }

            $(
                fn $day() {
                    let mut criterion = Criterion::default().without_plots();
                    let mut group = criterion.benchmark_group(stringify!($day));
                    let day = stringify!($day)[3..].parse().expect("dayX expected for module");

                    let data = $crate::input::get_input(crate::YEAR, day)
                        .expect("could not fetch input");

                    let input = data.as_str();

                    $(
                        group.bench_with_input(stringify!($parser), &input, |b, i| {
                            b.iter_with_large_drop(|| {
                                crate::$day::$parser(&data);
                            });
                        });
                    )+

                    $( let input = crate::$day::$parser(&data); )?

                    $(
                        group.bench_with_input(stringify!($solution), &input, |b, i| {
                            b.iter_batched(|| i, |input| crate::$day::$solution(input), BatchSize::SmallInput)
                        });
                    )+

                    group.finish();
                }
            )+
        }
    };
}
