#[allow(dead_code)]
pub mod gui;
use gui::{Draw};

// now we can create button structs for different types of button and impl the Draw train for them
// and they all will be accomodated in the Screen struct despite being different types.

pub struct Button {
    pub hieght: u32,
    pub breadth: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        // some code here.
    }
}

pub struct Options {
    pub hieght: u32,
    pub breadth: u32,
    pub options: Vec<String>
}

impl Draw for Options {
    fn draw(&self) {
        // some code here.
    }
}
