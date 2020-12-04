extern crate lazy_static;
extern crate regex;

use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;
use itertools::join;

static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    let rows: Vec<&str> = INPUT.lines().collect();

    let mut count = 0;

    for (_, g) in rows.iter().group_by(|&&l| l == "").into_iter() {
        let s = join(g, " ");


        if s == "" {
            continue;
        }

        if s.contains("byr:") && s.contains("iyr:") && s.contains("eyr:")
            && s.contains("hgt:") && s.contains("hcl:") && s.contains("ecl:")
                && s.contains("pid:") {
            count += 1
        }
    }

    count
}

pub fn part2() -> usize {
    lazy_static! {
        static ref BYR_RE: Regex = Regex::new(r"byr:(\d{4}) ").unwrap();
        static ref IYR_RE: Regex = Regex::new(r"iyr:(\d{4}) ").unwrap();
        static ref EYR_RE: Regex = Regex::new(r"eyr:(\d{4}) ").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"hgt:(\d{2,3})(cm|in) ").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"hcl:#([a-z0-9]{6}) ").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth) ").unwrap();
        static ref PID_RE: Regex = Regex::new(r"pid:(\d{9}) ").unwrap();
    }

    let rows: Vec<&str> = INPUT.lines().collect();

    let mut count = 0;

    for (_, g) in rows.iter().group_by(|&&l| l == "").into_iter() {
        let mut s = join(g, " ");

        if s == "" {
            continue;
        }

        s += " ";

        if let Some(c) = BYR_RE.captures(&s) {
            let byr = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            if byr < 1920 || byr > 2002 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(c) = IYR_RE.captures(&s) {
            let iyr = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            if iyr < 2010 || iyr > 2020 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(c) = EYR_RE.captures(&s) {
            let eyr = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            if eyr < 2020 || eyr > 2030 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(c) = HGT_RE.captures(&s) {
            let hgt = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let units = c.get(2).unwrap().as_str();
            if units == "cm" && (hgt < 150 || hgt > 193) {
                continue;
            }
            if units == "in" && (hgt < 59 || hgt > 76) {
                continue;
            }
        } else {
            continue;
        }

        if !HCL_RE.is_match(&s) {
            continue;
        }

        if !ECL_RE.is_match(&s) {
            continue;
        }

        if !PID_RE.is_match(&s) {
            continue;
        }
        
        count += 1
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 190);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 121);
    }
}
