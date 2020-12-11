use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    str::FromStr,
};

static INPUT: &str = std::include_str!("input.txt");

static DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
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

type Position = (i64, i64);

struct Grid {
    map: HashMap<Position, Place>,
    width: i64,
    height: i64,
}

impl Grid {
    fn new(map: HashMap<Position, Place>) -> Self {
        let w = map.iter().map(|((x, _), _)| *x).max().unwrap_or(0);
        let h = map.iter().map(|((_, y), _)| *y).max().unwrap_or(0);
        Self {
            map,
            width: w + 1,
            height: h + 1,
        }
    }

    fn adjacent_seats(&self, x: i64, y: i64) -> Vec<&Place> {
        DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| self.map.get(&(x - dx, y - dy)))
            .collect()
    }

    fn visible_seats(&self, sx: i64, sy: i64) -> Vec<&Place> {
        DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| {
                let mut x = sx;
                let mut y = sy;
                while x >= 0 && x < self.width && y >= 0 && y < self.height {
                    x += dx;
                    y += dy;

                    if let Some(p) = self.map.get(&(x, y)) {
                        if p != &Place::Floor {
                            return Some(p);
                        }
                    } else {
                        return None;
                    }
                }

                return None;
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
                .enumerate()
                .map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(move |(x, c)| ((x as i64, y as i64), Place::from(&c)))
                    // .filter(|(_, p)| p != &Place::Floor)
                })
                .flatten()
                .collect(),
        ))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            (0..self.height)
                .map(|y| (0..self.width)
                    .map(|x| self.map.get(&(x, y)).unwrap_or(&Place::Floor).to_string())
                    .join(""))
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
                .map(|((x, y), p)| {
                    (
                        (*x, *y),
                        match p {
                            Place::EmptySeat => {
                                if grid
                                    .adjacent_seats(*x, *y)
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
                                    .adjacent_seats(*x, *y)
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
                        },
                    )
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
        .filter(|&(_, p)| p == &Place::OccupiedSeat)
        .count()
}

pub fn part2() -> usize {
    let mut grid = Grid::from_str(INPUT).unwrap();

    loop {
        // println!("{}\n", grid);
        let new_grid = Grid::new(
            grid.map
                .iter()
                .map(|((x, y), p)| {
                    (
                        (*x, *y),
                        match p {
                            Place::EmptySeat => {
                                if grid
                                    .visible_seats(*x, *y)
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
                                    .visible_seats(*x, *y)
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
                        },
                    )
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
        .filter(|&(_, p)| p == &Place::OccupiedSeat)
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
