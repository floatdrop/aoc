extern crate itertools;

use itertools::Itertools;

static INPUT: &str = std::include_str!("day1.txt");

pub fn part1() -> i64 {
    let numbers = INPUT.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let pair = numbers.iter().tuple_combinations().filter(|&(a, b)| a + b == 2020).next().unwrap();

    pair.0 * pair.1
}

pub fn part2() -> i64 {
    let numbers = INPUT.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>(); 
    let tuple = numbers.iter().tuple_combinations().filter(|&(a, b, c)| a + b + c == 2020).next().unwrap();

    tuple.0 * tuple.1 * tuple.2
}

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1_opt() -> i64 {
    let numbers = INPUT.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let set = HashSet::<i64>::from_iter(numbers.iter().cloned());

    for number in numbers {
        let diff = 2020 - number;
        if set.contains(&diff) {
            return number * diff;   
        }
    }
    
    unreachable!();
}

pub fn part2_opt() -> i64 {
    let numbers = INPUT.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let set = HashSet::<i64>::from_iter(numbers.iter().cloned());

    for (i, a) in numbers.iter().enumerate() {
        for b in numbers[i + 1..].iter() {
            let c = 2020 - (a + b);
            if set.contains(&c) {
                return a * b * c;   
            }
        }
    }
    
    unreachable!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 692916);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 289270976);
    }

    #[test]
    fn check_part1_opt_answer() {
        assert_eq!(part1_opt(), 692916);
    }

    #[test]
    fn check_part2_opt_answer() {
        assert_eq!(part2_opt(), 289270976);
    }
}
