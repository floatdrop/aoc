extern crate recap;

static INPUT: &str = std::include_str!("input.txt");

use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<lo>\d+)\-(?P<hi>\d+) (?P<letter>[a-z]): (?P<password>.+)"#)]
struct OTCASPolicy {
    lo: usize,
    hi: usize,
    letter: char,
    password: String
}

pub fn part1() -> usize {
    let passwords = INPUT.lines().map(|l| {
        l.parse::<OTCASPolicy>().unwrap()
    });
    
    passwords.filter(|p| {
        let count = p.password.matches(p.letter).count();
        p.lo <= count && count <= p.hi
    }).count()
}

pub fn part2() -> usize {
    let passwords = INPUT.lines().map(|l| {
        l.parse::<OTCASPolicy>().unwrap()
    });
    
    passwords.filter(|p| {
        let a = p.password.chars().nth(p.lo - 1).unwrap();
        let b = p.password.chars().nth(p.hi - 1).unwrap();
        (a == p.letter) ^ (b == p.letter)
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 398);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 562);
    }
}
