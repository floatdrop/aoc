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

    println!("{:?}", ids);

    let mut offset: usize = 0;
    loop {
        let mismatch = ids
            .iter()
            .filter(|(t, id)| (offset + t) % id != 0)
            .max_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
        if let Some(b) = mismatch {
            offset = (((offset + b.0) / b.1 + 1) * b.1) - b.0;
            continue;
        }
        break;
    }

    offset
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
