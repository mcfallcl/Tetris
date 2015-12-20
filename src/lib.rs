#[macro_use]
extern crate log;
extern crate piston_window;
extern crate time;
extern crate rand;

use std::slice::{Iter, IterMut};

use self::piston_window::types::Color;

use self::time::{Duration, Timespec};

pub const OPEN_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

pub mod logger;

#[derive(Debug)]
pub enum Piece {
    Straight,
    LShape,
    BackwardLShape,
    TShape,
    RightZig,
    LeftZig,
    Square,
}

impl Piece {
    pub fn new(piece_num: u8) -> Piece {
        match piece_num {
            0 => Piece::Straight,
            1 => Piece::LShape,
            2 => Piece::BackwardLShape,
            3 => Piece::TShape,
            4 => Piece::RightZig,
            5 => Piece::LeftZig,
            6 => Piece::Square,
            _ => panic!("Invalid piece num"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    color: Color,
    status: CellStatus,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            color: OPEN_COLOR,
            status: CellStatus::Open,
        }
    }

    pub fn get_status(&self) -> CellStatus {
        self.status
    }

    pub fn set_status(&mut self, new_status: CellStatus) {
        match new_status {
            CellStatus::Open => self.open(),
            CellStatus::Closed => self.close(),
            CellStatus::Active => self.active(),
        }
    }

    pub fn open(&mut self) {
        self.status = CellStatus::Open;
        self.color = OPEN_COLOR;
    }

    pub fn close(&mut self) {
        self.status = CellStatus::Closed;
    }

    pub fn active(&mut self) {
        self.status = CellStatus::Active;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    active_cells: [usize; 4],
    active_color: Color,
    game_over: bool,
}

impl Grid {
    pub fn init() -> Grid {
        info!("Initializing grid...");
        let cells = vec![Cell::new(); 240];
        let grid = Grid {
            cells: cells,
            active_cells: [240; 4],
            active_color: BLACK,
            game_over: false,
        };
        info!("Grid successfully initialized.");
        grid
    }

    pub fn new_piece(&mut self, color: Color) {
        let mut rng = rand::random::<u8>();
        while rng > 252 {
            rng = rand::random::<u8>();
        }
        let piece = Piece::new(rng % 7);
        match piece {
            Piece::Straight => self.active_cells = [5, 15, 25, 35],
            Piece::LShape => self.active_cells = [14, 24, 34, 35],
            Piece::BackwardLShape => self.active_cells = [15, 25, 34, 35],
            Piece::TShape => self.active_cells = [23, 24, 25, 34],
            Piece::RightZig => self.active_cells = [24, 25, 35, 36],
            Piece::LeftZig => self.active_cells = [25, 26, 34, 35],
            Piece::Square => self.active_cells = [24, 25, 34, 35],
        }
        for i in 0..4 {
            self.activate_cell(i).unwrap();
        }
        self.active_color = color;
    }

    pub fn close_cell(&mut self, cell_num: usize) -> Result<(), GridError> {
        let color = self.active_color;
        self.change_cell_status(cell_num, CellStatus::Closed, color)
    }

    pub fn open_cell(&mut self, cell_num: usize) -> Result<(), GridError> {
        self.change_cell_status(cell_num, CellStatus::Open, OPEN_COLOR)
    }

    pub fn activate_cell(&mut self, cell_num: usize) -> Result<(), GridError> {
        let color = self.active_color;
        self.change_cell_status(cell_num, CellStatus::Active, color)
    }

    pub fn iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Cell> {
        self.cells.iter_mut()
    }

    pub fn cycle(&mut self) {
        let mut cell_num: usize;
        if self.check_active_down() {
            for i in 0..4 {
                cell_num = self.active_cells[i];
                self.open_cell(cell_num).unwrap();
                self.active_cells[i] += 10;
            }
            for i in 0..4 {
                cell_num = self.active_cells[i];
                self.activate_cell(cell_num).unwrap();
            }
        } else {
            for i in 0..4 {
                cell_num = self.active_cells[i];
                self.close_cell(cell_num).unwrap();
            }
            self.new_piece(BLACK);
        }
        self.check_rows();
    }

    pub fn move_active_down(&mut self) {
        self.cycle();
    }

    pub fn move_active_right(&mut self) {
        if !self.check_active_right() {
            return;
        }
        let mut cell_num: usize;
        for i in 0..4 {
            cell_num = self.active_cells[i];
            self.open_cell(cell_num).unwrap();
            self.active_cells[i] += 1;
        }
        for i in 0..4 {
            cell_num = self.active_cells[i];
            self.activate_cell(cell_num).unwrap();
        }
    }

    pub fn move_active_left(&mut self) {
        if !self.check_active_left() {
            return;
        }
        let mut cell_num: usize;
        for i in 0..4 {
            cell_num = self.active_cells[i];
            self.open_cell(cell_num).unwrap();
            self.active_cells[i] -= 1;
        }
        for i in 0..4 {
            cell_num = self.active_cells[i];
            self.activate_cell(cell_num).unwrap();
        }
    }

    fn check_if_game_over(&self) -> bool {
        let mut result = false;
        for i in 0..40 {
            match self.cells.get(i).unwrap().get_status() {
                CellStatus::Closed => result = true,
                _ => {}
            }
        }
        result
    }

    fn check_rows(&mut self) {
        self.clear_full_rows();
        if self.check_if_game_over() {
            self.game_over();
        }
    }

    fn clear_full_rows(&mut self) {
        for row in 0..24 {
            if self.check_if_row_full(row) {
                self.clear_row(row);
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        for i in 0..10 {
            let index = (row * 10) + i;
            let mut cell = self.cells.get_mut(index).unwrap();
            cell.open();
        }
    }

    fn check_if_row_full(&self, row: usize) -> bool {
        let mut result = true;
        for i in 0..10 {
            let index = (row * 10) + i;
            let cell = self.cells.get(index).unwrap();
            match cell.get_status() {
                CellStatus::Closed => {},
                _ => result = false,
            }
        }
        result
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn game_over(&mut self) {
        self.game_over = true;
    }

    // returns true if the piece can move down
    fn check_active_down(&self) -> bool {
        for i in 0..4 {
            let cell_num = self.active_cells[i];
            if cell_num + 10 >= 240 ||
               self.cells.get(cell_num + 10).unwrap().get_status() == CellStatus::Closed {
                return false;
            }
        }
        true
    }

    fn check_active_right(&self) -> bool {
        for i in 0..4 {
            let cell_num = self.active_cells[i];
            if (cell_num + 1) % 10 == 0 ||
               self.cells.get(cell_num + 1).unwrap().get_status() == CellStatus::Closed {
                return false;
            }
        }
        true
    }

    fn check_active_left(&self) -> bool {
        for i in 0..4 {
            let cell_num = self.active_cells[i];
            if (cell_num - 1) % 10 == 9 ||
               self.cells.get(cell_num - 1).unwrap().get_status() == CellStatus::Closed {
                return false;
            }
        }
        true
    }

    fn change_cell_status(&mut self,
                          cell_num: usize,
                          new_status: CellStatus,
                          new_color: Color)
                          -> Result<(), GridError> {
        if !self.is_valid_cell(cell_num) {
            return Err(GridError::InvalidGridNumber);
        }
        let mut cell = self.cells.get_mut(cell_num).unwrap();
        cell.set_status(new_status);
        cell.set_color(new_color);
        Ok(())
    }

    fn is_valid_cell(&self, cell_num: usize) -> bool {
        cell_num < 240
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellStatus {
    Open,
    Closed,
    Active,
}

#[derive(Debug, Clone, Copy)]
pub enum GridError {
    CellAlreadyClosed,
    CellAlreadyOpen,
    CellAlreadyActive,
    InvalidGridNumber,
}

#[derive(Debug)]
pub struct CycleTimer {
    last_cycle: Timespec,
    cycle_time: Duration,
}

impl CycleTimer {
    pub fn new(cycle_time: i64) -> CycleTimer {
        CycleTimer {
            last_cycle: time::get_time(),
            cycle_time: Duration::milliseconds(cycle_time),
        }
    }

    pub fn reset(&mut self) {
        self.last_cycle = time::get_time();
    }

    fn check(&self) -> bool {
        let current_time = time::get_time();

        self.last_cycle + self.cycle_time < current_time
    }

    pub fn cycle(&mut self) -> bool {
        if self.check() {
            self.reset();
            true
        } else {
            false
        }
    }
}
