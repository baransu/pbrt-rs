use image::{Pixel, Rgba};
use std::ops::{Add, Mul};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
  linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
  encoded.powf(GAMMA)
}

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
  pub fn white() -> Color {
    Color {
      r: 1.0,
      g: 1.0,
      b: 1.0,
    }
  }

  pub fn clamp(&self) -> Color {
    Color {
      r: self.r.min(1.0).max(0.0),
      g: self.g.min(1.0).max(0.0),
      b: self.b.min(1.0).max(0.0),
    }
  }

  pub fn to_rgba(&self) -> Rgba<u8> {
    Rgba::from_channels(
      (gamma_encode(self.r) * 255.0) as u8,
      (gamma_encode(self.g) * 255.0) as u8,
      (gamma_encode(self.b) * 255.0) as u8,
      255,
    )
  }

  pub fn from_rgba(rgba: Rgba<u8>) -> Color {
    Color {
      r: gamma_decode((rgba.data[0] as f32) / 255.0),
      g: gamma_decode((rgba.data[1] as f32) / 255.0),
      b: gamma_decode((rgba.data[2] as f32) / 255.0),
    }
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

impl Add for Color {
  type Output = Color;

  fn add(self, other: Color) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b,
    }
  }
}
