use super::*;

#[derive(ugli::Vertex, Clone)]
pub struct Vertex {
    pub a_pos: Vec2<f32>,
    pub a_circle: Vec2<f32>,
    pub a_color: Color<f32>,
}

pub struct Primitive {
    font: geng::Font,
    geometry: RefCell<ugli::VertexBuffer<Vertex>>,
    program: ugli::Program,
    texts: RefCell<Vec<(String, Vec2<f32>, f32, Color<f32>)>>,
}

impl Primitive {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self {
            font: geng::Font::new(
                geng,
                include_bytes!("../../static/Simply Rounded Bold.ttf").to_vec(),
            )
            .unwrap(),
            geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(geng.ugli(), vec![])),
            program: geng
                .shader_lib()
                .compile(include_str!("program.glsl"))
                .unwrap(),
            texts: RefCell::new(Vec::new()),
        }
    }
    pub fn flush(&self, framebuffer: &mut ugli::Framebuffer, camera: &Camera) {
        let camera_uniforms = camera.uniforms(framebuffer);
        let mut geom = self.geometry.borrow_mut();
        ugli::draw(
            framebuffer,
            &self.program,
            ugli::DrawMode::Triangles,
            &*geom,
            camera_uniforms,
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
        geom.clear();
        for (text, pos, size, color) in self.texts.borrow_mut().drain(..) {
            self.font.draw(framebuffer, &text, pos, size, color);
        }
    }
    pub fn text(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        text: String,
        pos: Vec2<f32>,
        size: f32,
        color: Color<f32>,
    ) {
        let p1 = camera.world_to_screen(framebuffer, pos);
        let p2 = camera.world_to_screen(framebuffer, pos + vec2(0.0, size));
        self.texts.borrow_mut().push((text, p1, p2.y - p1.y, color));
    }
    pub fn text_bubble(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        text: String,
        pos: Vec2<f32>,
        size: f32,
    ) {
        let text_width = self.font.measure(&text, 64.0).width() * size / 64.0;
        let x_align = (clamp((pos.x - camera.center.x) / camera.fov * 2.0, -1.0..=1.0) + 1.0) / 2.0;
        let pos = vec2(pos.x - text_width * x_align, pos.y + size);
        let cnt = (text_width / size).ceil() as usize;
        let circles = (0..cnt).map(|i| {
            let x = (i as f32 + 0.5) * text_width / cnt as f32;
            (vec2(pos.x + x, pos.y + size / 2.0), size * 0.9)
        });
        for (pos, radius) in circles.clone() {
            self.circle(framebuffer, camera, pos, radius, Color::BLACK);
        }
        for (pos, radius) in circles.clone() {
            self.circle(framebuffer, camera, pos, radius * 0.9, Color::WHITE);
        }
        self.text(framebuffer, camera, text, pos, size, Color::BLACK);
    }
    pub fn quad(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        rect: AABB<f32>,
        color: Color<f32>,
    ) {
        self.polygon(&[
            Vertex {
                a_pos: rect.bottom_left(),
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
            Vertex {
                a_pos: rect.bottom_right(),
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
            Vertex {
                a_pos: rect.top_right(),
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
            Vertex {
                a_pos: rect.top_left(),
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
        ]);
    }
    pub fn circle(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        pos: Vec2<f32>,
        radius: f32,
        color: Color<f32>,
    ) {
        self.polygon(&[
            Vertex {
                a_pos: pos + vec2(-radius, -radius),
                a_circle: vec2(-1.0, -1.0),
                a_color: color,
            },
            Vertex {
                a_pos: pos + vec2(radius, -radius),
                a_circle: vec2(1.0, -1.0),
                a_color: color,
            },
            Vertex {
                a_pos: pos + vec2(radius, radius),
                a_circle: vec2(1.0, 1.0),
                a_color: color,
            },
            Vertex {
                a_pos: pos + vec2(-radius, radius),
                a_circle: vec2(-1.0, 1.0),
                a_color: color,
            },
        ]);
    }
    pub fn polygon(&self, vs: &[Vertex]) {
        let mut geom = self.geometry.borrow_mut();
        for i in 2..vs.len() {
            geom.push(vs[0].clone());
            geom.push(vs[i - 1].clone());
            geom.push(vs[i].clone());
        }
    }
    pub fn line(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        p1: Vec2<f32>,
        p2: Vec2<f32>,
        width: f32,
        color: Color<f32>,
    ) {
        let v = (p2 - p1).normalize();
        let n = vec2(-v.y, v.x);
        let w = width / 2.0;
        let n = n * w;
        self.polygon(&[
            Vertex {
                a_pos: p1 + n,
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
            Vertex {
                a_pos: p2 + n,
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
            Vertex {
                a_pos: p2 - n,
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
            Vertex {
                a_pos: p1 - n,
                a_circle: vec2(0.0, 0.0),
                a_color: color,
            },
        ]);
    }
    pub fn half_circle(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        pos: Vec2<f32>,
        radius: f32,
        color: Color<f32>,
    ) {
        let mut geom = self.geometry.borrow_mut();
        geom.push(Vertex {
            a_pos: pos + vec2(-radius, -radius),
            a_circle: vec2(-1.0, -1.0),
            a_color: color,
        });
        geom.push(Vertex {
            a_pos: pos + vec2(radius, -radius),
            a_circle: vec2(1.0, -1.0),
            a_color: color,
        });
        geom.push(Vertex {
            a_pos: pos + vec2(radius, radius),
            a_circle: vec2(1.0, 1.0),
            a_color: color,
        });
    }
}
