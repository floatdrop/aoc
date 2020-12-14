use bitvec::prelude::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = std::include_str!("input.txt");

#[derive(Debug, Clone, PartialEq)]
pub enum Mask {
    One,
    Zero,
    X,
}

type BitMask = Vec<Mask>;

#[derive(Debug, Clone)]
pub struct MemInstruction {
    address: usize,
    value: usize,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mask(BitMask),
    Mem(MemInstruction),
}

peg::parser! {
  grammar program() for str {
    rule number() -> usize
      = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule mask() -> BitMask
      = m:$(['X' | '0' | '1']+) {
        m.chars().map(|c| match c {
            'X' => Mask::X,
            '0' => Mask::Zero,
            '1' => Mask::One,
            _ => unreachable!()
        }).collect()
      }

    rule instruction() -> Instruction
      = "mask = " m:mask() { Instruction::Mask(m) }
      / "mem[" i:number() "] = " v:number() { Instruction::Mem(MemInstruction{ address: i, value: v }) }

    pub rule instructions() -> Vec<Instruction>
      = ops:instruction() ++ "\n" {
        ops.iter().cloned().collect::<Vec<_>>()
      }
  }
}

pub fn part1() -> usize {
    let mut memory = HashMap::<usize, usize>::new();
    let mut mask = BitMask::new();
    for ins in program::instructions(INPUT).unwrap() {
        match ins {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(m) => {
                let o = m.value.view_bits::<Msb0>();

                let v = &o[o.len() - 36..]
                    .iter()
                    .zip(mask.iter())
                    .map(|(b, maskbit)| match maskbit {
                        Mask::X => b,
                        Mask::Zero => &false,
                        Mask::One => &true,
                    })
                    .collect::<BitVec<Msb0>>();
                memory.insert(m.address, v.load::<usize>());
            }
        }
    }

    memory.iter().map(|(_, v)| v).sum::<usize>()
}

pub fn part2() -> usize {
    let mut memory = HashMap::<usize, usize>::new();
    let mut mask = BitMask::new();
    for ins in program::instructions(INPUT).unwrap() {
        match ins {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(m) => {
                let o = m.address.view_bits::<Msb0>();
                let floating_bits_positions: Vec<usize> =
                    mask.iter().positions(|m| m == &Mask::X).collect();

                for enabled_floating_bits in (0..=floating_bits_positions.len())
                    .map(|count| floating_bits_positions.iter().combinations(count))
                    .flatten()
                    .collect::<HashSet<_>>()
                {
                    let address: BitVec<Msb0> = o[o.len() - 36..]
                        .iter()
                        .zip(mask.iter().enumerate())
                        .map(|(b, (e, maskbit))| match maskbit {
                            Mask::X => enabled_floating_bits.contains(&&e),
                            Mask::Zero => *b,
                            Mask::One => true,
                        })
                        .collect();

                    memory.insert(address.load::<usize>(), m.value);
                }
            }
        }
    }

    memory.iter().map(|(_, v)| v).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 14925946402938);
    }

    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 3706820676200);
    }
}
