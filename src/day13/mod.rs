static INPUT: &str = std::include_str!("input.txt");

pub fn part1() -> usize {
    let mut l = INPUT.lines();
    let timestamp: usize = l.next().unwrap().parse().unwrap();

    let ids: Vec<usize> = l
        .next()
        .unwrap()
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    let p = ids
        .iter()
        .enumerate()
        .map(|(e, id)| (((timestamp / id + 1) * id), e))
        .min()
        .unwrap();

    (p.0 - timestamp) * ids[p.1]
}

pub fn part2() -> usize {
    let ids: Vec<(usize, usize)> = INPUT
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(t, n)| n.parse::<usize>().ok().and_then(|i| Some((t, i))))
        .collect();

    ids.iter()
        .fold(
            (0, 1),
            |(solution_so_far, product_so_far), (remainder, bus_id)| {
                let mut m = solution_so_far;
                while (m + remainder) % bus_id != 0 {
                    m += product_so_far;
                }
                (m, product_so_far * bus_id)
            },
        )
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 0);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 0);
    }
}
