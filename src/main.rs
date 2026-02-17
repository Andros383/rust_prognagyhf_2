// csak implementálgatok, sok mindent nem használoks

#![allow(unused)]

mod board;
mod components;
mod wire;

use board::Board;
use components::*;
use std::{rc::Rc, str::MatchIndices};
use wire::Wire;

use crate::components::output::Output;

fn test_board() {
    let mut b = Board::new(3);
    b.add_component(Add::new(b.get_wire(0), b.get_wire(1), b.get_wire(2)));
    b.add_component(StdInp::new(b.get_wire(0), 1));
    b.add_component(StdInp::new(b.get_wire(1), 3));
    b.add_component(Output::new(b.get_wire(2), "Output"));
    b.write_components("valami.txt");
}
fn main() {
    test_board();
    let mut b = Board::from("valami.txt");
    let mut b = match b {
        Ok(b) => b,
        Err(e) => panic!("Hiba beolvasás közben"),
    };
    b.update();
    b.update();
    b.update();
}
