extern crate image;
extern crate serde;

use image::ImageBuffer;
use std::path::PathBuf;
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::scene::{Coloration, Texture, Material, Element, Plane, Scene, Sphere};
use pbrt::vector3::Vector3;
use pbrt::light::{Light, SphericalLight, DirectionalLight};
use pbrt::color::Color;

mod pbrt;

fn main() {

  let green_mat = Material {
    albedo: 0.18,
    color: Coloration::Color(Color {
      r: 0.4,
      g: 1.0,
      b: 0.4,
    })
  };

  let red_mat = Material {
    albedo: 0.58,
    color: Coloration::Color(Color {
      r: 1.0,
      g: 0.0,
      b: 0.4,
    })
  };

  let blue_mat = Material {
    albedo: 0.18,
    color: Coloration::Texture(Texture::load_texture(PathBuf::from("./checkerboard.png")).unwrap())
  };

  let plane_mat = Material {
    albedo: 0.18,
    color: Coloration::Texture(Texture::load_texture(PathBuf::from("./checkerboard.png")).unwrap())
  };

  let scene = Scene {
    shadow_bias: 1e-13,
    width: 1024,
    height: 1024,
    fov: 90.0,
    background: Color::black(),
    lights: vec![
      Light::Spherical(SphericalLight { 
        position: Point {x: -2.0, y: 10.0, z: -3.0},
        color: Color { r: 0.3, g: 0.8, b: 0.3},
        intensity: 10000.0
      }),
      Light::Spherical(SphericalLight { 
        position: Point {x: 0.25, y: 0.0, z: -2.0},
        color: Color { r: 0.8, g: 0.3, b: 0.3},
        intensity: 250.0
      }),
      Light::Directional(DirectionalLight { 
        direction: Vector3 {x: 0.0, y: 0.0, z: -1.0},
        color: Color { r: 1.0, g: 1.0, b: 1.0},
        intensity: 0.0
      }),
    ],
    entities: vec![
      Element::Plane(Plane {
        origin: Point::new(0.0, -2.0, -5.0),
        normal: Vector3::down(),
        material: plane_mat,
      }),
      Element::Sphere(Sphere{ center: Point::new(0.0, 0.0, -5.0), radius: 1.0, material: green_mat}),
      Element::Sphere(Sphere{ center: Point::new(-3.0, 1.0, -6.0), radius: 2.0, material: red_mat }),
      Element::Sphere(Sphere{ center: Point::new(5.0, 0.0, -10.0), radius: 3.0, material: blue_mat}),
    ],
  };

  render(&scene).save("test.png").unwrap();
}

fn render(scene: &Scene) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
  ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
    let ray = Ray::create_prime(x, y, scene);

    if let Some(intersection) = scene.trace(&ray) {
      let hit_point = ray.origin + (ray.direction * intersection.distance);
      let surface_normal = intersection.element.surface_normal(&hit_point);
      let texture_coords = intersection.element.texture_coords(&hit_point);

      let mut color = Color {r: 0.0, g: 0.0, b: 0.0};

      for light in &scene.lights {

        let direction_to_light = light.direction_from(&hit_point);

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light,
        };

        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none() ||
                       shadow_intersection.unwrap().distance > light.distance(&hit_point);

        let light_reflected = intersection.element.material().albedo / std::f32::consts::PI;

        let light_intensity = if in_light { light.intensity(&hit_point) } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;

        color = color + intersection.element.material().color.color(&texture_coords) * light.color() * light_power * light_reflected;
      }
      color.clamp().to_rgba()
    } else {
      scene.background.to_rgba()
    }
  })
}
