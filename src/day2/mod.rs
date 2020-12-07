use serde::Deserialize;
use serde_scan::scan;

static INPUT: &str = std::include_str!("input.txt");

#[derive(Debug, Deserialize)]
struct OTCASPolicy {
    lo: usize,
    hi: usize,
    letter: char,
    password: String,
}

pub fn part1() -> usize {
    let passwords = INPUT.lines().map(|l| {
        let policy: OTCASPolicy = scan!("{}-{} {}: {}" <- l).unwrap();
        policy
    });

    passwords
        .filter(|p| {
            let count = p.password.matches(p.letter).count();
            p.lo <= count && count <= p.hi
        })
        .count()
}

pub fn part2() -> usize {
    let passwords = INPUT.lines().map(|l| {
        let policy: OTCASPolicy = scan!("{}-{} {}: {}" <- l).unwrap();
        policy
    });

    passwords
        .filter(|p| {
            let a = p.password.chars().nth(p.lo - 1).unwrap();
            let b = p.password.chars().nth(p.hi - 1).unwrap();
            (a == p.letter) ^ (b == p.letter)
        })
        .count()
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
