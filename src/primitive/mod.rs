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
    quad_geometry: ugli::VertexBuffer<Vertex>,
    instances: RefCell<ugli::VertexBuffer<Instance>>,
    program: ugli::Program,
    circle_program: ugli::Program,
}

impl Primitive {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self {
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
    pub fn quads(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        instances: &ugli::VertexBuffer<Instance>,
    ) {
        self.draw(framebuffer, camera, &self.quad_geometry, instances);
    }
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        vertices: &ugli::VertexBuffer<Vertex>,
        instances: &ugli::VertexBuffer<Instance>,
    ) {
        self.draw_with(&self.program, framebuffer, camera, vertices, instances);
    }
    pub fn draw_with(
        &self,
        program: &ugli::Program,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        vertices: &ugli::VertexBuffer<Vertex>,
        instances: &ugli::VertexBuffer<Instance>,
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
