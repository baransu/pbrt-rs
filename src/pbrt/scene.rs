extern crate image;
extern crate serde;

use std::path::PathBuf;
use std::fmt;
use image::{DynamicImage, GenericImage};
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::vector3::Vector3;
use pbrt::light::Light;
use pbrt::color::Color;

pub struct Texture {
    pub path: PathBuf,

    pub texture: DynamicImage,
}

// fn dummy_texture() -> DynamicImage {
//     DynamicImage::new_rgb8(0, 0)
// }

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture({:?})", self.path)
    }
}

impl Texture {
  pub fn load_texture(path: PathBuf) -> Result<Texture, String> {
    if let Ok(img) = image::open(path.clone()) {
        Ok(Texture {
            path,
            texture: img,
        })
    } else {
        Err(format!(
            "Unable to open texture file: {:?}",
            &path
        ))
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
  Texture(Texture)
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
#[repr(C)]
pub struct Material {
  pub albedo: f32,
  pub color: Coloration,
}

pub struct Plane {
  pub origin: Point,
  pub normal: Vector3,
  pub material: Material
}

pub enum Element {
  Sphere(Sphere),
  Plane(Plane),
}

impl Element {
  pub fn material(&self) -> &Material {
    match *self {
      Element::Sphere(ref s) => &s.material,
      Element::Plane(ref p) => &p.material,
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

  fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
    match *self {
      Element::Sphere(ref s) => s.texture_coords(&hit_point),
      Element::Plane(ref p) => p.texture_coords(&hit_point),
    }
  }
}

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub material: Material,
}

pub struct Intersecion<'a> {
  pub distance: f64,
  pub element: &'a Element,
}

impl<'a> Intersecion<'a> {
  pub fn new<'b>(distance: f64, element: &'b Element) -> Intersecion<'b> {
    Intersecion { distance, element }
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
