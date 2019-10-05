use geng::prelude::*;

mod camera;
mod map;
mod particles;
mod player;
mod primitive;

use camera::*;
use map::*;
use particles::*;
use player::*;
use primitive::*;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Stage {
    Start,
    Moving,
    Born,
}

impl Stage {
    fn help(&self) -> &str {
        match self {
            Self::Start => "Use WASD to move around",
            Self::Moving => "Try to break the wall",
            Self::Born => "Space to jump",
        }
    }
}

pub struct Game {
    geng: Rc<Geng>,
    camera: Camera,
    particles: Particles,
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
            particles: Particles::new(),
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
        self.player.stage = self.stage;
        let delta_time = delta_time as f32;
        self.camera.target_fov = if self.stage == Stage::Start {
            5.0
        } else {
            max(self.map.size().x, self.map.size().y) as f32
                + if self.stage == Stage::Moving {
                    5.0
                } else {
                    2.0
                }
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
        let mut fix_pos = self.player.pos;
        if fix_pos.x < self.player.radius {
            fix_pos.x = self.player.radius;
        }
        if fix_pos.y < self.player.radius {
            fix_pos.y = self.player.radius;
        }
        if fix_pos.x > self.map.size().x as f32 - self.player.radius {
            fix_pos.x = self.map.size().x as f32 - self.player.radius;
        }
        if fix_pos.y > self.map.size().y as f32 - self.player.radius {
            fix_pos.y = self.map.size().y as f32 - self.player.radius;
        }
        if fix_pos != self.player.pos {
            self.player.pos = fix_pos;
            if self.player.vel.len() > self.player.max_speed / 2.0 && self.stage == Stage::Moving {
                for _ in 0..10 {
                    self.particles.spawn(self.player.pos, self.player.radius);
                }
                let mut shell_pos = Vec::new();
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let x = self.player.pos.x as i32 + dx;
                        let y = self.player.pos.y as i32 + dy;
                        if x >= 0
                            && x < self.map.size().x as _
                            && y >= 0
                            && y < self.map.size().y as _
                        {
                            shell_pos.push(vec2(x as usize, y as usize));
                        }
                    }
                }
                use rand::seq::SliceRandom;
                shell_pos.shuffle(&mut global_rng());
                for pos in shell_pos {
                    if self.map.tiles[pos.x][pos.y] == Tile::Nothing {
                        self.map.tiles[pos.x][pos.y] = Tile::BrokenShell;
                        for _ in 0..5 {
                            self.particles
                                .spawn(pos.map(|x| x as f32 + 0.5), self.player.radius);
                        }
                        break;
                    }
                }
                if self
                    .map
                    .tiles
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter(|tile| **tile == Tile::BrokenShell)
                            .count()
                    })
                    .sum::<usize>()
                    == 3
                {
                    self.player.radius = 0.3;
                    self.stage = Stage::Born;
                }
            }
            self.player.vel = vec2(0.0, 0.0);
        }
        self.particles.update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        self.map
            .draw(framebuffer, &self.camera, &self.primitive, self.stage);
        self.player.draw(framebuffer, &self.camera, &self.primitive);
        self.particles
            .draw(framebuffer, &self.camera, &self.primitive);

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
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Space => {
                    self.player.want_jump = true;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    let geng = Rc::new(Geng::new(geng::ContextOptions {
        title: "Egg Farm".to_owned(),
        ..default()
    }));
    let game = Game::new(&geng);
    geng::run(geng, game);
}
