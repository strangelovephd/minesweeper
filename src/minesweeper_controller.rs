//! Minesweeper Controller

use piston::input::GenericEvent;

use crate::minesweeper::Minesweeper;

/// Handles events for the Minesweeper game.
pub struct MinesweeperController {
    /// Stores current Minesweeper board state.
    pub board: Minesweeper,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Stores the last mouse position.
    pub cursor_pos: [f64; 2],
}

impl MinesweeperController {
    /// Creates a new MinesweeperController.
    pub fn new(b: Minesweeper) -> MinesweeperController {
        MinesweeperController {
            board: b,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        unimplemented!();
    }
}