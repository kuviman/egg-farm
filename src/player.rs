use super::*;

pub struct Player {
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub target_vel: Vec2<f32>,
    pub vel: Vec2<f32>,
    pub max_speed: f32,
    pub stage: Stage,
    pub leg_walk_phase: f32,
    pub stand_timer: f32,
}

impl Player {
    pub fn new(pos: Vec2<f32>) -> Self {
        Self {
            pos,
            radius: 0.3,
            target_vel: vec2(0.0, 0.0),
            vel: vec2(0.0, 0.0),
            max_speed: 4.0,
            stage: Stage::Start,
            leg_walk_phase: 0.0,
            stand_timer: 0.0,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        const ACCEL: f32 = 20.0;
        let dv = self.target_vel * self.max_speed - self.vel;
        if dv.len() > 1e-5 {
            self.vel += dv.normalize() * (ACCEL * delta_time).min(dv.len())
        }
        self.pos += self.vel * delta_time;
        if self.vel.len() > 1e-5 {
            self.stand_timer -= delta_time * 5.0;
        } else {
            self.stand_timer += delta_time * 5.0;
        }
        self.stand_timer = clamp(self.stand_timer, 0.0..=1.0);
        self.leg_walk_phase += delta_time * 30.0;
    }
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        primitive: &Primitive,
    ) {
        if self.stage >= Stage::Born {
            const WALK_AMP: f32 = 0.3;
            const LEG_X: f32 = 0.6;
            const LEG_RADIUS: f32 = 0.3;
            let left_y = self.leg_walk_phase.sin().max(0.0) / 2.0 * (1.0 - self.stand_timer);
            primitive.circle(
                framebuffer,
                camera,
                self.pos
                    + vec2(
                        -self.radius * LEG_X,
                        -self.radius * (1.0 - LEG_RADIUS - left_y * WALK_AMP),
                    ),
                self.radius * LEG_RADIUS,
                Color::BLACK,
            );
            let right_y = -self.leg_walk_phase.sin().min(0.0) / 2.0 * (1.0 - self.stand_timer);
            primitive.circle(
                framebuffer,
                camera,
                self.pos
                    + vec2(
                        self.radius * LEG_X,
                        -self.radius * (1.0 - LEG_RADIUS - right_y * WALK_AMP),
                    ),
                self.radius * LEG_RADIUS,
                Color::BLACK,
            );
        }
        primitive.circle(framebuffer, camera, self.pos, self.radius, Color::BLACK);
        primitive.circle(
            framebuffer,
            camera,
            self.pos,
            self.radius * 0.8,
            Color::WHITE,
        );

        if self.stage >= Stage::Born {
            const EYE_X: f32 = 0.3;
            const EYE_Y: f32 = 0.3;
            const EYE_RADIUS: f32 = 0.2;
            primitive.circle(
                framebuffer,
                camera,
                self.pos + vec2(EYE_X, EYE_Y) * self.radius,
                self.radius * EYE_RADIUS,
                Color::BLACK,
            );
            primitive.circle(
                framebuffer,
                camera,
                self.pos + vec2(-EYE_X, EYE_Y) * self.radius,
                self.radius * EYE_RADIUS,
                Color::BLACK,
            );
        }
    }
}
