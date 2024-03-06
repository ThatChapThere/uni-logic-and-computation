use std::fmt;
use std::fs;
use crate::operation::Operation;
use crate::Path;

#[derive(Default)]
pub struct Tape {
    pub current: bool,
    before: Vec<bool>,
    after: Vec<bool>,
}

impl Tape {
    pub fn operate(&mut self, operation: Operation) {
        match operation {
            Operation::Left => {
                self.after.push(self.current);
                self.current = self.before.pop().unwrap_or_default();
            },
            Operation::Right => {
                self.before.push(self.current);
                self.current = self.after.pop().unwrap_or_default();
            },
            Operation::Off => self.current = false,
            Operation::On => self.current = true,
        }
    }

    pub fn new(filename: &str) -> Tape {
        let mut tape: Tape = Default::default();
        let text = fs::read_to_string(filename).expect("the tape file should exist");
        let text = text
            .chars()
            .filter(|c| matches!(c, '1'|'0'))
            .rev();
        for (i, c) in text.enumerate() {
            if i != 0 {
                tape.operate(Operation::Left);
            }
            if c == '1' {
                tape.operate(Operation::On);
            }
        }
        return tape;
    }
}

fn bit_char(b: &bool) -> char {
    if *b {'1'} else {'0'}
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = String::from("");
        for b in self.before.iter() {
            disp.push(' ');
            disp.push(bit_char(b));
        }
        disp.push_str(&format!("[{}]", bit_char(&self.current)));
        for b in self.after.iter().rev() {
            disp.push(bit_char(b));
            disp.push(' ');
        }
        write!(f, "{}", disp)
    }
}
