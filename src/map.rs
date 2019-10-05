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
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        primitive: &Primitive,
        stage: Stage,
    ) {
        if stage > Stage::Start {
            const BORDER_WIDTH: f32 = 0.1;
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(-BORDER_WIDTH, -BORDER_WIDTH),
                    vec2(BORDER_WIDTH, self.size().y as f32 + 2.0 * BORDER_WIDTH),
                ),
                Color::BLACK,
            );
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(-BORDER_WIDTH, -BORDER_WIDTH),
                    vec2(self.size().x as f32 + 2.0 * BORDER_WIDTH, BORDER_WIDTH),
                ),
                Color::BLACK,
            );
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(self.size().x as f32, -BORDER_WIDTH),
                    vec2(BORDER_WIDTH, self.size().y as f32 + 2.0 * BORDER_WIDTH),
                ),
                Color::BLACK,
            );
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(-BORDER_WIDTH, self.size().y as f32),
                    vec2(self.size().x as f32 + 2.0 * BORDER_WIDTH, BORDER_WIDTH),
                ),
                Color::BLACK,
            );
        }
    }
}
