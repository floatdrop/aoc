use std::collections::HashSet;

static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    INPUT.lines().map(|s| {
        let pos = s.chars().fold((0, 8, 0, 128), |acc, c| {
            match c {
                'F' => { (acc.0, acc.1, acc.2, (acc.2 + acc.3) / 2) },
                'B' => { (acc.0, acc.1, (acc.2 + acc.3) / 2, acc.3) },
                'L' => { (acc.0, (acc.0 + acc.1) / 2, acc.2, acc.3) },
                'R' => { ((acc.0 + acc.1) / 2, acc.1, acc.2, acc.3) },
                _ => unreachable!(),
            }
        });
        (pos.0, pos.2)
    })
    .map(|(x, y)| y * 8 + x)
    .max().unwrap()
}

pub fn part2() -> usize {
    let ids: HashSet<usize> = INPUT.lines().map(|s| {
        let pos = s.chars().fold((0, 8, 0, 128), |acc, c| {
            match c {
                'F' => { (acc.0, acc.1, acc.2, (acc.2 + acc.3) / 2) },
                'B' => { (acc.0, acc.1, (acc.2 + acc.3) / 2, acc.3) },
                'L' => { (acc.0, (acc.0 + acc.1) / 2, acc.2, acc.3) },
                'R' => { ((acc.0 + acc.1) / 2, acc.1, acc.2, acc.3) },
                _ => unreachable!(),
            }
        });
        (pos.0, pos.2)
    })
    .map(|(x, y)| y * 8 + x)
    .collect();

    let all: HashSet<usize> = (100..900).collect();
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
