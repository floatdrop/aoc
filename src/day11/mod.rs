use itertools::{iterate, Itertools};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

static INPUT: &str = std::include_str!("input.txt");

#[derive(Debug)]
enum Direction {
    Inc,
    Dec,
    Nop,
}

impl Direction {
    fn next(&self, x: usize) -> usize {
        match self {
            Direction::Inc => x + 1,
            Direction::Dec => x.wrapping_sub(1),
            Direction::Nop => x,
        }
    }
}

static DIRECTIONS: [(Direction, Direction); 8] = [
    (Direction::Dec, Direction::Dec),
    (Direction::Nop, Direction::Dec),
    (Direction::Inc, Direction::Dec),
    (Direction::Dec, Direction::Nop),
    (Direction::Inc, Direction::Nop),
    (Direction::Dec, Direction::Inc),
    (Direction::Nop, Direction::Inc),
    (Direction::Inc, Direction::Inc),
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Place {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl From<&char> for Place {
    fn from(c: &char) -> Self {
        match c {
            '.' => Place::Floor,
            'L' => Place::EmptySeat,
            '#' => Place::OccupiedSeat,
            _ => unreachable!(),
        }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Place::Floor => ".",
                Place::EmptySeat => "L",
                Place::OccupiedSeat => "#",
            }
        )
    }
}

struct Grid {
    map: Vec<Vec<Place>>,
}

impl Grid {
    fn new(map: Vec<Vec<Place>>) -> Self {
        Self { map }
    }

    fn adjacent_seats(&self, x: usize, y: usize) -> Vec<&Place> {
        DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| {
                self.map
                    .get(dy.next(y))
                    .and_then(|row| row.get(dx.next(x)))
            })
            .collect()
    }

    fn visible_seats(&self, x: usize, y: usize) -> Vec<&Place> {
        DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| {
                iterate((dx.next(x), dy.next(y)), |&(x, y)| (dx.next(x), dy.next(y)))
                    .map(|(x, y)| self.map.get(y).and_then(|row| row.get(x)))
                    .while_some()
                    .find(|&p| p != &Place::Floor)
            })
            .collect()
    }
}

#[derive(Debug)]
struct ParseGridError {}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid::new(
            s.lines()
                .map(|l| l.chars().map(|c| Place::from(&c)).collect())
                .collect(),
        ))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .map(|row| row.iter().map(|p| p.to_string()).join(""))
                .join("\n")
        )
    }
}

pub fn part1() -> usize {
    let mut grid = Grid::from_str(INPUT).unwrap();

    loop {
        let new_grid = Grid::new(
            grid.map
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, p)| match p {
                            Place::EmptySeat => {
                                if grid
                                    .adjacent_seats(x, y)
                                    .iter()
                                    .all(|&p| p != &Place::OccupiedSeat)
                                {
                                    Place::OccupiedSeat
                                } else {
                                    Place::EmptySeat
                                }
                            }
                            Place::OccupiedSeat => {
                                if grid
                                    .adjacent_seats(x, y)
                                    .iter()
                                    .filter(|&p| *p == &Place::OccupiedSeat)
                                    .count()
                                    > 3
                                {
                                    Place::EmptySeat
                                } else {
                                    Place::OccupiedSeat
                                }
                            }
                            Place::Floor => Place::Floor,
                        })
                        .collect()
                })
                .collect(),
        );

        if grid == new_grid {
            break;
        }

        grid = new_grid;
    }

    grid.map
        .iter()
        .map(|row| row.iter().filter(|&p| p == &Place::OccupiedSeat))
        .flatten()
        .count()
}

pub fn part2() -> usize {
    let mut grid = Grid::from_str(INPUT).unwrap();

    loop {
        let new_grid = Grid::new(
            grid.map
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, p)| match p {
                            Place::EmptySeat => {
                                if grid
                                    .visible_seats(x, y)
                                    .iter()
                                    .all(|&p| p != &Place::OccupiedSeat)
                                {
                                    Place::OccupiedSeat
                                } else {
                                    Place::EmptySeat
                                }
                            }
                            Place::OccupiedSeat => {
                                if grid
                                    .visible_seats(x, y)
                                    .iter()
                                    .filter(|&p| *p == &Place::OccupiedSeat)
                                    .count()
                                    > 4
                                {
                                    Place::EmptySeat
                                } else {
                                    Place::OccupiedSeat
                                }
                            }
                            Place::Floor => Place::Floor,
                        })
                        .collect()
                })
                .collect(),
        );

        if grid == new_grid {
            break;
        }

        grid = new_grid;
    }

    grid.map
        .iter()
        .map(|row| row.iter().filter(|&p| p == &Place::OccupiedSeat))
        .flatten()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 2194);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 1944);
    }
}
