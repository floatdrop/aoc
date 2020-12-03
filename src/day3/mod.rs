static INPUT: &str = std::include_str!("input.txt");

pub fn count_trees(right: usize, down: usize) -> usize {
    let rows: Vec<&str> = INPUT.lines().collect();
    
    let width = rows.iter().next().unwrap().len();
    let height = rows.len();

    let mut count = 0;

    for (i, y) in (0..height).step_by(down).enumerate() {
        let x = i * right % width;
        if rows[y].chars().nth(x).unwrap() == '#' {
            count += 1;
        }
    }

    count
}

pub fn part1() -> usize {
    count_trees(3, 1)
}

pub fn part2() -> usize {
    count_trees(1, 1) * 
    count_trees(3, 1) * 
    count_trees(5, 1) * 
    count_trees(7, 1) * 
    count_trees(1, 2) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 265);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 3154761400);
    }
}