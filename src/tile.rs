extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Tile {
    x: usize,
    y: usize,
    value: Option<u16>,
}

impl Tile {
    pub fn new(x: usize, y: usize, value: Option<u16>) -> Tile {
        Tile { x, y, value }
    }

    pub fn set_value(&mut self, value: u16) {
        self.value = Some(value);
    }
}
