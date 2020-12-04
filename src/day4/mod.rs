use regex::Regex;
use lazy_static::lazy_static;

static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    INPUT.split("\n\n").filter(|s| {
        s.contains("byr:") && s.contains("iyr:") && s.contains("eyr:")
            && s.contains("hgt:") && s.contains("hcl:") && s.contains("ecl:")
                && s.contains("pid:")
    }).count()
}

pub fn part2() -> usize {
    lazy_static! {
        static ref BYR_RE: Regex = Regex::new(r"byr:(\d{4})\b").unwrap();
        static ref IYR_RE: Regex = Regex::new(r"iyr:(\d{4})\b").unwrap();
        static ref EYR_RE: Regex = Regex::new(r"eyr:(\d{4})\b").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"hgt:(\d{2,3})(cm|in)\b").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"hcl:#([a-z0-9]{6})\b").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
        static ref PID_RE: Regex = Regex::new(r"pid:(\d{9})\b").unwrap();
    }

    INPUT.split("\n\n").filter(|s| {
        if let Some(c) = BYR_RE.captures(&s) {
            let byr = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            if !(1920..=2002).contains(&byr) {
                return false;
            }
        } else {
            return false;
        }

        if let Some(c) = IYR_RE.captures(&s) {
            let iyr = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            if !(2010..=2020).contains(&iyr) {
                return false;
            }
        } else {
            return false;
        }

        if let Some(c) = EYR_RE.captures(&s) {
            let eyr = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            if !(2020..=2030).contains(&eyr) {
                return false;
            }
        } else {
            return false;
        }

        if let Some(c) = HGT_RE.captures(&s) {
            let hgt = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let units = c.get(2).unwrap().as_str();
            if units == "cm" && !(150..=193).contains(&hgt) {
                return false;
            }
            if units == "in" && !(59..=76).contains(&hgt) {
                return false;
            }
        } else {
            return false;
        }

        if !HCL_RE.is_match(&s) {
            return false;
        }

        if !ECL_RE.is_match(&s) {
            return false;
        }

        if !PID_RE.is_match(&s) {
            return false;
        }

        true
    }).count()
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
