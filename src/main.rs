mod apple;
mod block;
mod board;
mod entity;
mod snake;

use crate::apple::Apple;
use crate::snake::Snake;
use raylib::prelude::*;

#[doc = r#"* TODO: Implement dying."#]
fn main() {
    let (mut ctx, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Snake!")
        .build();

    let mut snake = Snake::new();
    let mut apple = Apple::new();

    let mut score = 0u32;

    ctx.set_target_fps(5);

    while !ctx.window_should_close() {
        // === UPDATE ===
        snake.update(&mut ctx);

        if snake.is_dead() {
            snake = Snake::new();
            apple = Apple::new();

            score = 0;
        }

        if snake.is_touching(&apple) {
            // loop {
            //   apple.randomize_position();
            //   if !snake.is_touching(&apple) {
            //     break;
            //   }
            // }
            game.randomize_apple_position();
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

        game.with_drawing_context(|context| {
            context.draw(apple);
            context.draw(snake);
        });
    }
}
