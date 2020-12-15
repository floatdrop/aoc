use std::collections::HashMap;

static INPUT: &str = std::include_str!("test.txt");

peg::parser! {
    grammar aoc15() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }
        pub rule list() -> Vec<usize>
          = l:number() ** "," { l }
      }
}

pub fn solve(n: usize) -> usize {
    let mut numbers = aoc15::list(INPUT).unwrap();
    let mut spoken: HashMap<usize, [usize; 2]> = numbers
        .iter()
        .enumerate()
        .map(|(p, n)| (*n, [p + 1, p + 1]))
        .collect();
    while numbers.len() < n {
        let last = numbers.last().unwrap();
        if let Some(last_times) = spoken.get(&last) {
            if last_times[0] == last_times[1] {
                numbers.push(0);
                if let Some(last_times_new) = spoken.get(&0) {
                    let ln = last_times_new[1];
                    spoken.insert(0, [ln, numbers.len()]);
                } else {
                    spoken.insert(0, [numbers.len(), numbers.len()]);
                }
            } else {
                let d = last_times[1] - last_times[0];
                numbers.push(d);
                if let Some(last_times_new) = spoken.get(&d) {
                    let ln = last_times_new[1];
                    spoken.insert(d, [ln, numbers.len()]);
                } else {
                    spoken.insert(d, [numbers.len(), numbers.len()]);
                }
            }
        } else {
            numbers.push(0);
            spoken.insert(0, [numbers.len(), numbers.len()]);
        }
    }
    numbers[n - 1]
}

pub fn part1() -> usize {
    solve(2020)
}

pub fn part2() -> usize {
    solve(30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 614);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 1065);
    }
}
