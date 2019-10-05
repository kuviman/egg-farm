use geng::prelude::*;

struct Game {
    geng: Rc<Geng>,
}

impl Game {
    fn new(geng: &Rc<Geng>) -> Self {
        Self { geng: geng.clone() }
    }
}

impl geng::State for Game {
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
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
