use super::*;

pub struct Player {
    pos: Vec2<f32>,
    radius: f32,
}

impl Player {
    pub fn new(pos: Vec2<f32>) -> Self {
        Self { pos, radius: 0.3 }
    }
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        primitive: &Primitive,
    ) {
        primitive.circle(framebuffer, camera, self.pos, self.radius, Color::BLACK);
        primitive.circle(
            framebuffer,
            camera,
            self.pos,
            self.radius * 0.8,
            Color::WHITE,
        );
    }
}
