extern crate piston_window;
extern crate tetris;

use piston_window::*;
use piston_window::rectangle::{Shape, Border};

use tetris::{Grid, CycleTimer};
use self::tetris::logger;

const GRID_WIDTH: f64 = 215.0;
const GRID_HEIGHT: f64 = 400.0;
const GRID_BACKGROUND_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];

const CELL_WIDTH: f64 = GRID_WIDTH / 10.0;
const CELL_HEIGHT: f64 = GRID_HEIGHT / 20.0;

const BACKGROUND_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    logger::init_debug().unwrap_or_else(|e| println!("Error initializing the logger: {:?}", e));
    let mut grid = Grid::init();
    let window: PistonWindow = WindowSettings::new("Tak's Tetris", (640, 480))
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();

    let mut timer = CycleTimer::new(800);
    grid.new_piece();

    for e in window {
        let grid_corner: f64 = 20.0;
        let game_grid: [f64; 4] = [grid_corner, grid_corner, GRID_WIDTH, GRID_HEIGHT];

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Left) => grid.move_active_left(),
                Button::Keyboard(Key::Right) => grid.move_active_right(),
                Button::Keyboard(Key::Up) => grid.rotate_active_cw(),
                Button::Keyboard(Key::Down) => {
                    grid.move_active_down();
                    timer.reset();
                }
                _ => {}
            };
        }

        e.draw_2d(|c, g| {
            clear(BACKGROUND_COLOR, g);
            rectangle(GRID_BACKGROUND_COLOR, game_grid, c.transform, g);
            let mut i = 0u32;
            for cell in grid.iter_mut() {
                if i >= 40 {
                    let cell_num = i - 40;
                    let color = cell.get_color();
                    let x = grid_corner + CELL_WIDTH * (cell_num % 10) as f64;
                    let y = grid_corner + CELL_HEIGHT * (cell_num / 10) as f64;
                    let rect = Rectangle::new(color).shape(Shape::Square).border(Border {
                        color: [0.0, 0.0, 0.0, 1.0],
                        radius: 0.5,
                    });
                    rect.draw([x, y, CELL_WIDTH, CELL_HEIGHT],
                              default_draw_state(),
                              c.transform,
                              g);
                }
                i += 1;
            }
        });
        if timer.cycle() {
            grid.cycle();
        }
        if grid.is_game_over() {
            break;
        }
    }
}
