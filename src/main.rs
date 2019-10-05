use geng::prelude::*;

mod camera;
mod map;
mod player;
mod primitive;

use camera::*;
use map::*;
use player::*;
use primitive::*;

#[derive(Debug, Copy, Clone)]
pub enum Stage {
    Start,
    Moving,
}

pub struct Game {
    geng: Rc<Geng>,
    camera: Camera,
    map: Map,
    player: Player,
    stage: Stage,
    primitive: Primitive,
}

impl Game {
    fn new(geng: &Rc<Geng>) -> Self {
        let map = Map::new();
        let mut camera = Camera::new(0.1);
        camera.center = map.size().map(|x| x as f32) / 2.0;
        let player = Player::new(camera.center);
        Self {
            geng: geng.clone(),
            camera,
            map,
            player,
            stage: Stage::Start,
            primitive: Primitive::new(geng),
        }
    }
    fn text_at(&self, pos: Vec2<f32>) -> String {
        if let Some(text) = self.map.text_at(pos) {
            return text;
        }
        if (self.player.pos - pos).len() < self.player.radius {
            return "YOU".to_owned();
        }
        "Nothing".to_owned()
    }
}

impl geng::State for Game {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.camera.target_fov = match self.stage {
            Stage::Start => 5.0,
            _ => max(self.map.size().x, self.map.size().y) as f32 + 5.0,
        };
        self.camera.update(delta_time);
        self.player.target_vel = vec2(0.0, 0.0);
        if self.geng.window().is_key_pressed(geng::Key::W) {
            self.player.target_vel.y += 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::A) {
            self.player.target_vel.x -= 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::S) {
            self.player.target_vel.y -= 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::D) {
            self.player.target_vel.x += 1.0;
        }
        if self.player.target_vel.len() > 1e-5 {
            self.player.target_vel = self.player.target_vel.normalize();
        }
        self.player.update(delta_time);
        if (self.player.pos - self.camera.center).len() > 1.0 {
            match self.stage {
                Stage::Start => {
                    self.stage = Stage::Moving;
                }
                _ => {}
            }
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        self.map
            .draw(framebuffer, &self.camera, &self.primitive, self.stage);
        self.player.draw(framebuffer, &self.camera, &self.primitive);
        let mouse_pos = self.camera.screen_to_world(
            framebuffer,
            self.geng.window().mouse_pos().map(|x| x as f32),
        );
        self.primitive.text_bubble(
            framebuffer,
            &self.camera,
            &self.text_at(mouse_pos),
            mouse_pos,
            self.camera.fov / 30.0,
        );
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
