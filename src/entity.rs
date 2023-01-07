use crate::math::Vector2;

pub trait Entity {
  fn position(&self) -> Vector2;
}
