use crate::core::{Cell, Position};
use core::fmt;

pub struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[const { Cell::Empty }; 3]; 3],
        }
    }

    pub fn play(&mut self, position: Position, cell: Cell) {
        if cell == Cell::Empty {
            panic!("Play value should be either X or O. It cannot be Empty.")
        }

        let (row, col) = position.coords();

        if self.cells[row][col] != Cell::Empty {
            panic!("Cannot play in a non-empty cell.")
        }

        self.cells[row][col] = cell;
    }

    pub fn cells(&self) -> &[[Cell; 3]; 3] {
        &self.cells
    }

    pub fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌───┬───┬───┐")?;

        for (row_idx, row) in self.cells.iter().enumerate() {
            write!(f, "│")?;
            for (col_idx, cell) in row.iter().enumerate() {
                let symbol = match cell {
                    Cell::X => " X ",
                    Cell::O => " O ",
                    Cell::Empty => &format!(" {} ", (3 * row_idx) + col_idx + 1),
                };
                write!(f, "{}│", symbol)?;
            }
            writeln!(f)?;

            if row_idx < self.cells.len() - 1 {
                writeln!(f, "├───┼───┼───┤")?;
            }
        }

        writeln!(f, "└───┴───┴───┘")?;
        Ok(())
    }
}
