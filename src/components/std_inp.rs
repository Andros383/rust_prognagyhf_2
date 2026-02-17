use std::{cell::RefCell, io, process::Output, rc::Weak, str::FromStr};

use crate::{
    components::{Component, ParseComponentError, WritableComponent},
    wire::Wire,
};

#[derive(Debug)]
pub struct StdInp {
    output: Weak<RefCell<Wire>>,
    value: i32,
}

impl StdInp {
    pub fn new(output: Weak<RefCell<Wire>>, value: i32) -> Self {
        // initnél állítja be az értéket, mert amúgy is kb ekkor állítaná be
        // és különben kéne, hogy mut legyen az operation függvény

        // TODO megoldani, hogy mondja, hanyadik kapu?
        println!("Give input for a STDINP gate");
        let stdin = io::stdin();
        let mut value = 0;
        if let Some(Ok(line)) = stdin.lines().next()
            && let Ok(x) = line.parse::<i32>()
        {
            value = x;
        };
        Self { output, value }
    }
    pub fn read_from_line(
        board: &crate::Board,
        line: &str,
    ) -> Result<Box<dyn WritableComponent>, ParseComponentError> {
        let mut iter = line.split(" ");
        match iter.next() {
            Some(idf) => {
                if idf != "STD_INP" {
                    return Err(ParseComponentError::NotThisGate);
                }
            }
            None => {
                return Err(ParseComponentError::OtherError(
                    "Error reading in identifier.".to_string(),
                ));
            }
        }
        if let (Some(output_id), Some(value), None) = (
            iter.next().and_then(|s| s.parse::<usize>().ok()),
            iter.next().and_then(|s| s.parse::<i32>().ok()),
            iter.next(),
        ) {
            Ok(Box::new(Self {
                output: board.get_wire(output_id),
                value,
            }))
        } else {
            Err(ParseComponentError::OtherError(format!(
                "Incorrect arguments for gate STD_INP: {}",
                line
            )))
        }
    }
}

impl Component for StdInp {
    fn operation(&self) {
        if let Some(output) = self.output.upgrade() {
            output.borrow_mut().write(self.value);
        } else {
            print!("Debug information: an Input's output wire is not found");
        }
    }
}

impl WritableComponent for StdInp {
    fn write_component(&self, board: &crate::board::Board) -> Result<String, ()> {
        if let (Some(out)) = (board.get_wire_pointer(&self.output)) {
            return Ok(format!("STD_INP {} {}", out, self.value));
        } else {
            println!("Problem writing STD_INP component.");
        }
        todo!();
    }
}
