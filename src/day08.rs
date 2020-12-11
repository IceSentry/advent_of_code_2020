use core::panic;
use rayon::prelude::*;

use serde_scan::scan;

type Data = (String, i32);

pub fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| scan!("{} {}" <- line).unwrap())
        .collect()
}

#[derive(Debug)]
enum ExitCondition {
    InfiniteLoop(i32),
    Terminated,
}

fn run(instructions: &[Data]) -> (i32, ExitCondition) {
    let mut acc = 0;
    let mut instruction_pointer = 0;
    let mut visited = vec![instruction_pointer];
    loop {
        let (instruction, value) = instructions[instruction_pointer as usize].clone();
        instruction_pointer += match &instruction[..] {
            "acc" => {
                acc += value;
                1
            }
            "jmp" => value,
            "nop" => 1,
            _ => panic!("Unknown instruction {}", instruction),
        };

        if visited.contains(&instruction_pointer) {
            return (acc, ExitCondition::InfiniteLoop(visited[visited.len() - 1]));
        } else {
            visited.push(instruction_pointer);
        }

        if instruction_pointer as usize >= instructions.len() {
            return (acc, ExitCondition::Terminated);
        }
    }
}

pub fn part_1(input: &[Data]) -> i32 {
    let (acc, _) = run(input);
    acc
}

pub fn part_2(input: &[Data]) -> i32 {
    input
        .iter()
        .enumerate()
        .filter(|(_, (instruction, _))| matches!(&instruction[..], "nop" | "jmp"))
        .find_map(|(i, _)| {
            let mut new_input = input.to_vec();
            let instruction = &mut new_input[i];
            instruction.0 = match &instruction.0[..] {
                "jmp" => "nop".to_string(),
                "nop" => "jmp".to_string(),
                _ => panic!("instructions is not jmp or nop"),
            };
            let (acc, exit) = run(&new_input);
            if let ExitCondition::Terminated = exit {
                Some(acc)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part_2_par(input: &[Data]) -> i32 {
    input
        .par_iter()
        .enumerate()
        .filter(|(_, (instruction, _))| matches!(&instruction[..], "nop" | "jmp"))
        .find_map_any(|(i, _)| {
            let mut new_input = input.to_vec();
            let instruction = &mut new_input[i];
            instruction.0 = match &instruction.0[..] {
                "jmp" => "nop".to_string(),
                "nop" => "jmp".to_string(),
                _ => panic!("instructions is not jmp or nop"),
            };
            let (acc, exit) = run(&new_input);
            if let ExitCondition::Terminated = exit {
                Some(acc)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        nop -4
        acc +6
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 8);
    }
}
