use crate::{apple::Apple, snake::Snake};
use raylib::prelude::*;

mod apple;
mod block;
mod board;
mod consts;
mod entity;
mod snake;

const SCREEN_WIDTH: i32 = (consts::BOARD_WIDTH as f32 * consts::CELL_WIDTH) as i32;
const SCREEN_HEIGHT: i32 = (consts::BOARD_HEIGHT as f32 * consts::CELL_WIDTH) as i32;

/**
 * TODO: Implement dying.
 */
fn main() {
  let (mut ctx, thread) = raylib::init()
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .title("Snake!")
    .build();

  let mut snake = Snake::new();
  let mut apple = Apple::new();

  let mut score = 0u32;

  ctx.set_target_fps(10);

  while !ctx.window_should_close() {
    // === UPDATE ===
    snake.update(&mut ctx);

    if snake.is_dead() {
      snake = Snake::new();
      apple = Apple::new();

      score = 0;
    }

    if snake.is_touching(&apple) {
      loop {
        apple.randomize_position();
        if !snake.is_touching(&apple) {
          break;
        }
      }
      snake.increase_length();

      score += 1;

      println!("Score: {}", score);
    }

    // === DRAW ===
    let mut gfx = ctx.begin_drawing(&thread);

    gfx.clear_background(Color::BLACK);

    board::draw_background(&mut gfx);

    apple.draw(&mut gfx);
    snake.draw(&mut gfx);
  }
}

const fn color_from_rgb(r: u8, g: u8, b: u8) -> Color {
  Color { r, g, b, a: 255 }
}
