use std::collections::HashSet;

static INPUT: &str = std::include_str!("input.txt");

#[derive(Debug)]
pub enum OpCode {
    Nop,
    Acc,
    Jmp,
}

type Programm = Vec<(OpCode, i64)>;

pub fn parse() -> Programm {
    INPUT
        .lines()
        .map(|line| {
            let mut p = line.split(" ");

            let op = match p.next() {
                Some(code) => match code {
                    "nop" => OpCode::Nop,
                    "acc" => OpCode::Acc,
                    "jmp" => OpCode::Jmp,
                    _ => unreachable!(),
                },
                None => unreachable!(),
            };

            let arg = match p.next() {
                Some(v) => v.parse::<i64>().unwrap(),
                None => unreachable!(),
            };

            (op, arg)
        })
        .collect()
}

pub fn part1() -> i64 {
    let programm = parse();

    let mut visited: HashSet<usize> = HashSet::new();
    let mut ip = 0;
    let mut reg = 0;
    while ip < programm.len() {
        if visited.contains(&ip) {
            break;
        }
        visited.insert(ip);
        match programm[ip] {
            (OpCode::Acc, x) => {
                reg += x;
                ip += 1;
            }
            (OpCode::Jmp, x) => {
                ip = ((ip as i64) + x) as usize;
            }
            (OpCode::Nop, _) => ip += 1,
        }
    }
    reg
}

pub fn part2() -> i64 {
    let mut programm = parse();

    let mut visited: HashSet<usize> = HashSet::new();
    let mut ip = 0;
    let mut reg = 0;
    let mut mutated = 0;
    match programm[mutated].0 {
        OpCode::Nop => programm[mutated].0 = OpCode::Jmp,
        OpCode::Jmp => programm[mutated].0 = OpCode::Nop,
        OpCode::Acc => {}
    }

    while ip < programm.len() {
        if visited.contains(&ip) {
            reg = 0;
            ip = 0;
            visited.clear();
            match programm[mutated].0 {
                OpCode::Nop => programm[mutated].0 = OpCode::Jmp,
                OpCode::Jmp => programm[mutated].0 = OpCode::Nop,
                OpCode::Acc => {}
            }
            mutated += 1;
            match programm[mutated].0 {
                OpCode::Nop => programm[mutated].0 = OpCode::Jmp,
                OpCode::Jmp => programm[mutated].0 = OpCode::Nop,
                OpCode::Acc => {}
            }
            continue;
        }
        visited.insert(ip);
        match programm[ip] {
            (OpCode::Acc, x) => {
                reg += x;
                ip += 1;
            }
            (OpCode::Jmp, x) => {
                ip = ((ip as i64) + x) as usize;
            }
            (OpCode::Nop, _) => ip += 1,
        }
    }
    reg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 1749);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 515);
    }
}
