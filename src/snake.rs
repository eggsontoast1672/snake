use crate::{block::Block, consts, entity::Entity, math::Vector2};
use raylib::{consts::KeyboardKey, drawing::RaylibDrawHandle, prelude::RaylibDraw, RaylibHandle};

#[derive(PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn as_vector(&self) -> Vector2 {
    match self {
      Self::Up => Vector2::new(0, -1),
      Self::Down => Vector2::new(0, 1),
      Self::Left => Vector2::new(-1, 0),
      Self::Right => Vector2::new(1, 0),
    }
  }
}

pub struct Snake {
  body: Vec<Block>,
  dead: bool,
  direction: Direction,
  length_increasing: bool,
}

impl Snake {
  pub fn draw(&self, gfx: &mut RaylibDrawHandle) {
    for block in self.body.iter() {
      let block_position = block.position();
      gfx.draw_rectangle_v(
        Vector2::new(
          block_position.x * consts::CELL_SIZE,
          block_position.y * consts::CELL_SIZE,
        ),
        // block.position() * consts::CELL_SIZE,
        Vector2::new(consts::CELL_SIZE, consts::CELL_SIZE),
        consts::SNAKE_COLOR,
      );
    }
  }

  pub fn handle_input(&mut self, ctx: &RaylibHandle) {
    // We can use if-else as opposed to just consecutive if's because if, say,
    // the down arrow key was pressed this frame, in a game like this, it's
    // pointless to check the other keys that frame.
    if ctx.is_key_pressed(KeyboardKey::KEY_UP) && self.direction != Direction::Down {
      self.direction = Direction::Up;
    } else if ctx.is_key_pressed(KeyboardKey::KEY_DOWN) && self.direction != Direction::Up {
      self.direction = Direction::Down;
    } else if ctx.is_key_pressed(KeyboardKey::KEY_LEFT) && self.direction != Direction::Right {
      self.direction = Direction::Left;
    } else if ctx.is_key_pressed(KeyboardKey::KEY_RIGHT) && self.direction != Direction::Left {
      self.direction = Direction::Right;
    };
  }

  pub fn is_dead(&self) -> bool {
    self.dead
  }

  pub fn increase_length(&mut self) {
    self.length_increasing = true;
  }

  fn increase_length_if(&mut self) {
    if self.length_increasing {
      self.length_increasing = false;
      return;
    }
    self.body.pop();
  }

  pub fn is_touching(&self, entity: &impl Entity) -> bool {
    for block in self.body.iter() {
      if block.position() == entity.position() {
        return true;
      }
    }
    false
  }

  pub fn is_touching_tail(&self) -> bool {
    for block in self.body.iter().skip(1) {
      if block.position() == self.body[0].position() {
        return true;
      }
    }
    false
  }

  pub fn new() -> Self {
    Self {
      body: consts::SNAKE_STARTING_POSITIONS
        .iter()
        .map(|position| Block::from(*position))
        .collect(),
      dead: false,
      direction: Direction::Right,
      length_increasing: false,
    }
  }

  pub fn update(&mut self, ctx: &mut RaylibHandle) {
    self.handle_input(ctx);
    self.update_position();

    if self.is_touching_tail() {
      self.dead = true;
    }
  }

  fn update_position(&mut self) {
    let head_position = self.body[0].position();
    let direction_vector = self.direction.as_vector();
    let new_position = Vector2::new(
      head_position.x + direction_vector.x,
      head_position.y + direction_vector.y,
    );
    if new_position.x < 0
      || new_position.y < 0
      || new_position.x + 1 > consts::BOARD_WIDTH
      || new_position.y + 1 > consts::BOARD_HEIGHT
    {
      // At this point, the snake is trying to move outside the bounds of the
      // screen.
      self.dead = true;
      return;
    }
    self.body.insert(0, Block::from(new_position));
    self.increase_length_if();
  }
}
