use std::{cell::RefCell, process::Output, rc::Weak};

use crate::{
    components::{Component, ParseComponentError, WritableComponent},
    wire::Wire,
};

#[derive(Debug)]
pub struct Input {
    output: Weak<RefCell<Wire>>,
    value: i32,
}

impl Input {
    pub fn new(output: Weak<RefCell<Wire>>, value: i32) -> Self {
        Self { output, value }
    }
    pub fn read_from_line(
        board: &crate::Board,
        line: &str,
    ) -> Result<Box<dyn WritableComponent>, ParseComponentError> {
        let mut iter = line.split(" ");
        match iter.next() {
            Some(idf) => {
                if idf != "INP" {
                    return Err(ParseComponentError::NotThisGate);
                }
            }
            None => {
                return Err(ParseComponentError::OtherError(
                    "Error reading in identifier.".to_string(),
                ));
            }
        }
        if let (Some(input_id), Some(value), None) = (
            iter.next().and_then(|x| x.parse::<usize>().ok()),
            iter.next().and_then(|x| x.parse::<i32>().ok()),
            iter.next(),
        ) {
            Ok(Box::new(Self {
                output: board.get_wire(input_id),
                value,
            }))
        } else {
            Err(ParseComponentError::OtherError(format!(
                "Incorrect arguments for gate INP: {}",
                line
            )))
        }
    }
}

impl Component for Input {
    fn operation(&self) {
        if let Some(output) = self.output.upgrade() {
            output.borrow_mut().write(self.value);
        } else {
            print!("Debug information: an Input's output wire is not found");
        }
    }
}

impl WritableComponent for Input {
    fn write_component(&self, board: &crate::board::Board) -> Result<String, ()> {
        if let (Some(out)) = (board.get_wire_pointer(&self.output)) {
            return Ok(format!("INP {} ", out));
        } else {
            println!("Problem writing INP component.");
        }
        todo!();
    }
}
