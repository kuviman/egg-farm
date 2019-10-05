use geng::prelude::*;

mod camera;
mod primitive;

use camera::*;
use primitive::*;

struct Game {
    geng: Rc<Geng>,
    camera: Camera,
    primitive: Primitive,
}

impl Game {
    fn new(geng: &Rc<Geng>) -> Self {
        Self {
            geng: geng.clone(),
            camera: Camera::new(30.0),
            primitive: Primitive::new(geng),
        }
    }
}

impl geng::State for Game {
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        self.primitive
            .circle(framebuffer, &self.camera, vec2(0.0, 0.0), 0.5, Color::BLACK);
        self.primitive
            .circle(framebuffer, &self.camera, vec2(0.0, 0.0), 0.4, Color::WHITE);
    }
    fn handle_event(&mut self, event: geng::Event) {}
}

fn main() {
    let geng = Rc::new(Geng::new(geng::ContextOptions {
        title: "Egg Farm".to_owned(),
        ..default()
    }));
    let game = Game::new(&geng);
    geng::run(geng, game);
}
