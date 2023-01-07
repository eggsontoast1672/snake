use crate::{consts, entity::Entity, math::Vector2};
use rand::Rng;
use raylib::{
  drawing::RaylibDrawHandle,
  prelude::{Color, RaylibDraw},
};

pub struct Apple {
  position: Vector2,
}

impl Apple {
  pub fn draw(&self, gfx: &mut RaylibDrawHandle) {
    gfx.draw_rectangle_v(
      Vector2::new(
        self.position.x * consts::CELL_SIZE,
        self.position.y * consts::CELL_SIZE,
      ),
      Vector2::new(consts::CELL_SIZE, consts::CELL_SIZE),
      Color::RED,
    );
  }

  pub fn new() -> Self {
    Self {
      position: consts::APPLE_START_POSITION,
    }
  }

  pub fn randomize_position(&mut self) {
    self.position.x = rand::thread_rng().gen_range(0..consts::BOARD_WIDTH);
    self.position.y = rand::thread_rng().gen_range(0..consts::BOARD_HEIGHT);
  }
}

impl Entity for Apple {
  fn position(&self) -> Vector2 {
    self.position
  }
}
