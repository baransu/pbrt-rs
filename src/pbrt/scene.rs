extern crate image;
extern crate serde;

use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::vector3::Vector3;

pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Color {
  pub fn black() -> Color {
    Color {
      r: 0.0,
      g: 0.0,
      b: 0.0,
    }
  }

  pub fn to_rgba(&self) -> [u8; 4] {
    // TODO: gamma encode
    [
      (self.r * 255.0) as u8,
      (self.g * 255.0) as u8,
      (self.b * 255.0) as u8,
      255,
    ]
  }
}

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub color: Color,
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: &Ray) -> bool {
    let l: Vector3 = self.center - ray.origin;

    let adj2 = l.dot(&ray.direction);

    let d2 = l.dot(&l) - (adj2 * adj2);

    d2 < (self.radius * self.radius)
  }
}

pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub sphere: Sphere,
}
