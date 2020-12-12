use std::{
    ops::{Add, AddAssign, Mul},
    unreachable,
};

static INPUT: &str = std::include_str!("input.txt");

#[derive(Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
}

type Direction = Position;

fn north(n: i64) -> Direction {
    Direction { x: 0, y: -n }
}

fn south(n: i64) -> Direction {
    Direction { x: 0, y: n }
}

fn west(n: i64) -> Direction {
    Direction { x: -n, y: 0 }
}

fn east(n: i64) -> Direction {
    Direction { x: n, y: 0 }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Direction {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Direction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Direction {
    fn rotate(&mut self, deg: i64) {
        let rad = (deg as f64).to_radians();
        *self = Self {
            x: self.x * (rad.cos() as i64) - self.y * (rad.sin() as i64),
            y: self.x * (rad.sin() as i64) + self.y * (rad.cos() as i64),
        }
    }
}

#[derive(Clone, Debug)]
struct Ship {
    pos: Position,
    dir: Direction,
}

pub fn part1() -> i64 {
    let ship = INPUT
        .lines()
        .map(|l| {
            let (c, n) = l.split_at(1);
            (c, n.parse::<i64>().unwrap())
        })
        .fold(
            Ship {
                pos: Position { x: 0, y: 0 },
                dir: east(1),
            },
            |mut ship, (c, n)| {
                match c {
                    "L" => ship.dir.rotate(-n),
                    "R" => ship.dir.rotate(n),
                    "F" => ship.pos += ship.dir.clone() * Direction { x: n, y: n },
                    "N" => ship.pos += north(n),
                    "S" => ship.pos += south(n),
                    "W" => ship.pos += west(n),
                    "E" => ship.pos += east(n),
                    _ => unreachable!(),
                };
                ship
            },
        );

    ship.pos.x.abs() + ship.pos.y.abs()
}

pub fn part2() -> i64 {
    let ship = INPUT
        .lines()
        .map(|l| {
            let (c, n) = l.split_at(1);
            (c, n.parse::<i64>().unwrap())
        })
        .fold(
            Ship {
                pos: Position { x: 0, y: 0 },
                dir: east(10) + north(1),
            },
            |mut ship, (c, n)| {
                match c {
                    "L" => ship.dir.rotate(-n),
                    "R" => ship.dir.rotate(n),
                    "F" => ship.pos += ship.dir.clone() * Direction { x: n, y: n },
                    "N" => ship.dir += north(n),
                    "S" => ship.dir += south(n),
                    "W" => ship.dir += west(n),
                    "E" => ship.dir += east(n),
                    _ => unreachable!(),
                };
                ship
            },
        );

    ship.pos.x.abs() + ship.pos.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 362);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 29895);
    }
}
