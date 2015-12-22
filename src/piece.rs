use std::collections::VecDeque;

use rand;
use rand::Rng;


pub mod colors {
    pub use piston_window::types::Color;

    pub const OPEN_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
    pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

    pub const LIGHT_BLUE: Color = [0.0, 1.0, 1.0, 1.0];
    pub const BLUE: Color = [0.0, 60.0 / 255.0, 1.0, 1.0];
    pub const ORANGE: Color = [1.0, 174.0 / 255.0, 0.0, 1.0];
    pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
    pub const GREEN: Color = [30.0 / 255.0, 1.0, 0.0, 1.0];
    pub const RED: Color = [1.0, 30.0 / 255.0, 0.0, 1.0];
    pub const PURPLE: Color = [220.0 / 255.0, 0.0, 1.0, 1.0];
}
use self::colors::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PieceShape {
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
pub struct Piece {
    cells: [usize; 4],
    color: Color,
    shape: PieceShape,
    orientation: Orientation,
}

impl Piece {
    fn create(shape: PieceShape) -> Piece {
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

    pub fn get_cell(&self, index: usize) -> usize {
        self.cells[index]
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

    fn cw(&self) -> [usize; 4] {
        let anchor = self.cells[0];
        let mut new_posit: [usize; 4] = [240; 4];
        new_posit[0] = anchor;
        for i in 1..4 {
            let cell = self.cells[i];
            if cell > anchor {
                match cell - anchor {
                    1 => new_posit[i] = anchor + 10,
                    11 => new_posit[i] = anchor + 9,
                    10 => new_posit[i] = anchor - 1,
                    9 => new_posit[i] = anchor - 11,
                    _ => panic!("Shouldn't have gotten here!"),
                };
            } else if anchor > cell {
                match anchor - cell {
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

    pub fn is_not_off_side(posit: [usize; 4]) -> bool {
        let mut has_zero = false;
        let mut has_nine = false;
        for i in 0..4 {
            let modulo = posit[i] % 10;
            if modulo == 0 {
                has_zero = true;
            }
            if modulo == 9 {
                has_nine = true;
            }
        }
        !(has_zero && has_nine)
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

    pub fn pop(&mut self) -> Piece {
        if self.pieces.len() < 4 {
            self.populate();
        }
        let shape = self.pieces.pop_front().expect("Poped when piece generator was empty.");

        Piece::create(shape)
    }

    fn push(&mut self, piece: PieceShape) {
        self.pieces.push_back(piece);
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
            let new_piece = new_pieces.swap_remove_back(index).unwrap();
            self.push(new_piece);
        }
        self.push(new_pieces.remove(0).unwrap());
    }
}
