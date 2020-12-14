use helper::main;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

main! {
    year 2020;
    day01 : parse => part_1, part_2;
    day02 : parse => part_1, part_2;
    day03 : parse => part_1, part_2;
    day04 : parse => part_1, part_2;
    day05 : parse => part_1, part_2;
    day06 : parse => part_1, part_2;
    day07 : parse => part_1, part_1_cache, part_2;
    day08 : parse => part_1, part_2, part_2_par;
    day09 : parse => part_1, part_2;
}
