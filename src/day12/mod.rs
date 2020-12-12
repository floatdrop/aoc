use std::unreachable;

static INPUT: &str = std::include_str!("input.txt");

type Position = (i64, i64);

#[derive(Debug)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn next(&self, pos: Position, n: i64) -> Position {
        match self {
            Direction::N => (pos.0, pos.1 - n),
            Direction::S => (pos.0, pos.1 + n),
            Direction::W => (pos.0 - n, pos.1),
            Direction::E => (pos.0 + n, pos.1),
        }
    }
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "N" => Direction::N,
            "S" => Direction::S,
            "W" => Direction::W,
            "E" => Direction::E,
            _ => unreachable!(),
        }
    }
}

impl From<i64> for Direction {
    fn from(deg: i64) -> Self {
        let norm = (deg % 360) + if deg < 0 { 360 } else { 0 };
        match norm {
            0 => Direction::N,
            90 => Direction::E,
            180 => Direction::S,
            270 => Direction::W,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for i64 {
    fn from(d: Direction) -> i64 {
        match d {
            Direction::N => 0,
            Direction::E => 90,
            Direction::S => 180,
            Direction::W => 270,
        }
    }
}

struct Ship {
    pos: Position,
    wyp: Position,
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
                pos: (0, 0),
                wyp: (0, 0),
                dir: Direction::E,
            },
            |mut ship, (c, n)| {
                match c {
                    "L" => ship.dir = Direction::from(i64::from(ship.dir) - n),
                    "R" => ship.dir = Direction::from(i64::from(ship.dir) + n),
                    "F" => ship.pos = ship.dir.next(ship.pos, n),
                    "N" | "S" | "W" | "E" => ship.pos = Direction::from(c).next(ship.pos, n),
                    _ => unreachable!(),
                };
                ship
            },
        );

    ship.pos.0.abs() + ship.pos.1.abs()
}

fn rotate(wp: Position, deg: f64) -> Position {
    (
        wp.0 * (deg.to_radians().cos() as i64) - wp.1 * (deg.to_radians().sin() as i64),
        wp.0 * (deg.to_radians().sin() as i64) + wp.1 * (deg.to_radians().cos() as i64),
    )
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
                pos: (0, 0),
                wyp: Direction::N.next(Direction::E.next((0, 0), 10), 1),
                dir: Direction::E,
            },
            |mut ship, (c, n)| {
                match c {
                    "L" => ship.wyp = rotate(ship.wyp, -n as f64),
                    "R" => ship.wyp = rotate(ship.wyp, n as f64),
                    "F" => ship.pos = (ship.pos.0 + ship.wyp.0 * n, ship.pos.1 + ship.wyp.1 * n),
                    "N" | "S" | "W" | "E" => ship.wyp = Direction::from(c).next(ship.wyp, n),
                    _ => unreachable!(),
                };
                println!("{}{}\tpos:{} {}", c, n, ship.pos.0, ship.pos.1);
                ship
            },
        );

    ship.pos.0.abs() + ship.pos.1.abs()
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
