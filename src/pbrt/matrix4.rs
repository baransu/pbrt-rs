use pbrt::vector3::Vector3;
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Debug)]
pub struct Matrix4x4 {
    elements: [[f64; 4]; 4],
}

impl Matrix4x4 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            elements: [[1.0, 0.0, 0.0, 0.0],
                       [0.0, 1.0, 0.0, 0.0],
                       [0.0, 0.0, 1.0, 0.0],
                       [0.0, 0.0, 0.0, 1.0]]
        }
    }

    pub fn scale_linear(s: f64) -> Matrix4x4 {
        Matrix4x4::scale(s, s, s)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn scale(sx: f64, sy: f64, sz: f64) -> Matrix4x4 {
        Matrix4x4 {
            elements: [[ sx, 0.0, 0.0, 0.0],
                       [0.0,  sy, 0.0, 0.0],
                       [0.0, 0.0,  sz, 0.0],
                       [0.0, 0.0, 0.0, 1.0]]
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotate_x(t: f64) -> Matrix4x4 {
        let sin = t.sin();
        let cos = t.cos();
        Matrix4x4 {
            elements: [[1.0, 0.0, 0.0, 0.0],
                       [0.0, cos, sin, 0.0],
                       [0.0,-sin, cos, 0.0],
                       [0.0, 0.0, 0.0, 1.0]],
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotate_y(t: f64) -> Matrix4x4 {
        let sin = t.sin();
        let cos = t.cos();
        Matrix4x4 {
            elements: [[cos, 0.0, -sin, 0.0],
                       [0.0, 1.0, 0.0, 0.0],
                       [sin, 0.0, cos, 0.0],
                       [0.0, 0.0, 0.0, 1.0]],
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotate_z(t: f64) -> Matrix4x4 {
        let sin = t.sin();
        let cos = t.cos();
        Matrix4x4 {
            elements: [[cos, sin, 0.0, 0.0],
                       [-sin, cos, 0.0, 0.0],
                       [0.0, 0.0, 1.0, 0.0],
                       [0.0, 0.0, 0.0, 1.0]],
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn translate(tx: f64, ty:f64, tz: f64) -> Matrix4x4 {
        Matrix4x4 {
            elements: [[1.0, 0.0, 0.0,  tx],
                       [0.0, 1.0, 0.0,  ty],
                       [0.0, 0.0, 1.0,  tz],
                       [0.0, 0.0, 0.0, 1.0]],
        }
    }

    pub fn inverse(&self) -> Matrix4x4 {
        let mut s = Matrix4x4::identity();
        let mut t = self.clone();
        // Forward elimination
        for i in 0..3 {
            let mut pivot = i;
            let mut pivotsize = t[i][i].abs();
            for j in (i + 1)..4 {
                let tmp = t[j][i].abs();
                if tmp > pivotsize {
                    pivot = j;
                    pivotsize = tmp;
                }
            }

            if pivotsize == 0.0 {
                return Matrix4x4::identity();
            }
            if pivot != i {
                for j in 0..4 {
                    let mut tmp: f64;

                    tmp = t[i][j];
                    t[i][j] = t[pivot][j];
                    t[pivot][j] = tmp;

                    tmp = s[i][j];
                    s[i][j] = s[pivot][j];
                    s[pivot][j] = tmp;
                }
            }
            for j in (i + 1)..4 {
                let f = t[j][i] / t[i][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }
        // Backward substitution
        for i in (0..4).rev() {
            let mut f: f64 = t[i][i];

            if f == 0.0 {
                // Cannot invert singular matrix
                return Matrix4x4::identity();
            }

            for j in 0..4 {
                t[i][j] /= f;
                s[i][j] /= f;
            }

            for j in 0..i {
                f = t[j][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }

        return s;
    }
}
impl Index<usize> for Matrix4x4 {
    type Output = [f64; 4];

    fn index(&self, idx: usize) -> &[f64; 4] {
        &self.elements[idx]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, idx: usize) -> &mut [f64; 4] {
        &mut self.elements[idx]
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][0] * other[0][j] +
                               self[i][1] * other[1][j] +
                               self[i][2] * other[2][j] +
                               self[i][3] * other[3][j];
            }
        }
        result
    }
}

impl Mul<Vector3> for Matrix4x4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        let result = Vector3 {
            x: other.x * self[0][0] + other.y * self[0][1] + other.z * self[0][2] + self[0][3],
            y: other.x * self[1][0] + other.y * self[1][1] + other.z * self[1][2] + self[1][3],
            z: other.x * self[2][0] + other.y * self[2][1] + other.z * self[2][2] + self[2][3],
        };

        result
    }
}
