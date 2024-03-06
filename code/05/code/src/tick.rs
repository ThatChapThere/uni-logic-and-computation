use crate::machine;
use crate::tape;

pub fn tick_tape(machine: &mut machine::Machine, tape: &mut tape::Tape) -> bool {
    if let Some(ref i) = machine.states[machine.state][tape.current] {
        tape.operate(i.operation);
        machine.state = i.state;
        if i.state >= machine.states.len() {
            machine.states.resize_with(i.state + 1, Default::default);
        }
        return true;
    }
    return false;
}
