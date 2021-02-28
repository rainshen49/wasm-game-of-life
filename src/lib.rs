mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;
extern crate js_sys;
use std::mem::swap;
use web_sys::console;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
    next_cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;
        let n = self.get_index(north, column);
        count += self.cells[n] as u8;
        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;
        let w = self.get_index(row, west);
        count += self.cells[w] as u8;
        let e = self.get_index(row, east);
        count += self.cells[e] as u8;
        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;
        let s = self.get_index(south, column);
        count += self.cells[s] as u8;
        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);
                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                self.next_cells[idx] = next_cell;
            }
        }
        swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn new() -> Universe {
        let width = 600;
        let height = 600;
        Universe::custom_size(width, height)
    }

    pub fn custom_size(width: usize, height: usize) -> Universe {
        utils::set_panic_hook();
        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() > 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        let next_cells = vec![Cell::Dead; width * height];
        Universe {
            width,
            height,
            cells,
            next_cells,
        }
    }

    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn render_canvas(&self) -> *const Cell {
        return self.cells.as_ptr();
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
