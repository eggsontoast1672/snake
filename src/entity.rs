use raylib::prelude::*;

pub trait Entity {
  fn position(&self) -> Vector2;
}
