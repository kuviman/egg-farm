use super::*;

pub struct Projectile {
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub vel: Vec2<f32>,
    pub alive: bool,
}

impl Projectile {
    pub fn new(pos: Vec2<f32>, radius: f32, vel: Vec2<f32>) -> Self {
        Self {
            pos,
            radius,
            vel,
            alive: true,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        self.pos += self.vel * delta_time;
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
            self.radius - 0.1,
            Color::WHITE,
        );
    }
}
