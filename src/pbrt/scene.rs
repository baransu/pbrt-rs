extern crate image;
extern crate serde;

use std::ops::{Mul, Add};
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::vector3::Vector3;
use pbrt::light::Light;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
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

  pub fn clamp(&self) -> Color {
    Color {
      r: self.r.min(1.0).max(0.0),
      g: self.g.min(1.0).max(0.0),
      b: self.b.min(1.0).max(0.0),
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

impl Mul for Color {
  type Output = Color;

  fn mul(self, other: Color) -> Color {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b,
    }
  }
}

impl Mul<f32> for Color {
  type Output = Color;

  fn mul(self, other: f32) -> Color {
    Color {
      r: self.r * other,
      g: self.g * other,
      b: self.b * other,
    }
  }
}

impl Mul<Color> for f32 {
  type Output = Color;

  fn mul(self, other: Color) -> Color {
    other * self
  }
}

impl Add for  Color{
  type Output = Color;

  fn add(self, other: Color) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b,
    }
  }
}

pub struct Plane {
  pub origin: Point,
  pub normal: Vector3,
  pub color: Color,
  pub albedo: f32,
}

pub enum Element {
  Sphere(Sphere),
  Plane(Plane),
}

impl Element {
  pub fn color(&self) -> Color {
    match *self {
      Element::Sphere(ref s) => s.color.clone(),
      Element::Plane(ref p) => p.color.clone(),
    }
  }

  pub fn albedo(&self) -> f32 {
    match *self {
      Element::Sphere(ref s) => s.albedo,
      Element::Plane(ref p) => p.albedo,
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

  fn surface_normal(&self, hit_point: &Point) -> Vector3 {
    match *self {
      Element::Sphere(ref s) => s.surface_normal(&hit_point),
      Element::Plane(ref p) => p.surface_normal(&hit_point),
    }
  }
}

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub color: Color,
  pub albedo: f32,
}

impl Sphere {
  pub fn new(center: Point, radius: f64, color: Color, albedo: f32) -> Sphere {
    Sphere {
      center,
      radius,
      color,
      albedo
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
  pub lights: Vec<Light>,
  pub shadow_bias: f64,
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
