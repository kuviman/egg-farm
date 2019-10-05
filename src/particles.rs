use super::*;

struct Particle {
    pos: Vec2<f32>,
    size: f32,
    vel: Vec2<f32>,
    t: f32,
}

impl Particle {
    fn update(&mut self, delta_time: f32) {
        self.pos += self.vel * delta_time * 2.0;
        self.t += delta_time * 3.0;
    }
    fn radius(&self) -> f32 {
        self.size * (1.0 - (self.t - 1.0).powi(2))
    }
}

pub struct Particles {
    particles: Vec<Particle>,
}

impl Particles {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }
    pub fn spawn(&mut self, pos: Vec2<f32>, size: f32) {
        self.particles.push(Particle {
            pos,
            size,
            vel: Vec2::rotated(
                vec2(size, 0.0),
                global_rng().gen_range(0.0, 2.0 * std::f32::consts::PI),
            ),
            t: 0.0,
        })
    }
    pub fn boom(&mut self, pos: Vec2<f32>) {
        for _ in 0..10 {
            self.spawn(pos, 0.4);
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.update(delta_time);
        }
        self.particles.retain(|particle| particle.t < 2.0);
    }
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        primitive: &Primitive,
    ) {
        for particle in &self.particles {
            primitive.circle(
                framebuffer,
                camera,
                particle.pos,
                particle.radius(),
                Color::BLACK,
            );
        }
        for particle in &self.particles {
            primitive.circle(
                framebuffer,
                camera,
                particle.pos,
                (particle.radius() - 0.1).max(0.0),
                Color::WHITE,
            );
        }
    }
}
