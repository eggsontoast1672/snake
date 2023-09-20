// The only reason I need to implement my own vector type is that the
// nalgebra_interop feature is broken. Very dumb.

use raylib::ffi;

#[derive(Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Vector2> for ffi::Vector2 {
    fn from(vector: Vector2) -> Self {
        Self {
            x: vector.x as f32,
            y: vector.y as f32,
        }
    }
}
