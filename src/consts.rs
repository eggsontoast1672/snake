use raylib::prelude::*;

// Board dimensions:      17x15
// Snake starting coords: (4, 7) (head)
//                        (3, 7) (body)
//                        (2, 7)   --
//                        (1, 7)   --
// Food starting coords:  (12, 7)

pub const APPLE_START_POSITION: Vector2 = Vector2::new(12.0, 7.0);

// Do not change these
pub const BOARD_WIDTH: u32 = 17;
pub const BOARD_HEIGHT: u32 = 15;

// Feel free to change this one :)
pub const CELL_WIDTH: f32 = 50.0;
pub const CELL_SIZE: Vector2 = Vector2::new(CELL_WIDTH, CELL_WIDTH);

pub const GRID_COLOR_LIGHT: Color = crate::color_from_rgb(119, 201, 64);
pub const GRID_COLOR_DARK: Color = crate::color_from_rgb(95, 167, 47);

// Snake settings
pub const SNAKE_STARTING_POSITIONS: [Vector2; 4] = [
  Vector2::new(4.0, 7.0),
  Vector2::new(3.0, 7.0),
  Vector2::new(2.0, 7.0),
  Vector2::new(1.0, 7.0),
];
pub const SNAKE_COLOR: Color = Color::BLUE;
