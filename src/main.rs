#![windows_subsystem = "windows"]

use geng::prelude::*;

mod camera;
mod map;
mod mutation;
mod particles;
mod player;
mod primitive;
mod projectile;

use camera::*;
use map::*;
use mutation::*;
use particles::*;
use player::*;
use primitive::*;
use projectile::*;

#[derive(geng::Assets)]
pub struct Assets {
    #[asset(path = "spit.wav")]
    spit: geng::Sound,
    #[asset(path = "eat.wav")]
    eat: geng::Sound,
    #[asset(path = "smoke.wav")]
    smoke: geng::Sound,
    #[asset(path = "crack.wav")]
    crack: geng::Sound,
    #[asset(path = "birth.wav")]
    birth: geng::Sound,
    #[asset(path = "poop.wav")]
    poop: geng::Sound,
    #[asset(path = "hit.wav")]
    hit: geng::Sound,
    #[asset(path = "death.wav")]
    death: geng::Sound,
    #[asset(path = "jump.wav")]
    jump: geng::Sound,
    #[asset(path = "projectile_hit.wav")]
    projectile_hit: geng::Sound,
    #[asset(path = "weed.wav")]
    weed: geng::Sound,
    #[asset(path = "weed_death.wav")]
    weed_death: geng::Sound,
    #[asset(path = "trophey.wav")]
    trophey: geng::Sound,
    #[asset(path = "win.wav")]
    win: geng::Sound,
    #[asset(path = "mutate.wav")]
    mutate: geng::Sound,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Stage {
    Start,
    Moving,
    Born,
    ToCrush,
    WaitForFood,
    Poop,
    PoopFertilize,
    GrowWeed,
    KillWeed,
    Mutate,
    GrowMutation,
    KillMutated,
    KillAll,
    Win,
}

impl Stage {
    fn help(&self) -> &str {
        match self {
            Self::Start => "Use WASD to move around",
            Self::Moving => "Try to break the wall",
            Self::Born => "Use Space to jump",
            Self::ToCrush => "Crush the shell to fertilize soil",
            Self::WaitForFood => "Fertilized soil will grow something eventually",
            Self::Poop => "Pooping is unavoidable if you jump on empty space after eating",
            Self::PoopFertilize => "Poop can also be used as fertilizer",
            Self::GrowWeed => "More food! More poop! More!",
            Self::KillWeed => "Getting rid of angry plants may require planting more",
            Self::Mutate => "This mutated root must be destroyed!",
            Self::GrowMutation => "Maybe mutation should be spread, make life more colorful",
            Self::KillMutated => "Well, you've done this before",
            Self::KillAll => "Collect all tropheys. Remember there are several levels of mutation!",
            Self::Win => "You WON! Congrats! Make screenshot, or nobody will believe you!",
        }
    }
}

pub struct Game {
    assets: Rc<Assets>,
    geng: Rc<Geng>,
    camera: Camera,
    particles: Particles,
    map: Map,
    player: Player,
    stage: Stage,
    primitive: Primitive,
    projectiles: Vec<Projectile>,
    restart: bool,
}

impl Game {
    fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        let map = Map::new();
        let mut camera = Camera::new(0.1);
        camera.center = map.size().map(|x| x as f32) / 2.0;
        let player = Player::new(camera.center);
        Self {
            assets: assets.clone(),
            geng: geng.clone(),
            camera,
            particles: Particles::new(),
            map,
            player,
            stage: Stage::Start,
            primitive: Primitive::new(geng),
            projectiles: Vec::new(),
            restart: false,
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
        self.camera.target_fov = if self.stage == Stage::Start || self.stage == Stage::Win {
            5.0
        } else {
            max(self.map.size().x, self.map.size().y) as f32
                + if self.stage == Stage::Moving {
                    5.0
                } else {
                    2.0
                }
        };
        if self.stage == Stage::Win {
            self.camera.center = self.player.pos;
        }
        self.camera.update(delta_time);
        if self.stage == Stage::Win {
            return;
        }
        self.map.update(
            delta_time,
            &mut self.particles,
            &mut self.projectiles,
            &mut self.player,
            &self.assets,
        );
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
        self.player.update(delta_time, &self.assets);
        if self.stage == Stage::Born && self.player.jump.is_some() {
            self.stage = Stage::ToCrush;
        }
        if self.stage == Stage::ToCrush
            && self.map.find(|tile| {
                if let Tile::FertilizedSoil { .. } = tile {
                    true
                } else {
                    false
                }
            }) > 0
        {
            self.stage = Stage::WaitForFood;
        }
        if self.stage == Stage::WaitForFood && self.map.find(|tile| tile.is_food()) > 0 {
            self.stage = Stage::Poop;
        }
        if self.stage == Stage::Poop && self.map.find(|tile| tile.is_poop()) > 0 {
            self.stage = Stage::PoopFertilize;
        }
        if self.stage == Stage::PoopFertilize && self.map.find(|tile| tile.is_poop()) == 0 {
            self.stage = Stage::GrowWeed;
        }
        if self.stage == Stage::GrowWeed
            && self.map.find(|tile| match tile {
                Tile::AngryWeed { .. } => true,
                _ => false,
            }) > 0
        {
            self.stage = Stage::KillWeed;
        }
        if self.stage == Stage::KillWeed && self.map.find(|tile| *tile == Tile::MutatedRoot) > 0 {
            self.stage = Stage::Mutate;
        }
        if self.stage == Stage::Mutate && self.player.mutation.is_some() {
            self.stage = Stage::GrowMutation;
        }
        if self.stage == Stage::GrowMutation
            && self.map.find(|tile| match tile {
                Tile::AngryWeed { mutation, .. } if mutation.is_some() => true,
                _ => false,
            }) > 0
        {
            self.stage = Stage::KillMutated;
        }
        if self.stage == Stage::KillMutated && self.map.find(|tile| tile.is_trophey()) > 0 {
            self.stage = Stage::KillAll;
        }
        if self.player.tropheys.len() == 7 {
            self.assets.win.play();
            self.stage = Stage::Win;
        }
        if !self.player.eaten {
            if let Tile::Food { mutation } =
                self.map.tiles[self.player.pos.x as usize][self.player.pos.y as usize]
            {
                self.assets.eat.play();
                self.player.eaten = true;
                self.player.almost_dead = false;
                if let Some(mutation) = mutation {
                    self.player.mutation = mutation.mix(self.player.mutation);
                }
                self.map.tiles[self.player.pos.x as usize][self.player.pos.y as usize] =
                    Tile::FertilizedSoil {
                        time: FERTILIZED_SOIL_TIME,
                        mutation,
                    };
            }
        }
        if self.player.landed() {
            if self.player.eaten
                && self.map.tiles[self.player.pos.x as usize][self.player.pos.y as usize]
                    == Tile::Nothing
            {
                self.assets.poop.play();
                self.player.eaten = false;
                self.assets.smoke.play();
                self.particles.boom(self.player.pos, self.player.mutation);
                self.map.tiles[self.player.pos.x as usize][self.player.pos.y as usize] =
                    Tile::Poop {
                        mutation: self.player.mutation,
                    };
                self.player.mutation = None;
            } else {
                self.map.land(
                    self.player.pos,
                    &mut self.particles,
                    &mut self.player,
                    &self.assets,
                );
            }
        }
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
                self.assets.crack.play();
                self.assets.smoke.play();
                self.particles.boom(self.player.pos, None);
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
                        self.particles.boom(pos.map(|x| x as f32 + 0.5), None);
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
                    self.assets.birth.play();
                    self.stage = Stage::Born;
                }
            }
            self.player.vel = vec2(0.0, 0.0);
        }
        self.particles.update(delta_time);
        for i in 0..self.projectiles.len() {
            for j in 0..i {
                if (self.projectiles[i].pos - self.projectiles[j].pos).len()
                    < self.projectiles[i].radius + self.projectiles[j].radius
                    && self.projectiles[i].mutation == self.projectiles[j].mutation
                {
                    self.projectiles[i].alive = false;
                    self.projectiles[j].alive = false;
                }
            }
        }
        for p in &mut self.projectiles {
            if self.player.alive && (p.pos - self.player.pos).len() < p.radius + self.player.radius
            {
                p.alive = false;
                if self.player.almost_dead {
                    self.assets.death.play();
                    self.particles.boom(self.player.pos, self.player.mutation);
                    self.player.alive = false;
                } else {
                    self.assets.hit.play();
                    self.player.almost_dead = true;
                    self.particles.boom(p.pos, p.mutation);
                    self.player.vel += p.vel * 2.0;
                }
            }
            self.map.collide_projectile(p, &self.assets);
            p.update(delta_time);
            if p.pos.x < 0.0
                || p.pos.y < 0.0
                || p.pos.x >= self.map.size().x as f32
                || p.pos.y >= self.map.size().y as f32
            {
                p.alive = false;
            }
            if !p.alive {
                self.assets.projectile_hit.play();
                self.assets.smoke.play();
                self.particles.boom(p.pos, p.mutation);
            }
        }
        self.projectiles.retain(|p| p.alive);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        self.map
            .draw(framebuffer, &self.camera, &self.primitive, self.stage);
        self.player.draw(
            framebuffer,
            &self.camera,
            &self.primitive,
            if self.stage < Stage::Born {
                Some(
                    self.map
                        .tiles
                        .iter()
                        .map(|row| {
                            row.iter()
                                .filter(|tile| **tile == Tile::BrokenShell)
                                .count()
                        })
                        .sum::<usize>(),
                )
            } else {
                None
            },
        );
        for p in &self.projectiles {
            p.draw(framebuffer, &self.camera, &self.primitive);
        }
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
            "?".to_owned(),
            help_pos,
            self.camera.fov / 20.0,
        );
        let text = if (mouse_pos - help_pos - vec2(0.0, self.camera.fov * 3.0 / 40.0)).len()
            < self.camera.fov / 20.0
        {
            let mut text = self.stage.help().to_owned();
            if !self.player.alive {
                text += " (press R to restart)";
            }
            text
        } else {
            self.text_at(mouse_pos)
        };
        self.primitive.text_bubble(
            framebuffer,
            &self.camera,
            text,
            mouse_pos,
            self.camera.fov / 30.0,
        );
        self.primitive.flush(framebuffer, &self.camera);
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Space => {
                    self.player.want_jump = true;
                }
                geng::Key::R => {
                    self.restart = true;
                }
                _ => {}
            },
            _ => {}
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        if self.restart {
            Some(geng::Transition::Switch(Box::new(Game::new(
                &self.geng,
                &self.assets,
            ))))
        } else {
            None
        }
    }
}

fn main() {
    if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    }
    let geng = Rc::new(Geng::new(geng::ContextOptions {
        title: "Egg Farm".to_owned(),
        ..default()
    }));
    let game = geng::LoadingScreen::new(
        &geng,
        geng::EmptyLoadingScreen,
        geng::LoadAsset::load(&geng, "."),
        {
            let geng = geng.clone();
            move |assets| {
                let assets = Rc::new(assets.unwrap());
                Game::new(&geng, &assets)
            }
        },
    );
    geng::run(geng, game);
}
