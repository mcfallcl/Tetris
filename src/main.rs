extern crate piston_window;
extern crate tetris;

use piston_window::*;
use piston_window::rectangle::{Shape, Border};

use tetris::{Grid, Piece, CellStatus, GridError};
use self::tetris::logger;

const grid_width: f64 = 215.0;
const grid_height: f64 = 400.0;
const grid_background_color: [f32; 4] = [0.95, 0.95, 0.95, 1.0];

const cell_width: f64 = grid_width / 10.0;
const cell_height: f64 = grid_height / 20.0;

const background_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    logger::init_debug();
    let mut exit = false;
    let mut grid = Grid::init();
    let mut window: PistonWindow = WindowSettings::new("Tak's Tetris", (640, 480))
                                       .exit_on_esc(true)
                                       .build()
                                       .unwrap();

    grid.active_cell(4, [0.5, 0.5, 0.5, 1.0]);
    grid.active_cell(14, [0.5, 0.5, 0.5, 1.0]);
    grid.active_cell(24, [0.5, 0.5, 0.5, 1.0]);
    grid.active_cell(34, [0.5, 0.5, 0.5, 1.0]);

    for e in window {
        let grid_corner: f64 = 20.0;
        let game_grid: [f64; 4] = [grid_corner, grid_corner, grid_width, grid_height];
        e.draw_2d(|c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            rectangle(grid_background_color, game_grid, c.transform, g);
            let mut i = 0u32;
            for cell in grid.get_cells() {
                if i >= 40 {
                    let cell_num = i - 40;
                    let color = cell.get_color();
                    let x = grid_corner + cell_width * (cell_num % 10) as f64;
                    let y = grid_corner + cell_height * (cell_num / 10) as f64;
                    let mut rect = Rectangle::new(color).shape(Shape::Square).border(Border {
                        color: [0.0, 0.0, 0.0, 1.0],
                        radius: 0.5,
                    });
                    rect.draw([x, y, cell_width, cell_height],
                              default_draw_state(),
                              c.transform,
                              g)
                }
                i += 1;
            }
        });
        grid.ping();
    }
}
