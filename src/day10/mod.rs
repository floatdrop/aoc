static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    let mut numbers: Vec<i64> = INPUT
        .split('\n')
        .filter_map(|l| l.parse::<i64>().ok())
        .collect();

    numbers.sort();

    numbers.push(numbers.last().unwrap() + 3);

    let mut jolts = 0;
    let chain = numbers
        .iter()
        .filter_map(|j| {
            let diff = j - jolts;
            if (0..=3).contains(&diff) {
                jolts += diff;
                return Some(diff);
            }
            None
        })
        .collect::<Vec<i64>>();

    chain.iter().filter(|&&d| d == 1).count() * chain.iter().filter(|&&d| d == 3).count()
}

pub fn part2() -> usize {
    let mut adapters: Vec<i64> = INPUT
        .split('\n')
        .filter_map(|l| l.parse::<i64>().ok())
        .collect();

    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let mut paths = vec![0; adapters.len()];
    paths[0] = 1;

    for i in 1..adapters.len() {
        let adapter = adapters[i];
        let mut count = 0;

        if i >= 3 && adapter - adapters[i - 3] <= 3 {
            count += paths[i - 3];
        }

        if i >= 2 && adapter - adapters[i - 2] <= 3 {
            count += paths[i - 2];
        }

        if i >= 1 && adapter - adapters[i - 1] <= 3 {
            count += paths[i - 1];
        }

        paths[i] = count;
    }

    paths[adapters.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 1984);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 3543369523456);
    }
}
