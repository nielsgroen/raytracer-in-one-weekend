extern crate glium;

use glium::uniforms::{UniformValue, UniformsStorage, EmptyUniforms, AsUniformValue, Uniforms};

use crate::constants;
use crate::constants::MAX_SPHERES;


// pub fn calculate_uniforms<'a>() -> UniformsStorage<'static, i32, UniformsStorage<'static, [[f32; 4]; 4], UniformsStorage<'static, i32, UniformsStorage<'static, i32, EmptyUniforms>>>> {
pub fn calculate_uniforms(spheres: &[Sphere]) -> impl glium::uniforms::Uniforms {

    // VIEWPORT
    let uniforms = UniformsStorage::new("height", constants::START_HEIGHT);
    let uniforms = uniforms.add("width", constants::START_WIDTH);
    let uniforms = uniforms.add("focalLength", 1.0f32);

    let uniforms = uniforms.add("matrix", [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]);

    // TODO:
    // IF I FIGURE OUT HOW TO MAKE THE LIFETIME(BIG) OF THE FORMATTED STRINGS BIG ENOUGH
    // I CAN USE THIS.
    // let mut i =  0;
    // let uniforms = loop {
    //     let pos_fmt = format!("spheres[{}].position", i);
    //     let uniforms = uniforms.add(&pos_fmt[..], spheres[i as usize].position);
    //
    //     let radius_fmt = format!("spheres[{}].radius", i);
    //     let uniforms = uniforms.add(&radius_fmt[..], spheres[i as usize].radius);
    //
    //     i += 1;
    //     if i == MAX_SPHERES {
    //         break uniforms;
    //     }
    // };

    // SPHERES
    let uniforms = uniforms.add("spheres[0].position", spheres[0].position);
    let uniforms = uniforms.add("spheres[0].radius", spheres[0].radius);

    // CAMERA
    let uniforms = uniforms.add("camera.position", [0.0, 0.0, 0.0f32]);
    let uniforms = uniforms.add("camera.direction", [0.0, 0.0, -1.0f32]);



    uniforms
}


macro_rules! calc_uniforms {
    () => {
        glium::uniforms::EmptyUniforms
    };

    (pair $name1:expr, $value1:expr, $(pair $name:expr, $value:expr),+) => {
        {
            let uniforms = glium::uniforms::UniformsStorage::new($name1, $value1);

            $(
                let uniforms = uniforms.add($name, $value);
            )+
            uniforms
        }
    };

    // ($($name:ident, $value:expr),*,) => {
    //     calc_uniforms!($($name, $value),*)
    // };
}


#[derive(Copy, Clone, Default)]
pub struct Sphere {
    pub position: [f32; 3],
    pub radius: f32,
}

impl glium::uniforms::Uniforms for Sphere {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("position", UniformValue::Vec3(
            self.position
        ));
        f("radius", UniformValue::Float(
            self.radius
        ))
    }
}


// Placeholder struct to register the `Sphere`s.
pub struct Spheres {
    spheres: [Sphere; MAX_SPHERES as usize],
}

impl glium::uniforms::Uniforms for Spheres {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        for i in 0..MAX_SPHERES {
            f(&format!("spheres[{}].position", i)[..], UniformValue::Vec3(self.spheres[i as usize].position));
            f(&format!("spheres[{}].radius", i)[..], UniformValue::Float(self.spheres[i as usize].radius));
        }
    }
}
