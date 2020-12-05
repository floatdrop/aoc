use std::collections::HashSet;

static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    INPUT.lines().map(|s| {
        s.chars().fold(0, |acc, c| {
            match c {
                'F' | 'L' => { acc << 1 },
                'B' | 'R' => { acc << 1 | 1 },
                _ => unreachable!(),
            }
        })
    })
    .max().unwrap()
}

pub fn part2() -> usize {
    let ids: HashSet<usize> = INPUT.lines().map(|s| {
        s.chars().fold(0, |acc, c| {
            match c {
                'F' | 'L' => { acc << 1 },
                'B' | 'R' => { acc << 1 | 1 },
                _ => unreachable!(),
            }
        })
    })
    .collect();

    let min = *ids.iter().min().unwrap();
    let max = *ids.iter().max().unwrap();

    let all: HashSet<usize> = (min..max).collect();
    *all.difference(&ids).next().unwrap()
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
