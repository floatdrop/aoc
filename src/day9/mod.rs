extern crate itertools;

use itertools::Itertools;
use std::collections::VecDeque;

static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> i64 {
    let numbers: Vec<i64> = INPUT
        .split('\n')
        .filter_map(|l| l.parse::<i64>().ok())
        .collect();

    *numbers
        .windows(26)
        .find_map(|g| {
            let (number, preamble) = g.split_last()?;
            if preamble
                .iter()
                .tuple_combinations::<(_, _)>()
                .all(|(a, b)| a + b != *number)
            {
                return Some(number);
            }

            None
        })
        .unwrap()
}

pub fn part2() -> i64 {
    let target = 26134589;

    let mut list: VecDeque<i64> = VecDeque::new();

    let numbers: Vec<i64> = INPUT
        .split('\n')
        .filter_map(|l| l.parse::<i64>().ok())
        .collect();

    for n in numbers {
        while list.iter().sum::<i64>() > target {
            list.pop_front();
        }

        if list.iter().sum::<i64>() == target && list.len() > 1 {
            break;
        }

        list.push_back(n);
    }

    list.iter().min().unwrap() + list.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 26134589);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 3535124);
    }
}
