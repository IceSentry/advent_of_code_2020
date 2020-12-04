use crate::solver::Solver;
use anyhow::Result;
use regex::Regex;
use serde_scan::scan;
use std::collections::HashMap;

fn contains_keys(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn validate_fields(key: &str, v: &str) -> bool {
    match key {
        "byr" => (1920..=2002).contains(&v.parse::<i32>().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&v.parse::<i32>().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&v.parse::<i32>().unwrap_or(0)),
        "hgt" => {
            let re = Regex::new(r"(\d*)(cm|in)").unwrap();
            if let Some(captures) = re.captures(v) {
                let number: i32 = captures[1].parse().unwrap();
                let unit: String = captures[2].parse().unwrap();
                match unit.as_str() {
                    "cm" => (150..=193).contains(&number),
                    "in" => (59..=76).contains(&number),
                    _ => false,
                }
            } else {
                false
            }
        }
        "hcl" => {
            v.starts_with("#") && v.len() == 7 && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => match v {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_numeric()),
        "cid" => true,
        _ => false,
    }
}

pub struct Day04 {}
impl Solver for Day04 {
    type Input = Vec<HashMap<String, String>>;
    type Output = usize;

    fn parse(input: &str) -> Result<Self::Input> {
        // TODO could probably use iterators here
        // split_whitespace or split([':', ' ', '\n'])
        let mut passports: Vec<HashMap<_, _>> = vec![HashMap::new()];
        for line in input.lines() {
            if line.is_empty() {
                passports.push(HashMap::new());
                continue;
            }

            let line_fields = line
                .split(' ')
                .map(|s| scan!("{}:{}" <- s).unwrap())
                .collect::<HashMap<String, String>>();
            passports.last_mut().unwrap().extend(line_fields);
        }
        Ok(passports.into_iter().filter(|p| contains_keys(p)).collect())
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().count()
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .filter(|p| p.iter().all(|(k, v)| validate_fields(k, v)))
                .count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_fields, Day04};
    use crate::solver::Solver;
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
    "};

    #[test]
    fn part1() {
        let input = Day04::parse(INPUTS);
        println!("{:?}", input);

        let result = Day04::part1(&input.unwrap());
        println!("result: {}", result);

        assert!(result == 2);
    }

    #[test]
    fn part2_invalid() {
        let invalid = indoc! {"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        "};
        let input = Day04::parse(invalid);
        let result = Day04::part2(&input.unwrap());
        assert!(result == Some(0));
    }

    #[test]
    fn part2_valid() {
        let valid = indoc! {"
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f
            
            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
            
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022
            
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "};
        let input = Day04::parse(valid);
        let result = Day04::part2(&input.unwrap());
        println!("result: {:?}", result);

        assert!(result == Some(4));
    }

    #[test]
    fn validate() {
        assert!(validate_fields("byr", "2002") == true);
        assert!(validate_fields("byr", "2003") == false);

        assert!(validate_fields("hgt", "60in") == true);
        assert!(validate_fields("hgt", "190cm") == true);
        assert!(validate_fields("hgt", "190in") == false);
        assert!(validate_fields("hgt", "190") == false);

        assert!(validate_fields("hcl", "#123abc") == true);
        assert!(validate_fields("hcl", "#123abz") == false);
        assert!(validate_fields("hcl", "123abc") == false);

        assert!(validate_fields("ecl", "brn") == true);
        assert!(validate_fields("ecl", "wat") == false);

        assert!(validate_fields("pid", "000000001") == true);
        assert!(validate_fields("pid", "0123456789") == false);
    }
}
