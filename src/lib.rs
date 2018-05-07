#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let mut cells = vec![Cell::Dead; width * height];
        let center = ((height / 2) * width + (width / 2));
        cells[center] = Cell::Alive;
        cells[center + 1] = Cell::Alive;
        cells[center + width] = Cell::Alive;
        cells[center + width + 1] = Cell::Alive;

        Universe {
            width: width as u32,
            height: height as u32,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                next[idx] = match (self.cells[idx], self.live_neighbor_count(row, col)) {
                    (Cell::Dead, 2...4) => Cell::Alive,
                    _ => Cell::Dead
                }
            }
        }

        self.cells = next;
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            f.write_str(match cell {
                Cell::Dead => "◻ ",
                Cell::Alive => "◼ "
            })?;
            if (i + 1) % self.width as usize == 0 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

#[test]
fn universe_displays_correctly() {
    let universe = Universe {
        width: 4,
        height: 4,
        cells: vec![
            Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Alive,
            Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive,
            Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Alive,
        ],
    };

    assert_eq!(
        universe.to_string(),
        "◻ ◻ ◻ ◻ \n\
         ◻ ◻ ◻ ◼ \n\
         ◻ ◻ ◼ ◼ \n\
         ◻ ◼ ◼ ◼ \n"
    );
}
