use geng::prelude::*;

mod camera;
mod map;
mod primitive;

use camera::*;
use map::*;
use primitive::*;

struct Game {
    geng: Rc<Geng>,
    camera: Camera,
    map: Map,
    primitive: Primitive,
}

impl Game {
    fn new(geng: &Rc<Geng>) -> Self {
        let map = Map::new();
        let mut camera = Camera::new(max(map.size().x, map.size().y) as f32 + 5.0);
        camera.center = map.size().map(|x| x as f32) / 2.0;
        Self {
            geng: geng.clone(),
            camera,
            map,
            primitive: Primitive::new(geng),
        }
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
