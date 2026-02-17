use std::{
    arch::x86_64::_mm_minpos_epu16,
    cell::{RefCell, RefMut},
    error::Error,
    fmt::Pointer,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    rc::{Rc, Weak},
    sync::PoisonError,
};

use crate::components::{
    Add, Component, Input, ParseComponentError, StdInp, WritableComponent, output::Output,
};
use crate::wire::Wire;

#[derive(Debug)]
pub struct Board {
    wires: Vec<Rc<RefCell<Wire>>>,
    gates: Vec<Box<dyn WritableComponent>>,
}

impl Board {
    pub fn new(wire_amount: usize) -> Self {
        let mut b = Board {
            wires: Vec::new(),
            gates: Vec::new(),
        };
        b.create_wires(wire_amount);
        b
    }

    fn update_gates(&self) {
        for gate in &self.gates {
            gate.operation();
        }
    }
    fn update_wires(&mut self) {
        for wire in &mut self.wires {
            wire.borrow_mut().update();
        }
    }
    pub fn update(&mut self) {
        self.update_gates();
        self.update_wires();
    }
    pub fn add_component(&mut self, component: impl WritableComponent) {
        self.gates.push(Box::new(component));
    }
    // direkt nem mut, mert nem ad hozzá automatikusan wire-t | vagy fordítva
    pub fn get_wire(&self, index: usize) -> Weak<RefCell<Wire>> {
        if let Some(wire) = self.wires.get(index) {
            Rc::downgrade(wire)
        } else {
            panic!("No such wire");
        }
    }

    // Visszaadja, az adott pointer hanyadik a tárolt wire-ok közt
    pub fn get_wire_pointer(&self, pointer: &Weak<RefCell<Wire>>) -> Option<usize> {
        for (idx, wire) in self.wires.iter().enumerate() {
            // .upgrade()? megcsinálja, ha már nincs a referencia, akkor kilép None-al
            if Rc::ptr_eq(wire, &pointer.upgrade()?) {
                return Some(idx);
            }
        }
        None
    }

    // TODO replacelni a konstruktorában / automatikus felvétel
    // TODO ugyan ez vektorok nélkül?
    pub fn create_wires(&mut self, wire_db: usize) {
        while self.wires.len() < wire_db {
            self.wires.push(Rc::new(RefCell::new(Wire::new())));
        }
    }
    pub fn write_components(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        for component in &self.gates {
            if let Ok(line) = component.write_component(self) {
                // ezt nehéz volt kiklippizni
                file.write_all((line + "\n").as_bytes())?;
            } else {
                println!("Hiba volt");
            }
        }
        Ok(())
    }
    pub fn from(filename: &str) -> Result<Self, Box<dyn Error>> {
        // TODO ez csak ideiglenesen 100, lehetne első sorban a fileban, hogy mennyi wire van
        let mut board = Board::new(100);

        let file = File::open(filename)?;
        let buf_read = BufReader::new(file);
        // egy lista, amiben a különböző függvények vannak, amikkel be lehet olvasni
        type ComponentParseResult = Result<Box<dyn WritableComponent>, ParseComponentError>;
        type ComponentParser = fn(&crate::Board, &str) -> ComponentParseResult;
        let mut read_functions: Vec<ComponentParser> = vec![
            Add::read_from_line,
            Input::read_from_line,
            Output::read_from_line,
            StdInp::read_from_line,
        ];
        for line in buf_read.lines() {
            let line = line?;
            println!("Processing line: {}", line);
            if line.is_empty() {
                break;
            }
            for component_type in &read_functions {
                let result = component_type(&board, &line);
                match result {
                    // manuálisa a vektorhoz adjuk hozzá a komponenst
                    Ok(component_ptr) => {
                        board.gates.push(component_ptr);
                        println!("Added a gate");
                        break;
                    }
                    Err(err) => match err {
                        ParseComponentError::NotThisGate => {
                            println!("Not this gate, going to next one");
                            continue;
                        }
                        ParseComponentError::OtherError(_) => {
                            println!("Other error");
                            break;
                        }
                    },
                }
            }
        }
        Ok(board)
    }
}
