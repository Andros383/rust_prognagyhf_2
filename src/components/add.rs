use std::{arch::x86_64::_mm_cvt_roundu64_ss, cell::RefCell, rc::Weak};

use crate::{
    components::{Component, ParseComponentError, WritableComponent},
    wire::Wire,
};

#[derive(Debug)]
pub struct Add {
    input_a: Weak<RefCell<Wire>>,
    input_b: Weak<RefCell<Wire>>,
    output: Weak<RefCell<Wire>>,
}

impl Add {
    pub fn new(
        input_a: Weak<RefCell<Wire>>,
        input_b: Weak<RefCell<Wire>>,
        output: Weak<RefCell<Wire>>,
    ) -> Self {
        Self {
            input_a,
            input_b,
            output,
        }
    }
    pub fn read_from_line(
        board: &crate::Board,
        line: &str,
    ) -> Result<Box<dyn WritableComponent>, ParseComponentError> {
        let mut iter = line.split(" ");
        match iter.next() {
            Some(idf) => {
                if idf != "ADD" {
                    return Err(ParseComponentError::NotThisGate);
                }
            }
            None => {
                return Err(ParseComponentError::OtherError(
                    "Error reading in identifier.".to_string(),
                ));
            }
        }
        if let (Some(wire1), Some(wire2), Some(wire3), None) = (
            iter.next().and_then(|x| x.parse::<usize>().ok()),
            iter.next().and_then(|x| x.parse::<usize>().ok()),
            iter.next().and_then(|x| x.parse::<usize>().ok()),
            iter.next(),
        ) {
            Ok(Box::new(Self {
                input_a: board.get_wire(wire1),
                input_b: board.get_wire(wire2),
                output: board.get_wire(wire3),
            }))
        } else {
            Err(ParseComponentError::OtherError(format!(
                "Incorrect arguments for gate ADD: {}",
                line
            )))
        }
    }
}

impl Component for Add {
    fn operation(&self) {
        // Ha nem elérhető (mert droppolták), akkor marad 0-n a signal value-ja
        let mut val1 = 0;
        if let Some(rc) = self.input_a.upgrade() {
            val1 = rc.borrow().read();
        } else {
            print!("Debug Message: Wire of a gate was dropped.");
        }
        let mut val2 = 0;
        if let Some(rc) = self.input_b.upgrade() {
            val2 = rc.borrow().read();
        } else {
            print!("Debug Message: Wire of a gate was dropped.");
        }

        if let Some(rc) = self.output.upgrade() {
            rc.borrow_mut().write(val1 + val2);
        } else {
            print!("Debug Message: Wire of a gate was dropped.");
        }
    }
}

impl WritableComponent for Add {
    fn write_component(&self, board: &crate::board::Board) -> Result<String, ()> {
        if let (Some(in_a), Some(in_b), Some(out)) = (
            board.get_wire_pointer(&self.input_a),
            board.get_wire_pointer(&self.input_b),
            board.get_wire_pointer(&self.output),
        ) {
            return Ok(format!("ADD {} {} {}", in_a, in_b, out));
        } else {
            println!("Problem writing ADD component.");
        }
        todo!();
    }
}
