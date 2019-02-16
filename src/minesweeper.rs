//! Gameboard logic.

use std::collections::HashSet;

/// Sizes of gameboard.
const SIZE_BEGINNER: usize = 8;
const SIZE_INTERMEDIATE: usize = 16;
const SIZE_EXPERT: usize = 24;

/// Mine number for different game modes.
const MINE_NUMBER_BEGINNER: usize = 10;
const MINE_NUMBER_INTERMEDIATE: usize = 40;
const MINE_NUMBER_EXPERT: usize = 99;

/// Game mode.
pub enum GameMode {
    Beginner,
    Intermediate,
    Expert,
}

/// Basic Minesweeper board.
pub struct Minesweeper {
    pub size: usize,
    pub mine_number: usize,
    pub cells: Vec<Cell>,
}

impl Minesweeper {
    /// Creates new Minesweeper struct.
    pub fn new(mode: GameMode) -> Minesweeper {
        let (s, m) = match mode {
            GameMode::Beginner => (SIZE_BEGINNER, MINE_NUMBER_BEGINNER),
            GameMode::Intermediate => (SIZE_INTERMEDIATE, MINE_NUMBER_INTERMEDIATE),
            GameMode::Expert => (SIZE_EXPERT, MINE_NUMBER_EXPERT),
        };

        Minesweeper {
            size: s,
            mine_number: m,
            cells: vec![Cell { mine: false, adjacency_num: 0, state: CellState::Closed }; s * s],
        }
    }

    /// Gets the adjacency number at the cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        println!("{:?}", ind);

        println!("{}", ind[1] * self.size + ind[0]);
        Some(match self.cells[ind[1] * self.size + ind[0]].adjacency_num {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            _ => return None,
        })
    }

    /// Initializes the gameboard, randomizes mines, and calculates adjacency indices.
    pub fn init(&mut self) {
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        let mut mines: HashSet<usize> = HashSet::new();
        
        while mines.len() <= self.mine_number {
            let temp: usize = rng.gen();
            let temp = temp % 64;
            mines.insert(temp);
        }
        
        for mine in mines {
            self.cells[mine].mine = true;
        }

        for index in 0..self.cells.len() {
        let temp_val: i32 = index as i32;

        
         
        }
    }
}

/// Stores the current cell state.
#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Open(u8),
    Closed,
}

/// Stores cell data.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub mine: bool,
    pub adjacency_num: u8,
    pub state: CellState,
}

impl Cell {
    /// Creates a new Cell.
    pub fn new() -> Cell {
        Cell {
            mine: false,
            adjacency_num: 0,
            state: CellState::Closed,
        }
    }

    /// Toggles the cell
    pub fn toggle(&mut self) -> bool {
        match self.state {
            CellState::Open(_) => false,
            CellState::Closed => {
                self.state = CellState::Open(self.adjacency_num);
                true
            },
        }
    }
}
