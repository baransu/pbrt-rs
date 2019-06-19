extern crate image;
extern crate serde;

use image::ImageBuffer;
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::scene::{Color, Element, Plane, Scene, Sphere};
use pbrt::vector3::Vector3;
use pbrt::light::Light;

mod pbrt;

fn main() {
  let green = Color {
    r: 0.4,
    g: 1.0,
    b: 0.4,
  };

  let red = Color {
    r: 1.0,
    g: 0.0,
    b: 0.4,
  };

  let blue = Color {
    r: 0.4,
    g: 0.4,
    b: 1.0,
  };

  let _white = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
  };

  let scene = Scene {
    shadow_bias: 1e-13,
    width: 800,
    height: 800,
    fov: 90.0,
    background: Color {
      r: 0.41,
      g: 0.85,
      b: 1.0,
    },
    lights: vec![
      Light { 
        color: Color { r: 0.3, g: 0.8, b: 0.3},
        direction: Vector3 {x: 0.25, y: -1.0, z: -1.0},
        intensity: 10.0
      },
      Light { 
        color: Color { r: 0.8, g: 0.3, b: 0.3},
        direction: Vector3 {x: 0.25, y: -0.5, z: -0.5},
        intensity: 10.0
      }
    ],
    entities: vec![
      Element::Plane(Plane {
        origin: Point::new(0.0, -2.0, 0.0),
        normal: Vector3::down(),
        color: Color {
          r: 0.4,
          g: 0.4,
          b: 0.4,
        },
        albedo: 0.2,
      }),
      Element::Sphere(Sphere::new(
        Point::new(-3.0, 4.0, -10.0), 2.0, red, 0.2),
      ),
      Element::Sphere(Sphere::new(Point::new(0.0, -2.0, -10.0), 1.0, green, 0.2)),
      Element::Sphere(Sphere::new(Point::new(5.0, 0.0, -10.0), 3.0, blue, 0.2)),
    ],
  };

  render(&scene).save("test.png").unwrap();
}

fn render(scene: &Scene) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
  ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
    let ray = Ray::create_prime(x, y, scene);

    if let Some(intersection) = scene.trace(&ray) {
      let hit_point = ray.origin + (ray.direction * intersection.distance);
      let surface_normal = intersection.object.surface_normal(&hit_point);

      let mut color = Color {r: 0.0, g: 0.0, b: 0.0};

      for light in &scene.lights {

        let direction_to_light = -light.direction.normalize();

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light,
        };

        let in_light = scene.trace(&shadow_ray).is_none();

        let light_reflected = intersection.object.albedo() / std::f32::consts::PI;

        let light_intensity = if in_light { light.intensity } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;

        color = color + intersection.object.color() * light.color * light_power * light_reflected;
      }
      image::Rgba(color.clamp().to_rgba())
    } else {
      image::Rgba(scene.background.to_rgba())
    }
  })
}
