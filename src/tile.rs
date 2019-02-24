extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use position::Position;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Tile {
    x: usize,
    y: usize,
    value: Option<u16>,
    previous_position: Option<Position>,
}

impl Tile {
    pub fn new(x: usize, y: usize, value: Option<u16>) -> Tile {
        Tile {
            x,
            y,
            value,
            previous_position: None,
        }
    }

    pub fn set_value(&mut self, value: u16) {
        self.value = Some(value);
    }
}

#[wasm_bindgen]
impl Tile {
    pub fn get_x(&self) -> u32 {
        self.x as u32
    }

    pub fn get_y(&self) -> u32 {
        self.y as u32
    }

    pub fn get_value(&self) -> Option<u16> {
        self.value
    }
}
