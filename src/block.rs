use raylib::prelude::*;

use crate::entity::Entity;

pub struct Block {
  position: Vector2,
}

impl Block {
  pub fn _new(x: f32, y: f32) -> Self {
    Self {
      position: Vector2::new(x, y),
    }
  }
}

impl Entity for Block {
  fn position(&self) -> Vector2 {
    self.position
  }
}

impl From<Vector2> for Block {
  fn from(position: Vector2) -> Self {
    Self { position }
  }
}
