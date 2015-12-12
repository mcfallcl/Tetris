#[macro_use]
extern crate log;
extern crate piston_window;

use self::piston_window::types::Color;

pub mod logger;

#[derive(Debug)]
pub enum Piece {
    Straight,
    LShape,
    BackwardLShape,
    RightZig,
    LeftZig,
    Square,
}

pub struct Grid {
    cells: Vec<CellStatus>,
}

impl Grid {
    pub fn init() -> Grid {
        info!("Initializing grid...");
        let cells = vec![CellStatus::Open; 240];
        let grid = Grid { cells: cells };
        info!("Grid successfully initialized.");
        grid
    }

    pub fn close_cell(&mut self, cell_num: usize, color: Color) -> Result<(), GridError> {
        self.change_cell_status(cell_num, CellStatus::Closed(color))
    }

    pub fn open_cell(&mut self, cell_num: usize) -> Result<(), GridError> {
        self.change_cell_status(cell_num, CellStatus::Open)
    }

    pub fn active_cell(&mut self, cell_num: usize, color: Color) -> Result<(), GridError> {
        self.change_cell_status(cell_num, CellStatus::ActivePiece(color))
    }

    pub fn get_cells(&self) -> &Vec<CellStatus> {
        &self.cells
    }

    fn change_cell_status(&mut self,
                          cell_num: usize,
                          new_status: CellStatus)
                          -> Result<(), GridError> {
        info!("Attempting to change the status of cell: {} to {:?}",
              cell_num,
              new_status);
        if !self.is_valid_cell(cell_num) {
            error!("Tried to change the status of an invalid cell: {}",
                   cell_num);
            return Err(GridError::InvalidGridNumber);
        }
        if self.cells[cell_num] == new_status {
            let error = match new_status {
                CellStatus::Open => GridError::CellAlreadyOpen,
                CellStatus::Closed(_) => GridError::CellAlreadyClosed,
                CellStatus::ActivePiece(_) => GridError::CellAlreadyActive,
            };
            return Err(error);
        }

        self.cells[cell_num] = new_status;
        info!("Success");
        Ok(())
    }

    fn is_valid_cell(&self, cell_num: usize) -> bool {
        cell_num < 240
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CellStatus {
    Open,
    Closed(Color),
    ActivePiece(Color),
}

impl PartialEq for CellStatus {
    fn eq(&self, other: &CellStatus) -> bool {
        match *self {
            CellStatus::Open => match *other {
                CellStatus::Open => true,
                _ => false,
            },
            CellStatus::Closed(_) => match *other {
                CellStatus::Closed(_) => true,
                _ => false,
            },
            CellStatus::ActivePiece(_) => match *other {
                CellStatus::ActivePiece(_) => true,
                _ => false,
            },
        }
    }
}

impl CellStatus {
    pub fn get_color(&self) -> Color {
        match *self {
            CellStatus::Closed(c) => c,
            CellStatus::ActivePiece(c) => c,
            CellStatus::Open => [0.95, 0.95, 0.95, 1.0],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GridError {
    CellAlreadyClosed,
    CellAlreadyOpen,
    CellAlreadyActive,
    InvalidGridNumber,
}
