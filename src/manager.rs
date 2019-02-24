extern crate wasm_bindgen;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use grid::Grid;
use tile::Tile;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Manager {
    grid: Grid,
}

#[wasm_bindgen]
impl Manager {
    pub fn new(width: usize, height: usize) -> Manager {
        Manager {
            grid: Grid::new(width, height),
        }
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn handle_event(&self, direction: &str) {
        match direction {
            "UP" => self.print(),
            "DOWN" => self.print(),
            "LEFT" => self.print(),
            "RIGHT" => self.print(),
            _otherwise => panic!("unsupported direction"),
        };
    }

    pub fn get_tile(&self, index: usize) -> Tile {
        self.grid.get_tile(index)
    }

    pub fn print(&self) {
        log!("{:#?}", self.grid);
    }
}
