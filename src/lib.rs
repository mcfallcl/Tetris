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
            _ => panic!("Invalid piece num")
        }
    }
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

    pub fn ping(&mut self) {
        let mut active_cell_indecies: [usize; 4] = [240; 4];
        println!("{:?}", active_cell_indecies);
        let mut i: usize = 0;
        let mut index: usize = 0;
        for cell in self.cells.iter() {
            match *cell {
                CellStatus::ActivePiece(_) => {
                    active_cell_indecies[index] = i;
                    index += 1;
                    println!("{:?}", active_cell_indecies);
                },
                _ => {},
            };
            i += 1;
        }
        if self.check_down(active_cell_indecies) {
            let mut active_cells = [CellStatus::Open; 4];
            for i in 0..4 {
                active_cells[i] = self.cells[active_cell_indecies[i]];
            }
            for i in 0..4 {
                self.open_cell(active_cell_indecies[i]);
            }
            for i in 0..4 {
                self.active_cell(active_cell_indecies[i] + 10, active_cells[i].get_color()).unwrap();
            }
        } else {
            for i in 0..4 {
                index = active_cell_indecies[i];
                println!("{}", index);
                let color = self.cells.get(index).unwrap().get_color();
                self.close_cell(index, color);
            }
        }
        println!("{:?}", active_cell_indecies);
    }

    fn check_down(&self, piece: [usize; 4]) -> bool {
        let mut result = true;
        for i in 0..4 {
            if piece[i] >= 230 {
                result = false;
            }
            match self.cells.get(i + 10) {
                None => {},
                Some(c) => match *c {
                    CellStatus::Closed(_) => result = false,
                    _ => {},
                }
            }
        }
        println!("{}", result);
        result
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
