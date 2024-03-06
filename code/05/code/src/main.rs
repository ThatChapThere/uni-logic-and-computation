use std::path::Path;
use std::fs::File;
mod operation;
mod tape;
mod machine;
mod tick;

fn main() {
    let mut tape = tape::Tape::new("tapes/empty.txt");
    let mut machine = machine::Machine::new("machines/simple.txt");
    println!("{tape}");
    while tick::tick_tape(&mut machine, &mut tape) {
        println!("{tape}");
    }
}
