use std::slice::{Iter, IterMut};

use piece::{Piece, PieceGenerator};
use piece::colors::*;



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
    active_piece: Piece,
    piece_generator: PieceGenerator,
    game_over: bool,
}

impl Grid {
    pub fn init() -> Grid {
        info!("Initializing grid...");
        let cells = vec![Cell::new(); 240];
        let mut generator = PieceGenerator::new();
        let grid = Grid {
            cells: cells,
            active_piece: generator.pop(),
            piece_generator: generator,
            game_over: false,
        };
        info!("Grid successfully initialized.");
        grid
    }

    pub fn new_piece(&mut self) {
        self.active_piece = self.piece_generator.pop();
    }

    pub fn close_cell(&mut self, cell_num: usize, color: Color) {
        debug!("Closing cell: {}", cell_num);
        self.change_cell_status(cell_num, CellStatus::Closed, color);
    }

    pub fn open_cell(&mut self, cell_num: usize) {
        debug!("Opening cell: {}", cell_num);
        self.change_cell_status(cell_num, CellStatus::Open, OPEN_COLOR);
    }

    pub fn activate_cell(&mut self, cell_num: usize, color: Color) {
        debug!("Activating cell: {}", cell_num);
        self.change_cell_status(cell_num, CellStatus::Active, color);
    }

    pub fn iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Cell> {
        self.cells.iter_mut()
    }

    pub fn cycle(&mut self) {
        let potential_posit = self.active_piece.potential_down_posit();
        if self.check_posit(potential_posit) {
            self.move_active_to(potential_posit);
        } else {
            for i in 0..4 {
                let cell_num = self.active_piece.get_cell(i);
                let color = self.active_piece.color();
                self.close_cell(cell_num, color);
            }
            self.new_piece();
        }
        self.check_rows();
    }

    fn move_active_to(&mut self, new_posit: [usize; 4]) {
        for i in 0..4 {
            let cell_num = self.active_piece.get_cell(i);
            self.open_cell(cell_num);
        }
        self.active_piece.move_to(new_posit);
        for i in 0..4 {
            let cell_num = new_posit[i];
            let color = self.active_piece.color();
            self.activate_cell(cell_num, color);
        }
    }

    pub fn move_active_down(&mut self) {
        self.cycle();
    }

    pub fn move_active_right(&mut self) {
        let potential_posit = self.active_piece.potential_right_posit();
        if !self.check_posit(potential_posit) {
            return;
        }
        self.move_active_to(potential_posit);
    }

    pub fn move_active_left(&mut self) {
        let potential_posit = self.active_piece.potential_left_posit();
        if !self.check_posit(potential_posit) {
            return;
        }
        self.move_active_to(potential_posit);
    }

    pub fn rotate_active_cw(&mut self) {
        let potential_posit = self.active_piece.potential_cw_posit();
        if self.check_posit(potential_posit) {
            self.move_active_to(potential_posit);
            self.active_piece.rotate_orient_cw();
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
        self.move_closed_down(row * 10);
    }

    fn check_if_row_full(&self, row: usize) -> bool {
        let mut result = true;
        for i in 0..10 {
            let index = (row * 10) + i;
            let cell = self.cells.get(index).unwrap();
            match cell.get_status() {
                CellStatus::Closed => {}
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
        info!("Game Over!");
    }

    fn move_closed_down(&mut self, above: usize) {
        let mut index = above - 1;
        while index > 0 {
            self.cells[index + 10] = self.cells[index].clone();
            self.cells[index].open();
            index -= 1;
        }
    }

    fn check_posit(&self, posit: [usize; 4]) -> bool {
        for i in 0..4 {
            let index = posit[i];
            if !self.is_valid_cell(index) ||
               self.cells.get(index).unwrap().get_status() == CellStatus::Closed {
                return false;
            }
        }
        Piece::is_not_off_side(posit)
    }

    fn change_cell_status(&mut self, cell_num: usize, new_status: CellStatus, new_color: Color) {
        let mut cell = self.cells
                           .get_mut(cell_num)
                           .expect("Index out of bounds in cell.change_cell_status.");
        cell.set_status(new_status);
        cell.set_color(new_color);
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
