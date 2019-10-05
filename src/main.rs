use geng::prelude::*;

mod camera;
mod map;
mod player;
mod primitive;

use camera::*;
use map::*;
use player::*;
use primitive::*;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Stage {
    Start,
    Moving,
}

impl Stage {
    fn help(&self) -> &str {
        match self {
            Self::Start => "Use WASD to move around",
            Self::Moving => "Try to break the wall",
        }
    }
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
        self.camera.target_fov = if self.stage == Stage::Start {
            5.0
        } else {
            max(self.map.size().x, self.map.size().y) as f32 + 5.0
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
        if self.stage == Stage::Start && (self.player.pos - self.camera.center).len() > 1.0 {
            self.stage = Stage::Moving;
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
        let help_pos = self.camera.screen_to_world(framebuffer, vec2(0.0, 0.0))
            + vec2(self.camera.fov / 20.0, 0.0);
        self.primitive.text_bubble(
            framebuffer,
            &self.camera,
            "?",
            help_pos,
            self.camera.fov / 20.0,
        );
        let text = if (mouse_pos - help_pos - vec2(0.0, self.camera.fov * 3.0 / 40.0)).len()
            < self.camera.fov / 20.0
        {
            self.stage.help().to_owned()
        } else {
            self.text_at(mouse_pos)
        };
        self.primitive.text_bubble(
            framebuffer,
            &self.camera,
            &text,
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
