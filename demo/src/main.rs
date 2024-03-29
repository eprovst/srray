use srray::camera::Camera;
use srray::color::Color;
use srray::fractals::SDFMengerSponge;
use srray::geometries::Plane;
use srray::light::PointLight;
use srray::materials::{None, PhongMaterial};
use srray::objects::Object;
use srray::point::Point;
use srray::scene::Scene;
use srray::sdf::{SDFMarcher, SDFRotate};
use srray::vector::Vector;
use std::fs::File;

fn main() {
    let camera = Camera {
        origin: Point::new(0., 0., 2.),
        azimuth: 0.,
        altitude: 0.,
        width: 1920,
        height: 1080,
        fov: 50.,
        spp: 8,
    };

    let scene = Scene {
        max_bounces: 5,
        objects: vec![
            Object {
                geometry: Box::new(SDFMarcher {
                    sdf: Box::new(SDFRotate {
                        sdf: Box::new(SDFMengerSponge { iterations: 5 }),
                        pitch: 40.,
                        yaw: 35.,
                        roll: 0.,
                        reference: Point::ORIGIN,
                    }),
                    max_iterations: 128,
                    max_distance: 20.,
                    tolerance: 1e-5,
                }),
                material: Box::new(PhongMaterial {
                    ambient_color: Color::new(0.02, 0.02, 0.02),
                    diffuse_color: Color::new(0.5, 0.5, 0.5),
                    specular_color: Color::new(2., 2., 2.),
                    specular_power: 50.,
                }),
            },
            Object {
                geometry: Box::new(Plane {
                    origin: Point::new(0., -2., 0.),
                    normal: Vector::new(0., 1., 1.),
                }),
                material: Box::new(PhongMaterial {
                    ambient_color: Color::BLACK,
                    diffuse_color: Color::new(0.1, 0.1, 0.1),
                    specular_color: Color::new(1., 1., 1.),
                    specular_power: 1.,
                }),
            },
        ],
        lights: vec![
            Box::new(PointLight {
                center: Point::new(-2., 1., 2.),
                color: Color::new(4., 2., 2.),
            }),
            Box::new(PointLight {
                center: Point::new(2., 1., 2.),
                color: Color::new(2., 2., 5.),
            }),
        ],
        background: Box::new(None {}),
    };

    let mut file = File::create("test.ppm").unwrap();
    camera.render(&scene).write_binary_ppm(&mut file).unwrap();
}
