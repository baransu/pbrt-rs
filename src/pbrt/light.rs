use pbrt::vector3::Vector3;
use pbrt::point::Point;
use pbrt::color::Color;

pub struct DirectionalLight {
  pub direction: Vector3,
  pub color: Color,
  pub intensity: f32,
}

pub struct SphericalLight {
  pub position: Point,
  pub color: Color,
  pub intensity: f32,
}

pub enum Light {
  Directional(DirectionalLight),
  Spherical(SphericalLight),
}

impl Light {
  pub fn color(&self) -> Color {
    match *self {
      Light::Directional(ref d) => d.color,
      Light::Spherical(ref s) => s.color,
    }
  }

  pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
    match *self {
      Light::Directional(ref d) => -d.direction,
      Light::Spherical(ref s) => (s.position - *hit_point).normalize(),
    }
  }

  pub fn intensity(&self, hit_point: &Point) -> f32 {
    match *self {
      Light::Directional(ref d) => d.intensity,
      Light::Spherical(ref s) => {
        let r2 = (s.position - *hit_point).norm() as f32;
        s.intensity / (4.0 * ::std::f32::consts::PI * r2)
      }
    }
  }

  pub fn distance(&self, hit_point: &Point) -> f64 {
    match *self {
      Light::Directional(_) => ::std::f64::INFINITY,
      Light::Spherical(ref s) => (s.position - *hit_point).length(),
    }
  }
}