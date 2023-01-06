use crate::consts;
use raylib::prelude::*;

pub fn draw_background(gfx: &mut RaylibDrawHandle) {
  for (x, y) in Coords::new() {
    draw_grid_cell(gfx, Vector2::new(x as f32, y as f32));
  }
}

const CELL_PRODUCT: i32 = (consts::BOARD_WIDTH * consts::BOARD_HEIGHT) as i32;

/// This is a bit weird. This struct is an iterator over all the possible board
/// positions. Here's a basic representation of the values that it produces. You
/// would read the first column from top to bottom, then the second from top to
/// bottom, etc.
///
/// ```text
/// (0, 0)  | (1, 0)  | (2, 0)  | ... | (16, 0)
/// (0, 1)  | (1, 1)  | (2, 1)  |     | (16, 1)
/// (0, 2)  | (1, 2)  | (2, 2)  |     | (16, 2)
/// ...     | ...     | ...     |     | ...
/// (0, 14) | (1, 14) | (2, 14) |     | (16, 14)
/// ```
pub struct Coords {
  iteration: i32,
}

impl Coords {
  pub fn new() -> Self {
    Self { iteration: 0 }
  }
}

impl Iterator for Coords {
  type Item = (i32, i32);

  fn next(&mut self) -> Option<Self::Item> {
    if self.iteration < CELL_PRODUCT {
      let last_iteration = self.iteration;
      self.iteration += 1;
      return Some((
        last_iteration / consts::BOARD_HEIGHT as i32,
        last_iteration % consts::BOARD_HEIGHT as i32,
      ));
    }
    None
  }
}

fn draw_grid_cell(gfx: &mut RaylibDrawHandle, position: Vector2) {
  gfx.draw_rectangle_v(
    position * consts::CELL_WIDTH,
    consts::CELL_SIZE,
    if position.x % 2.0 == position.y % 2.0 {
      consts::GRID_COLOR_LIGHT
    } else {
      consts::GRID_COLOR_DARK
    },
  );
}
