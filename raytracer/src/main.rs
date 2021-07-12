extern crate glium;

use glium::{implement_vertex, uniform};

#[macro_use]
mod primitives;
mod constants;

use primitives::calculate_uniforms;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC: &str = r#"
    #version 330

    in vec2 position;
    out vec2 space_color;

    uniform mat4 matrix;

    void main() {
        space_color = position;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;


const FRAGMENT_SHADER_SRC: &str = r#"
    #version 330

    const int MAX_SPHERES = 1;

    struct LightSource
    {
        vec3 position;
        vec3 color;
    };

    struct Sphere
    {
        vec3 position;
        float radius;
    };

    struct Ray
    {
        vec3 start;
        vec3 end;
    };

    in vec4 gl_FragCoord;
    in vec2 space_color;
    out vec4 color;

    uniform int width;
    uniform int height;
    uniform Sphere spheres[1];

    void main() {
        vec3 pixCoord = vec3(gl_FragCoord.x / width * 2 - 1, gl_FragCoord.y / height * 2 - 1, 0);

        vec3 pixColor = vec3(0,0,0);

        for (int i = 0; i < MAX_SPHERES; i++) {
            float pixDistance = distance(pixCoord, spheres[i].position);
            if (pixDistance < spheres[i].radius) {
                float greyval = 1 - pixDistance / spheres[i].radius;
                pixColor = vec3(greyval, greyval, greyval);
            }
        }

        color = vec4(pixColor, 1.0);
    }
"#;



fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Filling the screen with 2 triangles, so the fragment shader will draw everywhere.
    let shape = vec![
        Vertex { position: [-1.0, -1.0]},
        Vertex { position: [-1.0, 1.0]},
        Vertex { position: [1.0, -1.0]},
        Vertex { position: [1.0, -1.0]},
        Vertex { position: [-1.0, 1.0]},
        Vertex { position: [1.0, 1.0]},
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    let mut t: f32 = 0.0;
    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // t += 0.002;
        // if t > 2.0 * std::f32::consts::PI {
        //     t = 0.0
        // }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &calculate_uniforms(&[
                primitives::Sphere {
                    position: [0.0, 0.0, 0.0f32],
                    radius: 0.4f32,
                }
            ]),
            // &calc_uniforms!(
            //     pair "width", 600,
            //     pair "height", 500,
            //     pair "matrix", [
            //         [ t.cos(), t.sin(), 0.0, 0.0],
            //         [-t.sin(), t.cos(), 0.0, 0.0],
            //         [0.0, 0.0, 1.0, 0.0],
            //         [0.0, 0.0, 0.0, 1.0f32],
            //     ],
            //     pair "mysphere.position", [0.0, 0.0, 0.0f32],
            //     pair "mysphere.radius", 0.3f32
            // ),

            // &uniform! {
            //     matrix: [
            //         [ t.cos(), t.sin(), 0.0, 0.0],
            //         [-t.sin(), t.cos(), 0.0, 0.0],
            //         [0.0, 0.0, 1.0, 0.0],
            //         [0.0, 0.0, 0.0, 1.0f32],
            //     ],
            //     width: 600,  // TODO: width and height are wrong
            //     height: 500,
            //     // mysphere: primitives::Sphere {
            //     //     position: [0.0, 0.0, 0.0],
            //     //     radius: 0.3,
            //     // }
            // },
            &Default::default())
            .unwrap();
        target.finish().unwrap();

    });
}
