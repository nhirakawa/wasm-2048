extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use tile::Tile;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let total_length = width * height;

        let mut tiles = Vec::with_capacity(total_length);

        let first_random_index: usize = (js_sys::Math::random() * total_length as f64) as usize;
        let first_random_value = if js_sys::Math::random() > 0.9 { 4 } else { 2 };
        let second_random_index: usize = (js_sys::Math::random() * total_length as f64) as usize;
        let second_random_value = if js_sys::Math::random() > 0.9 { 4 } else { 2 };

        for i in 0..width {
            for j in 0..height {
                let index = i * width + j;
                if index == first_random_index {
                    tiles.push(Tile::new(i, j, Some(first_random_value)));
                } else if index == second_random_index {
                    tiles.push(Tile::new(i, j, Some(second_random_value)));
                } else {
                    tiles.push(Tile::new(i, j, None));
                }
            }
        }

        Grid {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_tile(&self, index: usize) -> Tile {
        self.tiles[index]
    }
}
