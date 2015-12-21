#[macro_use]
extern crate log;
extern crate piston_window;
extern crate time;
extern crate rand;

use std::slice::{Iter, IterMut};
use std::collections::VecDeque;

use rand::Rng;

use self::piston_window::types::Color;

use self::time::{Duration, Timespec};

pub const OPEN_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

pub const LIGHT_BLUE: Color = [0.0, 1.0, 1.0, 1.0];
pub const BLUE: Color = [0.0, 60.0 / 255.0, 1.0, 1.0];
pub const ORANGE: Color = [1.0, 174.0 / 255.0, 0.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const GREEN: Color = [30.0 / 255.0, 1.0, 0.0, 1.0];
pub const RED: Color = [1.0, 30.0 / 255.0, 0.0, 1.0];
pub const PURPLE: Color = [220.0 / 255.0, 0.0, 1.0, 1.0];

pub mod logger;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PieceShape {
    Straight,
    LShape,
    BackwardLShape,
    TShape,
    RightZig,
    LeftZig,
    Square,
}

impl PieceShape {
    pub fn get_color(&self) -> Color {
        match *self {
            PieceShape::Straight => LIGHT_BLUE,
            PieceShape::LShape => ORANGE,
            PieceShape::BackwardLShape => BLUE,
            PieceShape::TShape => PURPLE,
            PieceShape::RightZig => GREEN,
            PieceShape::LeftZig => RED,
            PieceShape::Square => YELLOW,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Piece {
    cells: [usize; 4],
    color: Color,
    shape: PieceShape,
    orientation: Orientation,
}

impl Piece {
    pub fn create(shape: PieceShape) -> Piece {
        let cells = match shape {
            PieceShape::Straight => [33, 34, 35, 36],
            PieceShape::LShape => [34, 33, 25, 35],
            PieceShape::BackwardLShape => [34, 33, 23, 35],
            PieceShape::TShape => [34, 33, 35, 24],
            PieceShape::RightZig => [34, 33, 24, 25],
            PieceShape::LeftZig => [34, 24, 23, 35],
            PieceShape::Square => [24, 25, 34, 35],
        };

        Piece {
            cells: cells,
            color: shape.get_color(),
            shape: shape,
            orientation: Orientation::Up,
        }
    }

    pub fn move_to(&mut self, new_posit: [usize; 4]) {
        self.cells = new_posit;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn cells(&self) -> [usize; 4] {
        self.cells
    }

    pub fn potential_cw_posit(&self) -> [usize; 4] {
        match self.shape {
            PieceShape::Square => self.cells,
            PieceShape::Straight => self.i_cw(),
            _ => self.cw(),
        }
    }

    pub fn potential_counter_cw_posit(&self) -> [usize; 4] {
        match self.shape {
            PieceShape::Square => self.cells,
            PieceShape::Straight => self.i_counter_cw(),
            _ => unimplemented!(),
        }
    }

    pub fn potential_down_posit(&self) -> [usize; 4] {
        let mut new_posit: [usize; 4] = [240; 4];
        for i in 0..4 {
            new_posit[i] = self.cells[i] + 10;
        }
        new_posit
    }

    pub fn potential_right_posit(&self) -> [usize; 4] {
        let mut new_posit: [usize; 4] = [240; 4];
        for i in 0..4 {
            new_posit[i] = self.cells[i] + 1;
        }
        new_posit
    }

    pub fn potential_left_posit(&self) -> [usize; 4] {
        let mut new_posit: [usize; 4] = [240; 4];
        for i in 0..4 {
            new_posit[i] = self.cells[i] - 1;
        }
        new_posit
    }

    pub fn rotate_orient_cw(&mut self) {
        use self::Orientation::*;

        match self.orientation {
            Up => self.orientation = Right,
            Right => self.orientation = Down,
            Down => self.orientation = Left,
            Left => self.orientation = Up,
        };
    }

    pub fn rotate_counter_cw(&mut self) {
        let potential_posit = self.potential_counter_cw_posit();
        if !Self::is_not_off_side(potential_posit) {
            self.cells = potential_posit;
        }
    }

    fn i_cw(&self) -> [usize; 4] {
        assert!(self.shape == PieceShape::Straight);
        match self.orientation {
            Orientation::Up => {
                [self.cells[0] - 8, self.cells[1] + 1, self.cells[2] + 10, self.cells[3] + 19]
            }
            Orientation::Right => {
                [self.cells[0] + 21, self.cells[1] + 10, self.cells[2] - 1, self.cells[3] - 12]
            }
            Orientation::Down => {
                [self.cells[0] + 8, self.cells[1] - 1, self.cells[2] - 10, self.cells[3] - 19]
            }
            Orientation::Left => {
                [self.cells[0] - 21, self.cells[1] - 10, self.cells[2] + 1, self.cells[3] + 12]
            }
        }
    }

    fn i_counter_cw(&self) -> [usize; 4] {
        assert!(self.shape == PieceShape::Straight);
        match self.orientation {
            Orientation::Up => {
                [self.cells[0] + 8, self.cells[1] - 1, self.cells[2] - 10, self.cells[3] - 19]
            }
            Orientation::Right => {
                [self.cells[0] - 21, self.cells[1] - 10, self.cells[2] + 1, self.cells[3] + 12]
            }
            Orientation::Down => {
                [self.cells[0] - 8, self.cells[1] + 1, self.cells[2] + 10, self.cells[3] + 19]
            }
            Orientation::Left => {
                [self.cells[0] + 21, self.cells[1] + 10, self.cells[2] - 1, self.cells[3] - 12]
            }
        }
    }

    fn cw(&self) -> [usize; 4] {
        let anchor = self.cells[0];
        let mut new_posit: [usize; 4] = [240; 4];
        new_posit[0] = anchor;
        for i in 1..4 {
            let cell = self.cells[i];
            if cell > anchor {
                match (cell - anchor) {
                    1 => new_posit[i] = anchor + 10,
                    11 => new_posit[i] = anchor + 9,
                    10 => new_posit[i] = anchor - 1,
                    9 => new_posit[i] = anchor - 11,
                    _ => panic!("Shouldn't have gotten here!"),
                };
            } else if anchor > cell {
                match (anchor - cell) {
                    1 => new_posit[i] = anchor - 10,
                    11 => new_posit[i] = anchor - 9,
                    10 => new_posit[i] = anchor + 1,
                    9 => new_posit[i] = anchor + 11,
                    _ => panic!("Shouldn't have gotten here!"),
                };
            } else {
                panic!("Something broke trying to rotate CW");
            }
        }
        new_posit
    }

    fn counter_cw(&self) -> [usize; 4] {
        unimplemented!();
    }

    // Very hacky solution. Especially the wrap around subtraction.
    fn is_not_off_side(mut posit: [usize; 4]) -> bool {
        for i in 0..4 {
            posit[i] %= 10;
        }
        for i in 0..4 {
            for j in i + 1..4 {
                let value = posit[i].wrapping_sub(posit[j]);
                if value > 4 && value < 400 {
                    return false;
                }
            }
        }
        return true;
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
            active_piece: Piece::create(generator.pop()),
            piece_generator: generator,
            game_over: false,
        };
        info!("Grid successfully initialized.");
        grid
    }

    pub fn new_piece(&mut self) {
        let piece_shape = self.piece_generator.pop();
        self.active_piece = Piece::create(piece_shape);
    }

    pub fn close_cell(&mut self, cell_num: usize, color: Color) -> Result<(), GridError> {
        debug!("Closing cell: {}", cell_num);
        self.change_cell_status(cell_num, CellStatus::Closed, color)
    }

    pub fn open_cell(&mut self, cell_num: usize) -> Result<(), GridError> {
        debug!("Opening cell: {}", cell_num);
        self.change_cell_status(cell_num, CellStatus::Open, OPEN_COLOR)
    }

    pub fn activate_cell(&mut self, cell_num: usize, color: Color) -> Result<(), GridError> {
        debug!("Activating cell: {}", cell_num);
        self.change_cell_status(cell_num, CellStatus::Active, color)
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
                let cell_num = self.active_piece.cells[i];
                let color = self.active_piece.color();
                self.close_cell(cell_num, color).unwrap();
            }
            self.new_piece();
        }
        self.check_rows();
    }

    fn move_active_to(&mut self, new_posit: [usize; 4]) {
        for i in 0..4 {
            let cell_num = self.active_piece.cells[i];
            self.open_cell(cell_num).unwrap();
        }
        self.active_piece.move_to(new_posit);
        for i in 0..4 {
            let cell_num = new_posit[i];
            let color = self.active_piece.color();
            self.activate_cell(cell_num, color).unwrap();
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

    pub fn rotate_active_ccw(&mut self) {}

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

    // returns true if the piece can move down
    fn check_posit(&self, mut posit: [usize; 4]) -> bool {
        for i in 0..4 {
            let index = posit[i];
            if !self.is_valid_cell(index) || self.cells.get(index).unwrap().get_status() == CellStatus::Closed {
                return false;
            }
        }
        Piece::is_not_off_side(posit)
    }

    fn change_cell_status(&mut self,
                          cell_num: usize,
                          new_status: CellStatus,
                          new_color: Color)
                          -> Result<(), GridError> {
        if !self.is_valid_cell(cell_num) {
            error!("Invalid cell number: {}", cell_num);
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

pub struct PieceGenerator {
    pieces: VecDeque<PieceShape>,
}

impl PieceGenerator {
    pub fn new() -> PieceGenerator {
        let pieces = VecDeque::with_capacity(10);
        let mut generator = PieceGenerator { pieces: pieces };
        generator.populate();
        generator
    }

    pub fn pop(&mut self) -> PieceShape {
        if self.pieces.len() < 4 {
            self.populate();
        }
        self.pieces.pop_front().unwrap()
    }

    fn push(&mut self, piece: PieceShape) {
        self.pieces.push_front(piece);
    }

    fn populate(&mut self) {
        use self::PieceShape::*;
        let mut new_pieces: VecDeque<_> = vec![Straight,
                                               LShape,
                                               BackwardLShape,
                                               TShape,
                                               RightZig,
                                               LeftZig,
                                               Square]
                                              .into_iter()
                                              .collect();
        let mut rng = rand::thread_rng();
        for i in (1..7).rev() {
            let index = rng.gen_range(0, i);
            self.push(new_pieces.swap_remove_back(index).unwrap());
        }
        self.push(new_pieces.remove(0).unwrap());
    }
}
