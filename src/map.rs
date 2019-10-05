use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Nothing,
    BrokenShell,
}

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new() -> Self {
        let size = vec2(16, 16);
        Self {
            tiles: vec![vec![Tile::Nothing; size.y]; size.x],
        }
    }
    pub fn size(&self) -> Vec2<usize> {
        vec2(self.tiles.len(), self.tiles[0].len())
    }
    pub fn text_at(&self, pos: Vec2<f32>) -> Option<String> {
        fn close(pos: f32, size: usize) -> bool {
            pos.abs() < 0.5 || (pos - size as f32).abs() < 0.5
        }
        match self
            .tiles
            .get(pos.x.max(0.0) as usize)
            .and_then(|row| row.get(pos.y.max(0.0) as usize))
        {
            None | Some(Tile::Nothing) => {
                if close(pos.x, self.size().x) || close(pos.y, self.size().y) {
                    Some("Wall".to_owned())
                } else {
                    None
                }
            }
            Some(Tile::BrokenShell) => Some("Broken shell".to_owned()),
        }
    }
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        primitive: &Primitive,
        stage: Stage,
    ) {
        if stage > Stage::Start {
            const BORDER_WIDTH: f32 = 0.1;
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(-BORDER_WIDTH, -BORDER_WIDTH),
                    vec2(BORDER_WIDTH, self.size().y as f32 + 2.0 * BORDER_WIDTH),
                ),
                Color::BLACK,
            );
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(-BORDER_WIDTH, -BORDER_WIDTH),
                    vec2(self.size().x as f32 + 2.0 * BORDER_WIDTH, BORDER_WIDTH),
                ),
                Color::BLACK,
            );
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(self.size().x as f32, -BORDER_WIDTH),
                    vec2(BORDER_WIDTH, self.size().y as f32 + 2.0 * BORDER_WIDTH),
                ),
                Color::BLACK,
            );
            primitive.quad(
                framebuffer,
                &camera,
                AABB::pos_size(
                    vec2(-BORDER_WIDTH, self.size().y as f32),
                    vec2(self.size().x as f32 + 2.0 * BORDER_WIDTH, BORDER_WIDTH),
                ),
                Color::BLACK,
            );
        }
        for (x, row) in self.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Nothing => {}
                    Tile::BrokenShell => {
                        primitive.half_circle(
                            framebuffer,
                            camera,
                            vec2(x as f32 + 0.5, y as f32 + 0.5),
                            0.4,
                            Color::BLACK,
                        );
                        primitive.half_circle(
                            framebuffer,
                            camera,
                            vec2(x as f32 + 0.5, y as f32 + 0.5),
                            0.3,
                            Color::WHITE,
                        );
                        primitive.line(
                            framebuffer,
                            camera,
                            vec2(x as f32 + 0.5, y as f32 + 0.5) + vec2(1.0, 1.0).normalize() * 0.4,
                            vec2(x as f32 + 0.5, y as f32 + 0.5) - vec2(1.0, 1.0).normalize() * 0.4,
                            0.1,
                            Color::BLACK,
                        );
                        primitive.line(
                            framebuffer,
                            camera,
                            vec2(x as f32 + 0.5, y as f32 + 0.5),
                            vec2(x as f32 + 0.5, y as f32 + 0.3),
                            0.1,
                            Color::BLACK,
                        );
                        primitive.line(
                            framebuffer,
                            camera,
                            vec2(x as f32 + 0.7, y as f32 + 0.35),
                            vec2(x as f32 + 0.45, y as f32 + 0.35),
                            0.1,
                            Color::BLACK,
                        );
                    }
                }
            }
        }
    }
}
