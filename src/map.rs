use super::*;

pub struct Map {}

impl Map {
    pub fn new() -> Self {
        Self {}
    }
    pub fn size(&self) -> Vec2<usize> {
        vec2(16, 16)
    }
}
