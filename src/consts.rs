use crate::math::Vector2;
use raylib::color::Color;

pub const APPLE_START_POSITION: Vector2 = Vector2::new(12, 7);
pub const BOARD_WIDTH: i32 = 17;
pub const BOARD_HEIGHT: i32 = 15;
pub const GRID_COLOR_DARK: Color = crate::color_from_rgb(95, 167, 47);
pub const GRID_COLOR_LIGHT: Color = crate::color_from_rgb(119, 201, 64);
pub const SNAKE_STARTING_POSITIONS: [Vector2; 4] = [
  Vector2::new(4, 7),
  Vector2::new(3, 7),
  Vector2::new(2, 7),
  Vector2::new(1, 7),
];
pub const SNAKE_COLOR: Color = Color::BLUE;

// This can be changed in order to change how big the window is
pub const CELL_SIZE: i32 = 50;
