use super::*;

pub struct Player {
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub target_vel: Vec2<f32>,
    pub vel: Vec2<f32>,
    pub max_speed: f32,
}

impl Player {
    pub fn new(pos: Vec2<f32>) -> Self {
        Self {
            pos,
            radius: 0.3,
            target_vel: vec2(0.0, 0.0),
            vel: vec2(0.0, 0.0),
            max_speed: 4.0,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        const ACCEL: f32 = 10.0;
        let dv = (self.target_vel * self.max_speed - self.vel);
        if dv.len() > 1e-5 {
            self.vel += dv.normalize() * (ACCEL * delta_time).min(dv.len())
        }
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
            self.radius * 0.8,
            Color::WHITE,
        );
    }
}
