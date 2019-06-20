use pbrt::point::Point;
use pbrt::scene::{TextureCoords, Plane, Scene, Sphere};
use pbrt::vector3::Vector3;

#[derive(Debug)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vector3,
}

impl Ray {
  pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
    let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
    let aspect_ratio = (scene.width as f64) / (scene.height as f64);
    let sensor_x =
      ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
    let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

    Ray {
      origin: Point::zero(),
      direction: Vector3 {
        x: sensor_x,
        y: sensor_y,
        z: -1.0,
      }.normalize(),
    }
  }

  pub fn create_reflection(normal: Vector3, incident: Vector3, intersection: Point, bias: f64) -> Ray {
    Ray {
      origin: intersection + (normal * bias),
      direction: incident - (2.0 * incident.dot(&normal) * normal),
    }
  }

  pub fn create_transmission(normal: Vector3, incident: Vector3, intersection: Point, bias: f64, index: f32) -> Option<Ray> {
    let mut ref_n = normal;
    let mut eta_t = index as f64;
    let mut eta_i = 1.0f64;
    let mut i_dot_n = incident.dot(&normal);
    if i_dot_n < 0.0 {
      // outside of surface
      i_dot_n = -i_dot_n;
    } else { 
      // inside surface; invert normal and swap he indicies of reflection
      ref_n = -normal;
      eta_t = 1.0f64;
      eta_i = index as f64;
    }

    let eta = eta_i / eta_t;
    let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
    if k < 0.0 {
      None
    } else {
      Some(Ray{
        origin: intersection + (ref_n * -bias),
        direction: (incident + i_dot_n * ref_n) * eta - ref_n * k.sqrt()
      })
    }
  }
}

pub trait Intersectable {
  fn intersect(&self, ray: &Ray) -> Option<f64>;

  fn surface_normal(&self, hit_point: &Point) -> Vector3;
  fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    let l: Vector3 = self.center - ray.origin;

    let adj = l.dot(&ray.direction);

    let d2 = l.dot(&l) - (adj * adj);

    let radius2 = self.radius * self.radius;

    if d2 > radius2 {
      return None;
    }

    let thc = (radius2 - d2).sqrt();
    let t0 = adj - thc;
    let t1 = adj + thc;

    if t0 < 0.0 && t1 < 0.0 {
      None
    } else if t0 < 0.0 {
      Some(t1)
    } else if t1 < 0.0 {
      Some(t0)
    } else {
      let distance = if t0 < t1 { t0 } else { t1 };
      Some(distance)
    }
  }

  fn surface_normal(&self, hit_point: &Point) -> Vector3 {
    (*hit_point - self.center).normalize()
  }

  fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
    let hit_vec = *hit_point - self.center;
    TextureCoords {
      x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) / std::f32::consts::PI) * 0.5,
      y: (hit_vec.y / self.radius).acos() as f32 / std::f32::consts::PI
    }
  }
}

impl Intersectable for Plane {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    let normal = &self.normal;
    let denom = normal.dot(&ray.direction);
    if denom > 1e-6 {
      let v = self.origin - ray.origin;
      let distance = v.dot(&normal) / denom;
      if distance >= 0.0 {
        return Some(distance);
      }
    }
    None
  }

  fn surface_normal(&self, _: &Point) -> Vector3 {
    -self.normal
  }

  fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
    let mut x_axis = self.normal.cross(&Vector3 { x: 0.0, y: 0.0, z: 1.0,});
    if x_axis.length() == 0.0 {
      x_axis = self.normal.cross(&Vector3 { x: 0.0, y: 1.0, z: 0.0 });
    }
    let y_axis = self.normal.cross(&x_axis);

    let hit_vec = *hit_point - self.origin;

    TextureCoords {
        x: hit_vec.dot(&x_axis) as f32,
        y: hit_vec.dot(&y_axis) as f32,
    }
  }
}
