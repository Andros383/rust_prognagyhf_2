use std::{cell::RefCell, rc::Weak};

use crate::{
    components::{Component, ParseComponentError, WritableComponent},
    wire::Wire,
};

#[derive(Debug)]
pub struct Output {
    input: Weak<RefCell<Wire>>,
    label: String,
}

impl Output {
    pub fn new(output: Weak<RefCell<Wire>>, label: &str) -> Self {
        Self {
            input: output,
            label: label.to_string(),
        }
    }
    pub fn read_from_line(
        board: &crate::Board,
        line: &str,
    ) -> Result<Box<dyn WritableComponent>, ParseComponentError> {
        let mut iter = line.split(" ");
        match iter.next() {
            Some(idf) => {
                if idf != "OUT" {
                    return Err(ParseComponentError::NotThisGate);
                }
            }
            None => {
                return Err(ParseComponentError::OtherError(
                    "Error reading in identifier.".to_string(),
                ));
            }
        }
        if let (Some(input), Some(label), None) = (
            iter.next().and_then(|x| x.parse::<usize>().ok()),
            iter.next(),
            iter.next(),
        ) {
            Ok(Box::new(Self {
                input: board.get_wire(input),
                label: label.to_string(),
            }))
        } else {
            Err(ParseComponentError::OtherError(format!(
                "Incorrect arguments for gate OUT: {}",
                line
            )))
        }
    }
}

impl Component for Output {
    fn operation(&self) {
        if let Some(rc) = self.input.upgrade() {
            println!("Output gate [{}]: {}", self.label, rc.borrow().read());
        } else {
            println!("Output gate [{}]: Couldn't read data", self.label);
        }
    }
}

impl WritableComponent for Output {
    fn write_component(&self, board: &crate::board::Board) -> Result<String, ()> {
        if let (Some(in_a)) = (board.get_wire_pointer(&self.input)) {
            return Ok(format!("OUT {} {}", in_a, self.label));
        } else {
            println!("Problem writing OUT component.");
        }
        todo!();
    }
}
