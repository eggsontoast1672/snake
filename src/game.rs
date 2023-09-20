use raylib::prelude::*;

use crate::consts;

const SCREEN_WIDTH: i32 = consts::BOARD_WIDTH * consts::CELL_SIZE;
const SCREEN_HEIGHT: i32 = consts::BOARD_HEIGHT * consts::CELL_SIZE;

pub struct DrawingContext<'a> {
    data: RaylibDrawHandle<'a>,
}

impl<'a> DrawingContext<'a> {
    fn new(context: &'a mut RaylibHandle, thread: &RaylibThread) -> Self {
        return Self {
            data: context.begin_drawing(thread),
        };
    }
}

pub struct Game {
    handle: RaylibHandle,
    thread: RaylibThread,
}

impl Game {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init()
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Snake!")
            .build();
        return Self { handle, thread };
    }

    pub fn randomize_apple_position(&mut self) {
        todo!()
    }
}
