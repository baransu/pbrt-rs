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
    // TODO: gamma correction
    [
      (self.r * 255.0) as u8,
      (self.g * 255.0) as u8,
      (self.b * 255.0) as u8,
      255,
    ]
  }
}

pub struct Plane {
  pub origin: Point,
  pub normal: Vector3,
  pub color: Color,
}

pub enum Element {
  Sphere(Sphere),
  Plane(Plane),
}

impl Element {
  pub fn color(&self) -> &Color {
    match *self {
      Element::Sphere(ref s) => &s.color,
      Element::Plane(ref p) => &p.color,
    }
  }
}

impl Intersectable for Element {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    match *self {
      Element::Sphere(ref s) => s.intersect(&ray),
      Element::Plane(ref p) => p.intersect(&ray),
    }
  }
}

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub color: Color,
}

impl Sphere {
  pub fn new(center: Point, radius: f64, color: Color) -> Sphere {
    Sphere {
      center,
      radius,
      color,
    }
  }
}

pub struct Intersecion<'a> {
  pub distance: f64,
  pub object: &'a Element,
}

impl<'a> Intersecion<'a> {
  pub fn new<'b>(distance: f64, object: &'b Element) -> Intersecion<'b> {
    Intersecion { distance, object }
  }
}

pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub background: Color,
  pub entities: Vec<Element>,
}

impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersecion> {
    self
      .entities
      .iter()
      .filter_map(|s| s.intersect(ray).map(|d| Intersecion::new(d, s)))
      .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
  }
}
