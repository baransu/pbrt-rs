extern crate image;
extern crate serde;

use image::ImageBuffer;
use std::path::PathBuf;
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::scene::{Coloration, Intersection, Texture, Material, Element, Plane, Scene, Sphere, SurfaceType};
use pbrt::vector3::Vector3;
use pbrt::light::{Light, SphericalLight, DirectionalLight};
use pbrt::color::Color;

mod pbrt;

fn main() {

  let green_mat = Material {
    albedo: 0.18,
    color: Coloration::Color(Color { r: 0.4, g: 1.0, b: 0.4 }),
    surface: SurfaceType::Diffuse
  };

  let red_mat = Material {
    albedo: 0.18,
    color: Coloration::Color(Color { r: 1.0, g: 0.4, b: 0.4 }),
    surface: SurfaceType::Diffuse
  };

  let transparent_mat = Material {
    albedo: 0.18,
    color: Coloration::Color(Color { r: 1.0, g: 1.0, b: 1.0 }),
    surface: SurfaceType::Refractive { transparency: 0.5, index: 0.5 }
  };

  let blue_mat = Material {
    albedo: 0.18,
    color: Coloration::Texture(Texture::load_texture(PathBuf::from("./checkerboard.png")).unwrap()),
    surface: SurfaceType::Reflective { reflectivity: 0.1 }
  };

  let scene = Scene {
    max_recursion_depth: 32,
    shadow_bias: 1e-13,
    width: 2048,
    height: 2048,
    fov: 90.0,
    background: Color::black(),
    lights: vec![
      Light::Spherical(SphericalLight { 
        position: Point {x: -2.0, y: 0.0, z: 3.0},
        color: Color { r: 0.3, g: 0.3, b: 0.8},
        intensity: 10000.0
      }),
      Light::Spherical(SphericalLight { 
        position: Point {x: 2.0, y: 3.0, z: -4.0},
        color: Color { r: 1.0, g: 1.0, b: 1.0},
        intensity: 1000.0
      }),
      Light::Directional(DirectionalLight { 
        direction: Vector3 {x: 0.0, y: 0.0, z: -1.0},
        color: Color { r: 1.0, g: 1.0, b: 1.0},
        intensity: 0.0
      }),
    ],
    entities: vec![
      // floor
      Element::Plane(Plane {
        origin: Point::new(0.0, -3.0, -5.0),
        normal: Vector3::down(),
        material: Material {
          albedo: 0.18,
          color: Coloration::Texture(Texture::load_texture(PathBuf::from("./checkerboard.png")).unwrap()),
          surface: SurfaceType::Diffuse
        },
      }),
      // ceiling
      Element::Plane(Plane {
        origin: Point::new(0.0, 5.0, 5.0),
        normal: Vector3::up(),
        material: Material {
          albedo: 0.18,
          color: Coloration::Color(Color {r: 1.0, g: 1.0, b: 1.0}),
          surface: SurfaceType::Diffuse
        },
      }),
      // right wall
      Element::Plane(Plane {
        origin: Point::new(5.0, 0.0, 5.0),
        normal: Vector3::right(),
        material: Material {
          albedo: 0.18,
          color: Coloration::Color(Color {r: 1.0, g: 1.0, b: 1.0}),
          surface: SurfaceType::Diffuse
        },
      }),
      // left wall
      Element::Plane(Plane {
        origin: Point::new(-5.0, 0.0, 5.0),
        normal: Vector3::left(),
        material: Material {
          albedo: 0.18,
          color: Coloration::Color(Color {r: 1.0, g: 1.0, b: 1.0}),
          surface: SurfaceType::Diffuse
        },
      }),
      // back wall
      Element::Plane(Plane {
        origin: Point::new(0.0, 0.0, -10.0),
        normal: Vector3::backward(),
        material: Material {
          albedo: 0.18,
          color: Coloration::Color(Color {r: 1.0, g: 1.0, b: 1.0}),
          surface: SurfaceType::Diffuse
        },
      }),
      // front wall
      Element::Plane(Plane {
        origin: Point::new(0.0, 0.0, 10.0),
        normal: Vector3::forward(),
        material: Material {
          albedo: 0.18,
          color: Coloration::Color(Color {r: 1.0, g: 1.0, b: 1.0}),
          surface: SurfaceType::Diffuse
        },
      }),
      Element::Sphere(Sphere{ center: Point::new(0.0, 0.0, -5.0), radius: 1.0, material: green_mat}),
      Element::Sphere(Sphere{ center: Point::new(-3.0, 1.0, -6.0), radius: 2.0, material: transparent_mat }),
      Element::Sphere(Sphere{ center: Point::new(-2.0, -1.0, -3.0), radius: 1.0, material: red_mat }),
      Element::Sphere(Sphere{ center: Point::new(3.0, 0.0, -10.0), radius: 2.0, material: blue_mat}),
    ],
  };

  render(&scene).save("test.png").unwrap();
}

fn shade_diffuse(scene: &Scene, element: &Element, hit_point: &Point, surface_normal: &Vector3) -> Color {
  let mut color = Color {r: 0.0, g: 0.0, b: 0.0};

  let texture_coords = element.texture_coords(&hit_point);

  for light in &scene.lights {
    let direction_to_light = light.direction_from(&hit_point);

    let shadow_ray = Ray {
        origin: *hit_point + (*surface_normal * scene.shadow_bias),
        direction: direction_to_light,
    };

    let shadow_intersection = scene.trace(&shadow_ray);
    let in_light = shadow_intersection.is_none() ||
                   shadow_intersection.unwrap().distance > light.distance(&hit_point);

    let light_reflected = element.material().albedo / std::f32::consts::PI;

    let light_intensity = if in_light { light.intensity(&hit_point) } else { 0.0 };
    let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;

    color = color + element.material().color.color(&texture_coords) * light.color() * light_power * light_reflected;
  }

  color.clamp()
}

fn fresnel(incident: Vector3, normal: Vector3, index: f32) -> f64 {
  let i_dot_n = incident.dot(&normal);
  let mut eta_i = 1.0;
  let mut eta_t = index as f64;
  if i_dot_n > 0.0 {
    eta_i = eta_t;
    eta_t = 1.0;
  }

  let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
  if sin_t > 1.0 {
    //Total internal reflection
    return 1.0;
  } else {
    let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
    let cos_i = cos_t.abs();
    let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
    let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
    return (r_s * r_s + r_p * r_p) / 2.0;
  }
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
  let hit_point = ray.origin + (ray.direction * intersection.distance);
  let surface_normal = intersection.element.surface_normal(&hit_point);
  
  let material = intersection.element.material();

  match material.surface {
    SurfaceType::Diffuse => shade_diffuse(scene, intersection.element, &hit_point, &surface_normal),

    SurfaceType::Reflective { reflectivity } => {
      let mut color = shade_diffuse(scene, intersection.element, &hit_point, &surface_normal);
      let reflection_ray = Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);

      color = color * (1.0 - reflectivity);
      color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);

      color
    },

    SurfaceType::Refractive { transparency, index } => {
      let mut refraction_color = Color::black();

      let kr = fresnel(ray.direction, surface_normal, index) as f32;

      let surface_color = material.color.color(&intersection.element.texture_coords(&hit_point));
      if kr < 1.0 {
        let transmission_ray =
          Ray::create_transmission(surface_normal, ray.direction, hit_point, scene.shadow_bias, index).unwrap();

          refraction_color = cast_ray(scene, &transmission_ray, depth + 1)
      }

      let reflection_ray = 
        Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);

      let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
      let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
      color = color * transparency * surface_color;

      color
    }
  }
}

fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
  if depth >= scene.max_recursion_depth {
    return Color::black()
  }

  scene
    .trace(&ray)
    .map(|i| get_color(scene, &ray, &i, depth))
    .unwrap_or(Color::black())
}

fn render(scene: &Scene) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
  ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
    let ray = Ray::create_prime(x, y, scene);
    cast_ray(scene, &ray, 0).to_rgba()
  })
}
