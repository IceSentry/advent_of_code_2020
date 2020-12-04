use crate::solver::Solver;
use anyhow::Result;
use regex::Regex;
use serde_scan::scan;
use std::collections::HashMap;

pub struct Day04 {}
impl Solver for Day04 {
    type Input = Vec<HashMap<String, String>>;
    type Output = usize;

    fn parse(input: &str) -> Result<Self::Input> {
        let mut passports: Vec<HashMap<String, String>> = vec![HashMap::new()];
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
        Ok(passports)
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|p| {
                p.contains_key("byr")
                    && p.contains_key("iyr")
                    && p.contains_key("eyr")
                    && p.contains_key("hgt")
                    && p.contains_key("hcl")
                    && p.contains_key("ecl")
                    && p.contains_key("pid")
            })
            .count()
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .filter(|p| {
                    p.contains_key("byr")
                        && p.contains_key("iyr")
                        && p.contains_key("eyr")
                        && p.contains_key("hgt")
                        && p.contains_key("hcl")
                        && p.contains_key("ecl")
                        && p.contains_key("pid")
                })
                .filter(|p| {
                    if let Some(byr) = p.get("byr").and_then(|v| v.parse::<i32>().ok()) {
                        if !(byr >= 1920 && byr <= 2002) {
                            // println!("byr: {}", byr);
                            return false;
                        }
                    }
                    if let Some(iyr) = p.get("iyr").and_then(|v| v.parse::<i32>().ok()) {
                        if !(iyr >= 2010 && iyr <= 2020) {
                            // println!("iyr: {}", iyr);
                            return false;
                        }
                    }
                    if let Some(eyr) = p.get("eyr").and_then(|v| v.parse::<i32>().ok()) {
                        if !(eyr >= 2020 && eyr <= 2030) {
                            // println!("eyr: {}", eyr);
                            return false;
                        }
                    }
                    if let Some(hgt) = p.get("hgt") {
                        let re = Regex::new(r"(\d*)(cm|in)").unwrap();
                        if let Some(captures) = re.captures(hgt) {
                            let number: i32 = captures[1].parse().unwrap();
                            let unit: String = captures[2].parse().unwrap();
                            let is_valid = match unit.as_str() {
                                "cm" => number >= 150 && number <= 193,
                                "in" => number >= 59 && number <= 76,
                                _ => false,
                            };

                            if !is_valid {
                                // println!("hgt: {} {:?}", hgt, captures);
                                return false;
                            }
                        }
                    }
                    if let Some(hcl) = p.get("hcl") {
                        let re = Regex::new(r"#[a-f\d]{6}").unwrap();
                        if !re.is_match(hcl) {
                            // println!("hcl: {}", hcl);
                            return false;
                        }
                    }
                    if let Some(ecl) = p.get("ecl") {
                        match ecl.as_str() {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                            _ => {
                                // println!("ecl: {}", ecl);
                                return false;
                            }
                        }
                    }
                    if let Some(pid) = p.get("pid") {
                        let re = Regex::new(r"\d{9}").unwrap();
                        if !re.is_match(pid) {
                            // println!("pid: {}", pid);
                            return false;
                        }
                    }
                    true
                })
                .count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Day04;
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
}
