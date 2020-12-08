use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Handheld Halting";

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Acc,
    Jump,
    Noop
}

use Opcode::*;

struct Instruction {
    opcode: Opcode,
    arg: isize,
    visited: bool
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((opcode, arg)) = s.split_once(" ") {
            let opcode = match opcode {
                "acc" => Ok(Acc),
                "jmp" => Ok(Jump),
                "nop" => Ok(Noop),
                _ => Err(AocError::Misc("Bad opcode".to_string()))
            }?;

            Ok(Instruction {
                opcode,
                arg: arg.parse()?,
                visited: false
            })
        } else {
            Err(AocError::Misc("Bad instruction".to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ExitReason {
    InfiniteLoop,
    Completed
}

use ExitReason::*;

#[derive(Debug, PartialEq, Eq)]
enum MachineState {
    Running,
    Exited
}

use MachineState::*;

struct Machine {
    instructions: Vec<Instruction>,
    pointer: usize,
    accumulator: isize,
    state: MachineState,
    exit_reason: Option<ExitReason>
}

impl FromStr for Machine {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Instruction>, AocError>>()?;

        Ok(Machine {
            instructions,
            pointer: 0,
            accumulator: 0,
            state: Running,
            exit_reason: None
        })
    }
}

impl Machine {
    fn step(&mut self) {
        if self.state == Exited {
            return;
        }

        if self.pointer >= self.instructions.len() {
            self.state = Exited;
            self.exit_reason = Some(Completed);
            return;
        }

        let mut instruction = &mut self.instructions[self.pointer];

        if instruction.visited {
            self.state = Exited;
            self.exit_reason = Some(InfiniteLoop);
            return;
        }

        match instruction.opcode {
            Acc => self.accumulator += instruction.arg,
            Jump => {
                // Subtract one here to compensate for the unconditional
                // pointer increment below.
                let pointer = self.pointer as isize + instruction.arg - 1;
                self.pointer = pointer as usize;
            }
            Noop => { }
        };

        instruction.visited = true;
        self.pointer += 1;
    }

    fn eval(&mut self) {
        loop {
            self.step();
            if self.state == Exited {
                break;
            }
        }
    }

    fn reset(&mut self) {
        for instruction in self.instructions.iter_mut() {
            instruction.visited = false;
        }

        self.accumulator = 0;
        self.pointer = 0;
        self.state = Running;
        self.exit_reason = None;
    }

    fn swap_opcode_at(&mut self, line: usize) {
        let mut instruction = &mut self.instructions[line];

        match instruction.opcode {
            Acc => {},
            Jump => instruction.opcode = Noop,
            Noop => instruction.opcode = Jump
        }
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut machine: Machine = input.parse()?;
    machine.eval();

    Ok(machine.accumulator.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut machine: Machine = input.parse()?;

    // Lets brute force our way through all possible instructions
    // I'm pretty sure you could solve this statically, but this runs in
    // less than 500us on my machine. so eh.
    for line in 0..machine.instructions.len() {
        if machine.instructions[line].opcode != Acc {
            machine.swap_opcode_at(line);
            machine.eval();

            if machine.exit_reason == Some(Completed) {
                return Ok(machine.accumulator.to_string());
            }

            machine.reset();
            machine.swap_opcode_at(line);
        }
    }

    Err(AocError::Misc("No opcode swap exits".to_string()))
}