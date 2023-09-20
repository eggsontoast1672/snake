pub mod game;

mod consts;
mod math;

use raylib::prelude::*;

const fn color_from_rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}
