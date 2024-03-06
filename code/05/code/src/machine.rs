use std::fs;
use std::ops::Index;
use std::ops::IndexMut;
use crate::operation::Operation;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub state: usize,
}

#[derive(Default)]
#[derive(Debug)]
pub struct State {
    off: Option<Instruction>, // Options so that None can be used to indicate halt
    on: Option<Instruction>,
}

pub struct Machine {
    pub states: Vec<State>,
    pub state: usize,
}

impl Machine {
    pub fn new(filename: &str) -> Machine {
        let text = fs::read_to_string(filename).expect("the machine file should exist");
        let mut states: Vec<State> = vec![];
        for line in text.split('\n') {
            let line: Vec<&str> = line.split(':').collect();
            if line.len() < 2 { continue; } // Ignore lines with no colon
            // Get the number of the state described on this line
            let state_number: usize = match line[0].parse::<usize>() {
                Ok(n) => if n > 0 {n-1} else {continue},
                _ => continue,
            };
            // Resize the finite state states if necessary
            if state_number >= states.len() {
                states.resize_with(state_number + 1, Default::default);
            }
            let instructions: Vec<&str> = line[1].split(',').collect();
            for i in 0..=1 {
                if i >= instructions.len() {continue;}
                let mut operation: Option<Operation> = None;
                let mut target_state_number: Option<usize> = None;
                // Try to find instructions
                for instruction in instructions[i].split(' ') {
                    // Try to find operations
                    if let Some(o) = match instruction {
                        "on" => Some(Operation::On),
                        "off" => Some(Operation::Off),
                        "left" => Some(Operation::Left),
                        "right" => Some(Operation::Right),
                        _ => None,
                    } {
                        operation = Some(o);
                    }
                    // Try to find target states
                    if let Some(s) = match instruction.parse::<usize>() {
                        Ok(n) => if n > 0 {Some(n-1)} else {None},
                        _ => None,
                    } {
                        target_state_number = Some(s);
                    }
                }
                // If we have both, add the instruction
                match (operation, target_state_number) {
                    (Some(o), Some(s)) => states[state_number][i==1] = Some(Instruction {operation: o, state: s}),
                    _ => (),
                }
            }
        }
        return Machine{states: states, state: 0};
    }
}

impl Index<bool> for State {
    type Output = Option<Instruction>;
    fn index(&self, tape_value: bool) -> &Self::Output {
        if tape_value {
            &self.on
        } else {
            &self.off
        }
    }
}

impl IndexMut<bool> for State {
    fn index_mut(&mut self, tape_value: bool) -> &mut Self::Output {
        if tape_value {
            &mut self.on
        } else {
            &mut self.off
        }
    }
}
