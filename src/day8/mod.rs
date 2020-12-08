use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

static INPUT: &str = std::include_str!("input.txt");

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i64),
    Jmp(i64),
    Acc(i64),
}

#[derive(Debug, Clone)]
struct ParseInstructionError;

impl From<ParseIntError> for ParseInstructionError {
    fn from(_: ParseIntError) -> ParseInstructionError {
        ParseInstructionError {}
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code, argument) = s
            .split(' ')
            .take(2)
            .collect_tuple::<(_, _)>()
            .ok_or(ParseInstructionError)?;

        let x = argument.parse::<i64>()?;

        match code {
            "nop" => Ok(Instruction::Nop(x)),
            "jmp" => Ok(Instruction::Jmp(x)),
            "acc" => Ok(Instruction::Acc(x)),
            _ => Err(ParseInstructionError),
        }
    }
}

type Program = Vec<Instruction>;

enum ExitMode {
    RevisitedInstruction,
    EndOfProgram,
}

fn execute_programm(program: &Program) -> (ExitMode, i64) {
    let mut ip = 0;
    let mut accumulator: i64 = 0;
    let mut visited = vec![false; program.len()];

    loop {
        if visited[ip] {
            return (ExitMode::RevisitedInstruction, accumulator);
        }
        visited[ip] = true;
        match program[ip] {
            Instruction::Nop(_) => ip += 1,
            Instruction::Jmp(x) => ip = ((ip as i64) + x) as usize,
            Instruction::Acc(x) => {
                accumulator += x;
                ip += 1;
            }
        }

        if ip >= program.len() {
            return (ExitMode::EndOfProgram, accumulator);
        }
    }
}

pub fn part1() -> i64 {
    let program = INPUT.split('\n').filter_map(|l| l.parse().ok()).collect();
    execute_programm(&program).1
}

pub fn part2() -> i64 {
    let program: Program = INPUT.split('\n').filter_map(|l| l.parse().ok()).collect();

    (0..program.len())
        .find_map(|index| {
            let mut patched = program.clone();
            patched[index] = match patched[index] {
                Instruction::Nop(x) => Instruction::Jmp(x),
                Instruction::Jmp(x) => Instruction::Nop(x),
                Instruction::Acc(_) => {
                    return None;
                }
            };

            match execute_programm(&patched) {
                (ExitMode::RevisitedInstruction, _) => None,
                (ExitMode::EndOfProgram, i) => Some(i),
            }
        })
        .unwrap()
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
