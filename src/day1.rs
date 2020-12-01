extern crate itertools;

use itertools::Itertools;

static INPUT: &str = std::include_str!("day1.txt");

pub fn part1() -> i64 {
    let numbers = INPUT.lines()
        .map(|l| l.parse::<i64>().unwrap())
        .filter(|&i| i <= 2020)
        .collect::<Vec<i64>>();

    let pair = numbers.iter().tuple_combinations().filter(|&(a, b)| a + b == 2020).next().unwrap();

    pair.0 * pair.1
}

pub fn part2() -> i64 {
    let numbers = INPUT.lines()
        .map(|l| l.parse::<i64>().unwrap())
        .filter(|&i| i <= 2020)
        .collect::<Vec<i64>>();

    let tuple = numbers.iter().tuple_combinations().filter(|&(a, b, c)| a + b + c == 2020).next().unwrap();

    tuple.0 * tuple.1 * tuple.2
}
