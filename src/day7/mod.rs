use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = std::include_str!("input.txt");

type Rules = HashMap<String, Vec<(usize, String)>>;

pub fn find_my_color(rules: &Rules, bag: &str) -> bool {
    if bag == MY_COLOR {
        return true
    }

    match rules.get(bag) {
        Some(contents) => contents.iter()
                                  .map(|(_, color)| find_my_color(rules, color))
                                  .any(|x| x),
        None => false,
    }
}

pub fn count_bags(rules: &Rules, bag: &str) -> usize {
    1 + match rules.get(bag) {
        Some(contents) => contents.iter()
                                  .map(|(count, color)| count * count_bags(rules, color))
                                  .sum(),
        None => 0,
    }
}

pub fn parse() -> Rules {
    INPUT
        .lines()
        .filter_map(|line| line.splitn(2, " bags contain ").collect_tuple::<(_, _)>())
        .map(|(parent, contents)|
            (parent.to_string(), contents
                .split(", ")
                .map(|bag| {
                    let p = bag.split_at(bag.find(" ").unwrap());   
                    (
                        p.0.parse::<usize>().unwrap_or(0),
                        p.1.split(" ").skip(1).take(2).collect::<Vec<_>>().join(" "),
                    )
                })
                .collect::<Vec<_>>()
            )
        )
        .collect()
}

static MY_COLOR: &str = "shiny gold";

pub fn part1() -> usize {
    let rules = parse();
    rules.iter().filter(|(k, _)| k != &MY_COLOR && find_my_color(&rules, k)).count()
}

pub fn part2() -> usize {
    let rules = parse();
    count_bags(&rules, MY_COLOR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 252);
    }
    
    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 35487);
    }
}
