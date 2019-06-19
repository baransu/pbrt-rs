use pbrt::vector3::Vector3;
use pbrt::scene::Color;

pub struct Light {
  pub direction: Vector3,
  pub color: Color,
  pub intensity: f32 
}
