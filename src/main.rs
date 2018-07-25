extern crate image;
extern crate serde;

use image::ImageBuffer;
use pbrt::point::Point;
use pbrt::rendering::{Intersectable, Ray};
use pbrt::scene::{Color, Scene, Sphere};

mod pbrt;

fn main() {
  let scene = Scene {
    width: 800,
    height: 600,
    fov: 90.0,
    sphere: Sphere {
      center: Point {
        x: 0.0,
        y: 0.0,
        z: -5.0,
      },
      radius: 1.0,
      color: Color {
        r: 0.4,
        g: 1.0,
        b: 0.4,
      },
    },
  };

  render(&scene).save("test.png").unwrap();
}

fn render(scene: &Scene) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
  let black = Color::black().to_rgba();
  ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
    let ray = Ray::create_prime(x, y, scene);

    if scene.sphere.intersect(&ray) {
      let rgba = &scene.sphere.color.to_rgba();
      image::Rgba(*rgba)
    } else {
      image::Rgba(black)
    }
  })
}
