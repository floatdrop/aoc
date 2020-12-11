use std::collections::HashMap;

static INPUT: &str = std::include_str!("input.txt");

type Grid = HashMap<(i64, i64), Position>;

type Visibility = HashMap<(i64, i64), Vec<(i64, i64)>>;

fn display(grid: &Grid) {
    let w = *grid.iter().map(|((x, _), _)| x).max().unwrap();
    let h = *grid.iter().map(|((_, y), _)| y).max().unwrap();

    for j in 0..=h {
        for i in 0..=w {
            if let Some(p) = grid.get(&(i, j)) {
                match p {
                    Position::Floor => print!("."),
                    Position::Occupied => print!("#"),
                    Position::Empty => print!("L"),
                }
            }
        }
        print!("\n");
    }
}

fn adjastent_seats(grid: &Grid, x: &i64, y: &i64) -> Vec<Position> {
    vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|(i, j)| (x + i, y + j))
    .filter_map(|(x, y)| grid.get(&(x, y)))
    .cloned()
    .collect()
}

fn visible_seats(grid: &Grid, x: &i64, y: &i64) -> Vec<(i64, i64)> {
    vec![
        (0..grid.len())
            .filter_map(|r| grid.get(&(x + r as i64, *y)))
            .find(|p| *p != &Position::Empty),
        (0..grid.len())
            .filter_map(|r| grid.get(&(*x, y + r as i64)))
            .find(|p| *p != &Position::Empty),
        (0..grid.len())
            .filter_map(|r| grid.get(&(x + r as i64, y + r as i64)))
            .find(|p| *p != &Position::Empty),
        (0..grid.len())
            .filter_map(|r| grid.get(&(x - r as i64, y + r as i64)))
            .find(|p| *p != &Position::Empty),
        (0..grid.len())
            .filter_map(|r| grid.get(&(x + r as i64, y - r as i64)))
            .find(|p| *p != &Position::Empty),
        (0..grid.len())
            .filter_map(|r| grid.get(&(x - r as i64, *y)))
            .find(|p| *p != &Position::Empty),
        (0..grid.len())
            .filter_map(|r| grid.get(&(*x, y - r as i64)))
            .find(|p| *p != &Position::Empty),
        (0..grid.len())
            .filter_map(|r| grid.get(&(x - r as i64, y - r as i64)))
            .find(|p| *p != &Position::Empty),
    ]
    .into_iter()
    .filter_map(|x| x)
    .cloned()
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

fn parse() -> Grid {
    INPUT
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| match c {
                '.' => ((x as i64, y as i64), Position::Floor),
                'L' => ((x as i64, y as i64), Position::Empty),
                '#' => ((x as i64, y as i64), Position::Occupied),
                _ => unreachable!(),
            })
        })
        .flatten()
        .collect()
}

fn round(grid: &Grid) -> Grid {
    grid.iter()
        .map(|((x, y), p)| {
            let new_p = match p {
                Position::Occupied => {
                    if adjastent_seats(grid, x, y)
                        .iter()
                        .filter(|s| *s == &Position::Occupied)
                        .count()
                        > 3
                    {
                        Position::Empty
                    } else {
                        Position::Occupied
                    }
                }
                Position::Empty => {
                    if adjastent_seats(grid, x, y)
                        .iter()
                        .all(|s| *s != Position::Occupied)
                    {
                        Position::Occupied
                    } else {
                        Position::Empty
                    }
                }
                Position::Floor => Position::Floor,
            };
            ((x.clone(), y.clone()), new_p)
        })
        .collect()
}

pub fn part1() -> usize {
    let mut grid = parse();

    loop {
        let new_grid = round(&grid);

        if grid == new_grid {
            break;
        }

        grid = new_grid;
    }

    grid.iter()
        .filter(|(_, s)| *s == &Position::Occupied)
        .count()
}

fn round2(grid: &Grid) -> Grid {
    grid.iter()
        .map(|((x, y), p)| {
            let new_p = match p {
                Position::Occupied => {
                    if visible_seats(grid, x, y)
                        .iter()
                        .filter(|s| *s == &Position::Occupied)
                        .count()
                        > 4
                    {
                        Position::Empty
                    } else {
                        Position::Occupied
                    }
                }
                Position::Empty => {
                    if visible_seats(grid, x, y)
                        .iter()
                        .all(|s| *s != Position::Occupied)
                    {
                        Position::Occupied
                    } else {
                        Position::Empty
                    }
                }
                Position::Floor => Position::Floor,
            };
            ((x.clone(), y.clone()), new_p)
        })
        .collect()
}

pub fn part2() -> usize {
    let mut grid = parse();

    display(&grid);
    print!("\n");

    loop {
        let new_grid = round2(&grid);

        display(&new_grid);
        print!("\n");

        if grid == new_grid {
            break;
        }

        grid = new_grid;
    }

    grid.iter()
        .filter(|(_, s)| *s == &Position::Occupied)
        .count()
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
