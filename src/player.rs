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
    pub jump: Option<f32>,
    pub want_jump: bool,
    pub landed: bool,
    pub eaten: bool,
}

impl Player {
    pub fn new(pos: Vec2<f32>) -> Self {
        Self {
            pos,
            radius: 0.5,
            target_vel: vec2(0.0, 0.0),
            vel: vec2(0.0, 0.0),
            max_speed: 4.0,
            stage: Stage::Start,
            leg_walk_phase: 0.0,
            stand_timer: 0.0,
            jump: None,
            want_jump: false,
            landed: false,
            eaten: false,
        }
    }
    pub fn landed(&mut self) -> bool {
        if self.landed {
            self.landed = false;
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        if self.stage >= Stage::Born && (self.want_jump || self.jump.is_some()) {
            self.vel = vec2(0.0, 0.0);
            if self.jump.is_none() {
                self.jump = Some(1.0);
            }
            let time_left = self.jump.unwrap() - delta_time * 3.0;
            if time_left < 0.0 {
                self.jump = None;
                self.landed = true;
            } else {
                self.jump = Some(time_left);
            }
        } else {
            const ACCEL: f32 = 20.0;
            let dv = self.target_vel * self.max_speed - self.vel;
            if dv.len() > 1e-5 {
                self.vel += dv.normalize() * (ACCEL * delta_time).min(dv.len())
            }
            self.pos += self.vel * delta_time;
        }
        self.want_jump = false;
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
        brokes: Option<usize>,
    ) {
        let mut pos_with_jump = self.pos;
        if let Some(time) = self.jump {
            pos_with_jump.y += (1.0 - ((time - 0.5) * 2.0).powi(2)) * 0.5;
        }
        if self.stage >= Stage::Born {
            let amp = if self.jump.is_some() { 0.0 } else { 0.3 };
            let leg_x = if self.jump.is_some() { 0.8 } else { 0.6 };
            let leg_y = if self.jump.is_some() { 0.3 } else { 0.0 };
            const LEG_RADIUS: f32 = 0.3;
            let left_y = self.leg_walk_phase.sin().max(0.0) / 2.0 * (1.0 - self.stand_timer);
            primitive.circle(
                framebuffer,
                camera,
                pos_with_jump
                    + vec2(-leg_x, -(1.0 - LEG_RADIUS - left_y * amp - leg_y)) * self.radius,
                self.radius * LEG_RADIUS,
                Color::BLACK,
            );
            let right_y = -self.leg_walk_phase.sin().min(0.0) / 2.0 * (1.0 - self.stand_timer);
            primitive.circle(
                framebuffer,
                camera,
                pos_with_jump
                    + vec2(leg_x, -(1.0 - LEG_RADIUS - right_y * amp - leg_y)) * self.radius,
                self.radius * LEG_RADIUS,
                Color::BLACK,
            );
        }
        primitive.circle(
            framebuffer,
            camera,
            pos_with_jump,
            self.radius,
            Color::BLACK,
        );
        primitive.circle(
            framebuffer,
            camera,
            pos_with_jump,
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
                pos_with_jump + vec2(EYE_X, EYE_Y) * self.radius,
                self.radius * EYE_RADIUS,
                Color::BLACK,
            );
            primitive.circle(
                framebuffer,
                camera,
                pos_with_jump + vec2(-EYE_X, EYE_Y) * self.radius,
                self.radius * EYE_RADIUS,
                Color::BLACK,
            );
        }

        if let Some(brokes) = brokes {
            for i in 0..brokes {
                let pos_with_jump =
                    pos_with_jump + vec2(0.2 - i as f32 * 0.2, -0.2 + i as f32 * 0.2);
                primitive.line(
                    framebuffer,
                    camera,
                    pos_with_jump + vec2(-0.2, 0.2) * self.radius,
                    pos_with_jump + vec2(-0.2, -0.2) * self.radius,
                    self.radius * 0.2,
                    Color::BLACK,
                );
                primitive.line(
                    framebuffer,
                    camera,
                    pos_with_jump + vec2(0.3, -0.2) * self.radius,
                    pos_with_jump + vec2(-0.3, -0.2) * self.radius,
                    self.radius * 0.2,
                    Color::BLACK,
                );
            }
        }
    }
}
