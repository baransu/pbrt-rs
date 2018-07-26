extern crate image;
extern crate serde;

use image::ImageBuffer;
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::scene::{Color, Element, Plane, Scene, Sphere};
use pbrt::vector3::Vector3;

mod pbrt;

fn main() {
  let green = Color {
    r: 0.4,
    g: 1.0,
    b: 0.4,
  };

  let red = Color {
    r: 1.0,
    g: 0.0,
    b: 0.4,
  };

  let blue = Color {
    r: 0.4,
    g: 0.4,
    b: 1.0,
  };

  let scene = Scene {
    width: 800,
    height: 800,
    fov: 90.0,
    background: Color {
      r: 0.41,
      g: 0.85,
      b: 1.0,
    },
    entities: vec![
      Element::Plane(Plane {
        origin: Point::new(0.0, -2.0, -5.0),
        normal: Vector3::down(),
        color: Color {
          r: 0.4,
          g: 0.4,
          b: 0.4,
        },
      }),
      Element::Sphere(Sphere::new(Point::new(-3.0, 4.0, -10.0), 2.0, red)),
      Element::Sphere(Sphere::new(Point::new(0.0, -2.0, -10.0), 1.0, green)),
      Element::Sphere(Sphere::new(Point::new(5.0, 0.0, -10.0), 3.0, blue)),
    ],
  };

  render(&scene).save("test.png").unwrap();
}

fn render(scene: &Scene) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
  ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
    let ray = Ray::create_prime(x, y, scene);

    if let Some(intersection) = scene.trace(&ray) {
      let rgba = intersection.object.color().to_rgba();
      image::Rgba(rgba)
    } else {
      image::Rgba(scene.background.to_rgba())
    }
  })
}
