use super::*;

pub struct Map {}

impl Map {
    pub fn new() -> Self {
        Self {}
    }
    pub fn size(&self) -> Vec2<usize> {
        vec2(16, 16)
    }
    pub fn text_at(&self, pos: Vec2<f32>) -> Option<String> {
        None
    }
}
