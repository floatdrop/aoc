use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    INPUT
        .split("\n\n")
        .filter(|s| {
            s.contains("byr:")
                && s.contains("iyr:")
                && s.contains("eyr:")
                && s.contains("hgt:")
                && s.contains("hcl:")
                && s.contains("ecl:")
                && s.contains("pid:")
        })
        .count()
}

#[derive(Debug)]
pub struct Passport;

#[derive(Debug, Clone)]
pub struct ParsePassportError;

pub fn check_year(s: &str, range: std::ops::RangeInclusive<i64>) -> bool {
    s.parse::<i64>()
        .and_then(|year| Ok(range.contains(&year)))
        .unwrap_or(false)
}

pub fn check_height(s: &str) -> bool {
    if let Ok((height, unit)) = scan_fmt!(s, "{d}{}", usize, String) {
        return match unit.as_str() {
            "cm" => (150..=193).contains(&height),
            "in" => (59..=76).contains(&height),
            _ => false,
        };
    }
    false
}

impl FromStr for Passport {
    type Err = ParsePassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#([a-z0-9]{6})$").unwrap();
            static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^(\d{9})$").unwrap();
        }

        let map: HashMap<&str, &str> = s
            .split_whitespace()
            .filter_map(|field| field.splitn(2, ":").collect_tuple::<(_, _)>())
            .collect();

        if !["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&k| map.contains_key(k))
        {
            return Err(ParsePassportError);
        }

        if map
            .iter()
            .map(|(&k, &v)| match k {
                "byr" => check_year(v, 1920..=2002),
                "iyr" => check_year(v, 2010..=2020),
                "eyr" => check_year(v, 2020..=2030),
                "hgt" => check_height(v),
                "hcl" => HCL_RE.is_match(v),
                "ecl" => ECL_RE.is_match(v),
                "pid" => PID_RE.is_match(v),
                _ => true,
            })
            .any(|check| check == false)
        {
            return Err(ParsePassportError);
        }

        Ok(Passport {})
    }
}

pub fn part2() -> usize {
    INPUT
        .split("\n\n")
        .filter_map(|s| Passport::from_str(s).ok())
        .count()
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
