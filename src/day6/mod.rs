use std::collections::{HashSet, HashMap};

static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    INPUT
        .split("\n\n")
        .map(|g| {
            g.chars()
             .filter(|c| ('a'..='z').contains(c))
             .collect::<HashSet<_>>()
             .iter()
             .count()
        })
        .sum()
}

pub fn part2() -> usize {
    INPUT
        .split("\n\n")
        .map(|g| {
            let people = g.lines().count();
            g.chars()
             .filter(|c| ('a'..='z').contains(c))
             .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
             })
             .iter()
             .filter(|&(_, &v)| v == people)
             .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 906);
    }
    
    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 519);
    }
}
