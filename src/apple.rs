use crate::{consts, entity::Entity};
use rand::Rng;
use raylib::prelude::*;

pub struct Apple {
  position: Vector2,
}

impl Apple {
  pub fn draw(&self, gfx: &mut RaylibDrawHandle) {
    gfx.draw_rectangle_v(
      self.position * consts::CELL_WIDTH,
      Vector2::new(consts::CELL_WIDTH, consts::CELL_WIDTH),
      Color::RED,
    );
  }

  pub fn new() -> Self {
    Self {
      // body: Block::new(
      //   consts::APPLE_START_POSITION.x,
      //   consts::APPLE_START_POSITION.y,
      // ),
      position: Vector2::new(
        consts::APPLE_START_POSITION.x,
        consts::APPLE_START_POSITION.y,
      ),
    }
  }

  pub fn randomize_position(&mut self) {
    self.position.x = rand::thread_rng().gen_range(0..consts::BOARD_WIDTH) as f32;
    self.position.y = rand::thread_rng().gen_range(0..consts::BOARD_HEIGHT) as f32;
  }
}

impl Entity for Apple {
  fn position(&self) -> Vector2 {
    self.position
  }
}
