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
        fn close(pos: f32, size: usize) -> bool {
            pos.abs() < 0.5 || (pos - size as f32).abs() < 0.5
        }
        if close(pos.x, self.size().x) || close(pos.y, self.size().y) {
            return Some("Wall".to_owned());
        }
        None
    }
}
