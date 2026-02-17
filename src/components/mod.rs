pub mod add;
pub mod input;
pub mod output;
pub mod std_inp;

pub use add::Add;
use core::fmt;
pub use input::Input;
use std::str::FromStr;
pub use std_inp::StdInp;

use crate::board::Board;

// itt elméletben azt jelenti a + 'static, hogy csak olyan objektum lehet, amiben nincsen referencia
// azaz minden adatát birtokolja, így amikor megkapja a vektor, az ténylegesen át tudja venni
pub trait Component: 'static + fmt::Debug {
    fn operation(&self);
}

pub trait WritableComponent: Component {
    // TODO stream szerű valamit kapjon, amibe írja ki magát
    fn write_component(&self, board: &Board) -> Result<String, ()>;
}

pub enum ParseComponentError {
    NotThisGate,
    OtherError(String),
}
