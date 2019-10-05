use geng::prelude::*;

mod camera;
mod map;
mod player;
mod primitive;

use camera::*;
use map::*;
use player::*;
use primitive::*;

struct Game {
    geng: Rc<Geng>,
    camera: Camera,
    map: Map,
    player: Player,
    primitive: Primitive,
}

impl Game {
    fn new(geng: &Rc<Geng>) -> Self {
        let map = Map::new();
        let mut camera = Camera::new(max(map.size().x, map.size().y) as f32 + 5.0);
        camera.center = map.size().map(|x| x as f32) / 2.0;
        let player = Player::new(camera.center);
        Self {
            geng: geng.clone(),
            camera,
            map,
            player,
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
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        const BORDER_WIDTH: f32 = 0.1;
        self.primitive.quad(
            framebuffer,
            &self.camera,
            AABB::pos_size(
                vec2(-BORDER_WIDTH, -BORDER_WIDTH),
                vec2(BORDER_WIDTH, self.map.size().y as f32 + 2.0 * BORDER_WIDTH),
            ),
            Color::BLACK,
        );
        self.primitive.quad(
            framebuffer,
            &self.camera,
            AABB::pos_size(
                vec2(-BORDER_WIDTH, -BORDER_WIDTH),
                vec2(self.map.size().x as f32 + 2.0 * BORDER_WIDTH, BORDER_WIDTH),
            ),
            Color::BLACK,
        );
        self.primitive.quad(
            framebuffer,
            &self.camera,
            AABB::pos_size(
                vec2(self.map.size().x as f32, -BORDER_WIDTH),
                vec2(BORDER_WIDTH, self.map.size().y as f32 + 2.0 * BORDER_WIDTH),
            ),
            Color::BLACK,
        );
        self.primitive.quad(
            framebuffer,
            &self.camera,
            AABB::pos_size(
                vec2(-BORDER_WIDTH, self.map.size().y as f32),
                vec2(self.map.size().x as f32 + 2.0 * BORDER_WIDTH, BORDER_WIDTH),
            ),
            Color::BLACK,
        );
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
