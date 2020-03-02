extern crate image;
extern crate serde;

use image::{DynamicImage, GenericImage};
use pbrt::color::Color;
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::vector3::Vector3;
use std::fmt;
use std::path::PathBuf;

pub struct Texture {
  pub path: PathBuf,
  pub texture: DynamicImage,
}

impl fmt::Debug for Texture {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Texture({:?})", self.path)
  }
}

impl Texture {
  pub fn load_texture(path: PathBuf) -> Result<Texture, String> {
    if let Ok(img) = image::open(path.clone()) {
      Ok(Texture { path, texture: img })
    } else {
      Err(format!("Unable to open texture file: {:?}", &path))
    }
  }
}

pub struct TextureCoords {
  pub x: f32,
  pub y: f32,
}

#[derive(Debug)]
pub enum Coloration {
  Color(Color),
  Texture(Texture),
}

fn wrap(val: f32, bound: u32) -> u32 {
  let signed_bound = bound as i32;
  let float_coord = val * bound as f32;
  let wrapped_coord = (float_coord as i32) % signed_bound;
  if wrapped_coord < 0 {
    (wrapped_coord + signed_bound) as u32
  } else {
    wrapped_coord as u32
  }
}

impl Coloration {
  pub fn color(&self, texture_coords: &TextureCoords) -> Color {
    match *self {
      Coloration::Color(color) => color,
      Coloration::Texture(ref texture) => {
        let tex_x = wrap(texture_coords.x, texture.texture.width());
        let tex_y = wrap(texture_coords.y, texture.texture.height());

        Color::from_rgba(texture.texture.get_pixel(tex_x, tex_y))
      }
    }
  }
}

#[derive(Debug)]
pub enum Material {
  Diffuse { albedo: f32, color: Coloration },
  Reflective,
  Refractive { index: f32 },
  Emissive { emission: Color, intensity: f32 },
}

pub struct Polygon {
  pub vertices: [Vector3; 3],
  pub normal: Vector3,
  pub material: Material,
}

pub struct Plane {
  pub origin: Point,
  pub normal: Vector3,
  pub material: Material,
}

pub enum Element {
  Sphere(Sphere),
  Plane(Plane),
  Polygon(Polygon),
}

impl Element {
  pub fn material(&self) -> &Material {
    match *self {
      Element::Sphere(ref s) => &s.material,
      Element::Plane(ref p) => &p.material,
      Element::Polygon(ref p) => &p.material,
    }
  }
}

impl Intersectable for Element {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    match *self {
      Element::Sphere(ref s) => s.intersect(&ray),
      Element::Plane(ref p) => p.intersect(&ray),
      Element::Polygon(ref p) => p.intersect(&ray),
    }
  }

  fn surface_normal(&self, hit_point: &Point) -> Vector3 {
    match *self {
      Element::Sphere(ref s) => s.surface_normal(&hit_point),
      Element::Plane(ref p) => p.surface_normal(&hit_point),
      Element::Polygon(ref p) => p.surface_normal(&hit_point),
    }
  }

  fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
    match *self {
      Element::Sphere(ref s) => s.texture_coords(&hit_point),
      Element::Plane(ref p) => p.texture_coords(&hit_point),
      Element::Polygon(ref p) => p.texture_coords(&hit_point),
    }
  }
}

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub material: Material,
}

pub struct Intersection<'a> {
  pub distance: f64,
  pub element: &'a Element,
}

impl<'a> Intersection<'a> {
  pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
    Intersection { distance, element }
  }
}

pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub entities: Vec<Element>,
}

impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self
      .entities
      .iter()
      .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
      .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
  }
}
