use std::str::FromStr;
use std::collections::HashMap;
use crate::aoc_error::AocError;

pub const NAME: &str = "Docking Data";

#[derive(Debug)]
struct MaskedAddresses {
    base: u64,
    floats: Vec<usize>,
    n: usize
}

impl Iterator for MaskedAddresses {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= 2usize.pow(self.floats.len() as u32) {
            None
        } else {
            let mut val = self.base;

            for i in 0..self.floats.len() {
                if (self.n & (1 << i)) != 0 {
                    val += 1 << self.floats[i];
                }
            }

            self.n += 1;

            Some(val)
        }
    }
}

trait Bitmask {
    fn apply_val(&self, x: u64) -> u64;
    fn apply_mem(&self, x: u64) -> MaskedAddresses;
}

#[derive(Debug, Copy, Clone)]
struct ValueBitmask {
    ones: u64,
    zeros: u64
}

impl FromStr for ValueBitmask {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones = 0;
        let mut zeros = 0;
        for (i, c) in s.chars().rev().enumerate() {
            if c == '1' {
                ones += 1 << i;
                zeros += 1 << i;
            } else if c == 'X' {
                zeros += 1 << i;
            }
        }

        Ok(ValueBitmask { ones, zeros })
    }
}

impl Bitmask for ValueBitmask {
    fn apply_val(&self, x: u64) -> u64 {
        (x | self.ones) & self.zeros
    }

    fn apply_mem(&self, x: u64) -> MaskedAddresses {
        MaskedAddresses {
            base: x,
            floats: vec![],
            n: 0
        }
    }
}

#[derive(Debug, Clone)]
struct MemBitmask {
    ones: u64,
    floats: Vec<usize>
}

impl FromStr for MemBitmask {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let floats = s
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| c == &'X')
            .map(|(i, _)| i)
            .collect();

        let mut ones = 0;
        for (i, c) in s.chars().rev().enumerate() {
            if c == '1' {
                ones += 1 << i;
            }
        }

        Ok(MemBitmask { ones, floats })
    }
}

impl Bitmask for MemBitmask {
    fn apply_val(&self, x: u64) -> u64 {
        x
    }

    fn apply_mem(&self, x: u64) -> MaskedAddresses {
        let mask = u64::MAX;

        let mut base = x | self.ones;
        for bit in &self.floats {
            base = base & (mask - (1 << bit));
        }

        MaskedAddresses {
            base,
            floats: self.floats.clone(),
            n: 0
        }
    }
}

#[derive(Debug)]
enum Instruction<T: FromStr<Err = AocError>> {
    Mask(T),
    Mem(u64, u64)
}

impl<T: FromStr<Err = AocError>> FromStr for Instruction<T> {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Instruction::Mask(s[7..].parse()?))
        } else if s.starts_with("mem") {
            let tokens: Vec<&str> = s.split(&['[', ']'][..]).collect();

            if tokens.len() != 3 {
                Err(AocError::Misc("Invalid mem instruction".to_string()))
            } else {
                let addr = tokens[1].parse()?;
                let value = tokens[2][3..].parse()?;
                Ok(Instruction::Mem(addr, value))
            }
        } else {
            Err(AocError::Misc("Invalid instruction".to_string()))
        }
    }
}

#[derive(Debug)]
struct Machine<T: Bitmask + Clone + FromStr<Err = AocError>> {
    instructions: Vec<Instruction<T>>,
    pointer: usize,
    memory: HashMap<u64, u64>,
    mask: T
}

impl<T: Bitmask + Clone + FromStr<Err = AocError>> FromStr for Machine<T> {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Instruction<T>>, AocError>>()?;

        Ok(Machine {
            instructions,
            pointer: 0,
            memory: HashMap::new(),
            // This mask will get immediately overwritten. It might be more
            // correct to have this be an Option<Bitmask> to represent that
            // there is no mask before we run the program, but that's a lot of
            // hassle for something we don't need to check.
            mask: T::from_str("")?
        })
    }
}

impl<T: Bitmask + Clone + FromStr<Err = AocError>> Machine<T> {
    fn step(&mut self) {
        if self.pointer >= self.instructions.len() {
            return;
        }

        match &self.instructions[self.pointer] {
            Instruction::Mask(bitmask) => self.mask = bitmask.clone(),
            Instruction::Mem(addr, value) => {
                for masked_addr in self.mask.apply_mem(*addr) {
                    self.memory.insert(masked_addr, self.mask.apply_val(*value));
                }
            }
        }

        self.pointer += 1;
    }

    fn eval(&mut self) {
        while self.pointer < self.instructions.len() {
            self.step();
        }
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut machine: Machine<ValueBitmask> = input.parse()?;
    machine.eval();

    let sum: u64 = machine.memory.values().sum();

    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut machine: Machine<MemBitmask> = input.parse()?;
    machine.eval();

    let sum: u64 = machine.memory.values().sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_bitmask_parse() {
        let s = "1101000X0110100X1X1X00001010XX00X0X0";
        let bitmask: ValueBitmask = s.parse().unwrap();

        let one_str = format!("{:b}", bitmask.ones);
        let zero_str = format!("{:b}", bitmask.zeros);

        assert_eq!(one_str, s.replace("X", "0"));
        assert_eq!(zero_str, s.replace("X", "1"));
    }

    #[test]
    fn test_value_bitmask_apply() {
        let s = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let bitmask: ValueBitmask = s.parse().unwrap();

        assert_eq!(bitmask.apply_val(11), 73);
    }

    #[test]
    fn test_mem_bitmask_parse() {
        let s = "000000000000000000000000000000X1001X";
        let bitmask: MemBitmask = s.parse().unwrap();

        assert_eq!(bitmask.floats, vec![0, 5]);
    }

    #[test]
    fn test_mem_bitmask_apply() {
        let s = "000000000000000000000000000000X1001X";
        let bitmask: MemBitmask = s.parse().unwrap();

        let addresses: Vec<u64> = bitmask.apply_mem(42).collect();
        assert_eq!(addresses, vec![ 26, 27, 58, 59 ]);
    }
}