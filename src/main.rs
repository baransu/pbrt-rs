extern crate image;
extern crate obj;
extern crate rand;
extern crate rayon;
extern crate serde;

use image::ImageBuffer;
use obj::Obj;
use pbrt::color::Color;
use pbrt::matrix4::Matrix4x4;
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::scene::{Coloration, Element, Material, Plane, Polygon, Scene, Sphere, Texture};
use pbrt::vector3::Vector3;
use rand::Rng;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

mod pbrt;

const FLOATING_POINT_BACKOFF: f64 = 0.01;
const RAY_COUNT: u32 = 16;
const BOUNCE_CAP: u32 = 8;
// RAY_COUNT + BOUNCE_CAP
const ROUND_COUNT: u32 = 128;
const NUM_RAYS: usize = 16;

fn convert_objects_to_polygons(
  obj: &Obj<obj::SimplePolygon>,
  object_to_world: Matrix4x4,
) -> Vec<Element> {
  let mut polygons = vec![];

  let make_vector = |floats: &[f32; 3]| {
    let v = Vector3 {
      x: floats[0] as f64,
      y: floats[1] as f64,
      z: floats[2] as f64,
    };

    let t = object_to_world.clone() * v;
    t
  };

  let make_polygon = |index1, index2, index3| {
    let obj::IndexTuple(index1, _, _) = index1;
    let obj::IndexTuple(index2, _, _) = index2;
    let obj::IndexTuple(index3, _, _) = index3;

    let vertex1 = make_vector(&obj.position[index1]);
    let vertex2 = make_vector(&obj.position[index2]);
    let vertex3 = make_vector(&obj.position[index3]);

    let a = vertex2 - vertex1;
    let b = vertex3 - vertex1;

    let normal = a.cross(&b).normalize();

    Element::Polygon(Polygon {
      vertices: [vertex1, vertex2, vertex3],
      normal,
      material: Material::Diffuse {
        albedo: 0.18,
        color: Coloration::Color(Color {
          r: 0.4,
          g: 1.0,
          b: 0.4,
        }),
      },
    })
  };

  for object in &obj.objects {
    for group in &object.groups {
      for poly in &group.polys {
        let index1 = poly[0];
        for others in poly[1..].windows(2) {
          let polygon = make_polygon(index1, others[0], others[1]);
          polygons.push(polygon);
        }
      }
    }
  }

  return polygons;
}

fn main() {
  let load_start = Instant::now();

  let mesh_path = Path::new("teapot.obj");
  let mesh: Obj<obj::SimplePolygon> = Obj::load(mesh_path).expect("Failed to load mesh");

  let object_to_world_matrix = Matrix4x4::translate(0.0, -3.0 - -1.575, -5.0)
    * Matrix4x4::scale_linear(1.0)
    * Matrix4x4::translate(1.0, -1.575, 0.0);

  let teapot_1_polygons = convert_objects_to_polygons(&mesh, object_to_world_matrix);

  // let green_mat = Material::Diffuse {
  //   albedo: 0.18,
  //   color: Coloration::Color(Color {
  //     r: 0.4,
  //     g: 1.0,
  //     b: 0.4,
  //   }),
  // };

  let red_mat = Material::Emissive {
    intensity: 200.0,
    emission: Color {
      r: 1.0,
      g: 0.0,
      b: 0.0,
    },
  };

  let transparent_mat = Material::Refractive { index: 1.5 };

  let blue_mat = Material::Reflective;

  let entities = vec![
    // floor
    Element::Plane(Plane {
      origin: Point::new(0.0, -3.0, -5.0),
      normal: Vector3::down(),
      material: Material::Diffuse {
        albedo: 0.18,
        color: Coloration::Texture(
          Texture::load_texture(PathBuf::from("./checkerboard.png")).unwrap(),
        ),
      },
    }),
    // ceiling
    Element::Plane(Plane {
      origin: Point::new(0.0, 5.0, 5.0),
      normal: Vector3::up(),
      material: Material::Emissive {
        intensity: 200.0,
        emission: Color {
          r: 1.0,
          g: 1.0,
          b: 1.0,
        },
      },
      // material: Material::Diffuse {
      //   albedo: 0.18,
      //   color: Coloration::Color(Color {
      //     r: 1.0,
      //     g: 1.0,
      //     b: 1.0,
      //   }),
      // },
    }),
    // right wall
    Element::Plane(Plane {
      origin: Point::new(5.0, 0.0, 5.0),
      normal: Vector3::right(),
      material: Material::Diffuse {
        albedo: 0.18,
        color: Coloration::Color(Color {
          r: 1.0,
          g: 1.0,
          b: 1.0,
        }),
      },
    }),
    // left wall
    Element::Plane(Plane {
      origin: Point::new(-5.0, 0.0, 5.0),
      normal: Vector3::left(),
      material: Material::Diffuse {
        albedo: 0.18,
        color: Coloration::Color(Color {
          r: 1.0,
          g: 1.0,
          b: 1.0,
        }),
      },
    }),
    // back wall
    Element::Plane(Plane {
      origin: Point::new(0.0, 0.0, -10.0),
      normal: Vector3::backward(),
      material: Material::Diffuse {
        albedo: 0.18,
        color: Coloration::Color(Color {
          r: 1.0,
          g: 1.0,
          b: 1.0,
        }),
      },
    }),
    // front wall
    Element::Plane(Plane {
      origin: Point::new(0.0, 0.0, 10.0),
      normal: Vector3::forward(),
      material: Material::Diffuse {
        albedo: 0.18,
        color: Coloration::Color(Color {
          r: 1.0,
          g: 1.0,
          b: 1.0,
        }),
      },
    }),
    // Element::Sphere(Sphere {
    //   center: Point::new(0.0, 0.0, -5.0),
    //   radius: 1.0,
    //   material: green_mat,
    // }),
    Element::Sphere(Sphere {
      center: Point::new(-3.0, 1.0, -6.0),
      radius: 2.0,
      material: transparent_mat,
    }),
    Element::Sphere(Sphere {
      center: Point::new(-2.0, -2.0, -6.0),
      radius: 1.0,
      material: red_mat,
    }),
    Element::Sphere(Sphere {
      center: Point::new(3.0, 0.0, -10.0),
      radius: 2.0,
      material: blue_mat,
    }),
  ];

  let scene = Scene {
    width: 1280,
    height: 720,
    fov: 90.0,
    entities: entities
      .into_iter()
      .chain(teapot_1_polygons.into_iter())
      .collect(),
  };

  let load_time = load_start.elapsed();
  println!("Load time: {:?}", load_time);

  render(&scene).save("test.png").unwrap();
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

fn create_scatter_direction(normal: &Vector3) -> (Vector3, f32) {
  let mut rng = rand::thread_rng();
  let r1: f64 = rng.gen();
  let r2: f64 = rng.gen();

  let y = r1;
  let azimuth = r2 * 2.0 * std::f64::consts::PI;
  let sin_elevation = (1.0 - y * y).sqrt();
  let x = sin_elevation * (azimuth).cos();
  let z = sin_elevation * (azimuth).sin();

  let hemisphere_vec = Vector3 { x, y, z };

  let (n_t, n_b) = create_coordinate_system(normal);

  let scatter = Vector3 {
    x: hemisphere_vec.x * n_b.x + hemisphere_vec.y * normal.x + hemisphere_vec.z * n_t.x,
    y: hemisphere_vec.x * n_b.y + hemisphere_vec.y * normal.y + hemisphere_vec.z * n_t.y,
    z: hemisphere_vec.x * n_b.z + hemisphere_vec.y * normal.z + hemisphere_vec.z * n_t.z,
  };

  let weight = (1.0 / scatter.dot(normal)) as f32;

  (scatter, weight)
}

fn create_coordinate_system(normal: &Vector3) -> (Vector3, Vector3) {
  let n_t = if (normal.x.abs()) > (normal.y.abs()) {
    Vector3 {
      x: normal.z,
      y: 0.0,
      z: -normal.x,
    }
    .normalize()
  } else {
    Vector3 {
      x: 0.0,
      y: -normal.z,
      z: normal.y,
    }
    .normalize()
  };
  let n_b = normal.cross(&n_t);

  (n_t, n_b)
}

fn make_reflection(incident: Vector3, normal: Vector3) -> Vector3 {
  incident - normal * (2.0 * incident.dot(&normal))
}

fn get_color(scene: &Scene, x: u32, y: u32) -> Color {
  let mut color_acc = Color::black();

  let mut rays = vec![];
  let mut masks = vec![];

  rays.push(Ray::create_prime(x, y, scene));
  masks.push(Color::white());

  let mut bounce_i = 0;
  while bounce_i < BOUNCE_CAP {
    let mut ray_i = (rays.len() - 1) as i32;
    while ray_i >= 0 {
      let ray_u = ray_i as usize;
      let mut ray = *rays.get_mut(ray_u).unwrap();
      let mut color_mask = *masks.get_mut(ray_u).unwrap();

      if let Some(intersection) = scene.trace(&ray) {
        let hit_point = ray.origin + (ray.direction * intersection.distance);
        let surface_normal = intersection.element.surface_normal(&hit_point);

        ray.origin = hit_point + (surface_normal * FLOATING_POINT_BACKOFF);
        let material = intersection.element.material().clone();

        match material {
          Material::Diffuse { color, albedo } => {
            let texture_coords = intersection.element.texture_coords(&hit_point);

            let (direction, weight) = create_scatter_direction(&surface_normal);
            ray.direction = direction;

            let cosine_angle = direction.dot(&surface_normal) as f32;
            let reflected_power = albedo * std::f32::consts::PI;
            let reflected_color =
              color.color(&texture_coords) * cosine_angle * reflected_power * weight;

            color_mask = color_mask * reflected_color;
          }

          Material::Emissive {
            emission,
            intensity,
          } => {
            let (direction, _) = create_scatter_direction(&surface_normal);
            ray.direction = direction;
            color_acc = color_acc + (*emission * color_mask * *intensity);
          }

          Material::Reflective => {
            ray.direction = make_reflection(ray.direction, surface_normal);
          }

          Material::Refractive { index } => {
            let kr = fresnel(ray.direction, surface_normal, *index) as f32;

            if kr < 1.0 {
              if rays.len() < NUM_RAYS {
                rays.push(
                  Ray::create_transmission(
                    surface_normal,
                    ray.direction,
                    hit_point,
                    FLOATING_POINT_BACKOFF,
                    *index,
                  )
                  .unwrap(),
                );
                masks.push(color_mask * (1.0 - kr));
              }
            }

            ray.direction = make_reflection(ray.direction, surface_normal);

            color_mask = color_mask * kr
          }
        }
      } else {
        print!("Resseting to black: {:?}\n", color_mask);
        color_mask = Color::black();
      }

      rays[ray_u] = ray;
      masks[ray_u] = color_mask;

      ray_i -= 1;
    }
    bounce_i += 1;
  }

  return color_acc;
}

fn render_pixel(scene: &Scene, x: &u32, y: &u32) -> Vec<u8> {
  let mut ray_num = 0;
  let mut color_acc = Color::black();

  while ray_num < RAY_COUNT {
    color_acc = color_acc + get_color(scene, *x, *y) * (1.0 / (RAY_COUNT * ROUND_COUNT) as f32);
    ray_num += 1;
  }

  color_acc.clamp().to_rgba().data.to_vec()
}

fn render(scene: &Scene) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
  let height = scene.height as usize;
  let width = scene.width as usize;
  let mut buffer = vec![vec![(0, 0); width]; height];

  for x in 0..width {
    for y in 0..height {
      buffer[y][x] = (x as u32, y as u32);
    }
  }

  let source: Vec<u8> = buffer
    .clone()
    .par_iter()
    .flat_map(|vec| vec)
    .flat_map(|(x, y)| render_pixel(&scene, &x, &y))
    .collect();
  ImageBuffer::from_vec(scene.width, scene.height, source).unwrap()
  // ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
  //   let mut ray_num = 0;
  //   let mut color_acc = Color::black();

  //   while ray_num < RAY_COUNT {
  //     color_acc = color_acc
  //       + get_color(scene, x, y)
  //       * (1.0 / (RAY_COUNT * ROUND_COUNT) as f32);
  //     ray_num += 1;
  //   }

  //   color_acc.clamp().to_rgba()
  // })
}
