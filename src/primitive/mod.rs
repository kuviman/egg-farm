use super::*;

#[derive(ugli::Vertex)]
pub struct Vertex {
    pub a_pos: Vec2<f32>,
}

#[derive(ugli::Vertex)]
pub struct Instance {
    pub i_pos: Vec2<f32>,
    pub i_size: Vec2<f32>,
    pub i_color: Color<f32>,
}

pub struct Primitive {
    font: geng::Font,
    quad_geometry: ugli::VertexBuffer<Vertex>,
    dyn_geometry: RefCell<ugli::VertexBuffer<Vertex>>,
    instances: RefCell<ugli::VertexBuffer<Instance>>,
    program: ugli::Program,
    circle_program: ugli::Program,
}

impl Primitive {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self {
            font: geng::Font::new(
                geng,
                include_bytes!("../../static/Simply Rounded Bold.ttf").to_vec(),
            )
            .unwrap(),
            quad_geometry: ugli::VertexBuffer::new_static(
                geng.ugli(),
                vec![
                    Vertex {
                        a_pos: vec2(0.0, 0.0),
                    },
                    Vertex {
                        a_pos: vec2(1.0, 0.0),
                    },
                    Vertex {
                        a_pos: vec2(1.0, 1.0),
                    },
                    Vertex {
                        a_pos: vec2(0.0, 1.0),
                    },
                ],
            ),
            dyn_geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(geng.ugli(), vec![])),
            instances: RefCell::new(ugli::VertexBuffer::new_dynamic(geng.ugli(), vec![])),
            program: geng
                .shader_lib()
                .compile(include_str!("program.glsl"))
                .unwrap(),
            circle_program: geng
                .shader_lib()
                .compile(concat!("#define CIRCLE\n", include_str!("program.glsl")))
                .unwrap(),
        }
    }
    pub fn text(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        text: &str,
        pos: Vec2<f32>,
        size: f32,
        color: Color<f32>,
    ) {
        let p1 = camera.world_to_screen(framebuffer, pos);
        let p2 = camera.world_to_screen(framebuffer, pos + vec2(0.0, size));
        self.font.draw(framebuffer, text, p1, p2.y - p1.y, color);
    }
    pub fn text_bubble(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        text: &str,
        pos: Vec2<f32>,
        size: f32,
    ) {
        let text_width = self.font.measure(text, 64.0).width() * size / 64.0;
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
        let mut instances = self.instances.borrow_mut();
        instances.clear();
        instances.push(Instance {
            i_pos: rect.bottom_left(),
            i_size: rect.size(),
            i_color: color,
        });
        self.quads(framebuffer, camera, &instances);
    }
    pub fn circle(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        pos: Vec2<f32>,
        radius: f32,
        color: Color<f32>,
    ) {
        let mut instances = self.instances.borrow_mut();
        instances.clear();
        instances.push(Instance {
            i_pos: pos - vec2(radius, radius),
            i_size: vec2(radius, radius) * 2.0,
            i_color: color,
        });
        self.draw_with(
            &self.circle_program,
            framebuffer,
            camera,
            &self.quad_geometry,
            &instances,
        );
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
        let mut geom = self.dyn_geometry.borrow_mut();
        geom.clear();
        geom.push(Vertex { a_pos: p1 + n });
        geom.push(Vertex { a_pos: p2 + n });
        geom.push(Vertex { a_pos: p2 - n });
        geom.push(Vertex { a_pos: p1 - n });
        let mut instances = self.instances.borrow_mut();
        instances.clear();
        instances.push(Instance {
            i_pos: vec2(0.0, 0.0),
            i_size: vec2(1.0, 1.0),
            i_color: color,
        });
        self.draw(framebuffer, camera, &*geom, &*instances)
    }
    pub fn half_circle(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        pos: Vec2<f32>,
        radius: f32,
        color: Color<f32>,
    ) {
        let mut instances = self.instances.borrow_mut();
        instances.clear();
        instances.push(Instance {
            i_pos: pos - vec2(radius, radius),
            i_size: vec2(radius, radius) * 2.0,
            i_color: color,
        });
        self.draw_with(
            &self.circle_program,
            framebuffer,
            camera,
            self.quad_geometry.slice(0..3),
            &instances,
        );
    }
    pub fn quads(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        instances: &ugli::VertexBuffer<Instance>,
    ) {
        self.draw(framebuffer, camera, &self.quad_geometry, instances);
    }
    pub fn draw<'a, V: ugli::IntoVertexBufferSlice<'a, Vertex>>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        vertices: V,
        instances: &'a ugli::VertexBuffer<Instance>,
    ) {
        self.draw_with(&self.program, framebuffer, camera, vertices, instances);
    }
    pub fn draw_with<'a, V: ugli::IntoVertexBufferSlice<'a, Vertex>>(
        &self,
        program: &ugli::Program,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        vertices: V,
        instances: &'a ugli::VertexBuffer<Instance>,
    ) {
        let camera_uniforms = camera.uniforms(framebuffer);
        ugli::draw(
            framebuffer,
            program,
            ugli::DrawMode::TriangleFan,
            ugli::instanced(vertices, instances),
            camera_uniforms,
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
    }
}
