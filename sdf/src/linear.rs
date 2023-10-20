use std::cmp::PartialEq;
use std::fmt;
use std::num;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[doc(hidden)]
#[macro_export]
macro_rules! op_default {
    ($func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl $bound for $cls {
            type Output = Self;

            fn $func(mut self, _rhs: Self) -> Self {
                for i in 0..self.size() {
                    self[i] $op _rhs[i];
                }
                self
            }
        }
    };
    ($type:ty, $func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl<I: Into<$type>> $bound<I> for $cls {
            type Output = Self;

            fn $func(mut self, _rhs: I) -> Self {
                self $op _rhs;
                self
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! op_assign {
    ($func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl $bound for $cls {
            fn $func(&mut self, _rhs: Self) {
                for i in 0..self.size() {
                    self[i] $op _rhs[i];
                }
            }
        }
    };
    ($type:ty, $func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl<I: Into<$type>> $bound<I> for $cls {
            fn $func(&mut self, _rhs: I) {
                let k = _rhs.into();
                for i in 0..self.size() {
                    self[i] $op k;
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    /// component of vector
    pub x: f32,
    /// component of vector
    pub y: f32,
    /// component of vector
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    /// Constructs a new `Vec3`.
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// // create `Vec3` with int
    /// let a = Vec3::new(10, 20, 30);
    /// // create `Vec3` with float
    /// let b = Vec3::new(3.5, 2.5, 1.5);
    /// // Supported types implemented for trait Into (with convertion to f32)
    /// ```
    pub fn new<I: Into<f32>>(x: I, y: I, z: I) -> Vec3 {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    /// Constructs a new `Vec3` from spherical coordinates $(r, \theta, \phi)$.
    ///
    /// # Example
    /// ```
    /// # use std::f32::consts::PI;
    /// # use linal::Vec3;
    /// // calculation error
    /// let eps = 1E-15;
    /// // Create `Vec3` use spherical coordinates
    /// let v = Vec3::from_spherical(2.0, PI / 2.0, PI / 2.0);
    /// assert!(v.x < eps && v.y - 2.0 < eps && v.z < eps);
    /// ```
    pub fn from_spherical<I: Into<f32>>(r: I, theta: I, phi: I) -> Vec3 {
        let (r, theta, phi) = (r.into(), theta.into(), phi.into());
        Vec3::new(
            r * f32::sin(theta) * f32::cos(phi),
            r * f32::sin(theta) * f32::sin(phi),
            r * f32::cos(theta),
        )
    }
    /// Create a zero `Vec3`
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// // create zero `Vec3`
    /// let zero = Vec3::zero();
    /// assert_eq!(zero, Vec3::new(0, 0, 0));
    /// ```
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    /// Scalar product
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// let a = Vec3::new(1, 2, 3);
    /// let b = Vec3::new(4, 5, 6);
    /// // The scalar production of `a` by `b`
    /// let r = a.dot(b);
    /// assert_eq!(r, 32.0);
    /// ```
    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    /// Cross product
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// let a = Vec3::new(1, 2, 3);
    /// let b = Vec3::new(2, 4, 6);
    /// let c = Vec3::zero();
    /// // Calculate cross production of `a` and `b` vectors
    /// let d = a.cross(b);
    /// assert_eq!(c, d);
    /// ```
    pub fn cross(self, rhs: Vec3) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    /// Vector length
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// let a = Vec3::new(4, 0, 0);
    /// let e = Vec3::new(0, 0, 1);
    /// let b = a.cross(e);
    /// // Calculate vector length
    /// let len1 = a.len();
    /// let len2 = b.len();
    /// assert!(a != b);
    /// assert!(len1 == len2 && len1 == 4.0);
    /// ```
    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }
    /// Unary vector, co-directed with given
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// let a = Vec3::new(2, 0, 0);
    /// // Calculate unary vector from `a`
    /// let b = a.ort();
    /// assert_eq!(b, Vec3::new(1, 0, 0));
    /// ```
    pub fn ort(self) -> Vec3 {
        self / self.len()
    }
    /// Squares of the vector coordinates
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// let a = Vec3::new(2, 3, 4);
    /// let b = Vec3::new(4, 9, 16);
    /// // Calculate squre of `a`
    /// let c = a.sqr();
    /// assert_eq!(b, c);
    /// ```
    pub fn sqr(self) -> Vec3 {
        self * self
    }
    /// Square root of vector coordinates
    ///
    /// # Example
    /// ```
    /// # use linal::Vec3;
    /// let a = Vec3::new(2, 3, 4);
    /// let b = Vec3::new(4, 9, 16);
    /// // Calculate squre root of `b`
    /// let c = b.sqrt();
    /// assert_eq!(a, c);
    /// ```
    pub fn sqrt(self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }
    pub fn normalize(self) -> Vec3 {
        let mag = self.dot(self).sqrt();
        if mag == 0.0 {
            self
        } else {
            self * 1.0 / mag
        }
    }

    // need for op_default & op_assign
    fn size(&self) -> usize {
        3
    }
}

op_default!(add, Add, +=, Vec3);
op_default!(sub, Sub, -=, Vec3);
op_default!(mul, Mul, *=, Vec3);
op_default!(f32, mul, Mul, *=, Vec3);
op_default!(f32, div, Div, /=, Vec3);
op_assign!(add_assign, AddAssign, +=, Vec3);
op_assign!(sub_assign, SubAssign, -=, Vec3);
op_assign!(mul_assign, MulAssign, *=, Vec3);
op_assign!(f32, mul_assign, MulAssign, *=, Vec3);
op_assign!(f32, div_assign, DivAssign, /=, Vec3);

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("Index {} out of [0, 2] range", i),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            i => panic!("Index {} out of [0, 2] range", i),
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl std::convert::From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vec3::new(x, y, z)
    }
}

impl FromStr for Vec3 {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let x: f32 = words[0].parse()?;
        let y: f32 = words[1].parse()?;
        let z: f32 = words[2].parse()?;
        Ok(Self::new(x, y, z))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        let w = vec.x * self.0[3][0] + vec.y * self.0[3][1] + vec.z * self.0[3][2] + self.0[3][3];

        Vec3 {
            x: (vec.x * self.0[0][0] + vec.y * self.0[0][1] + vec.z * self.0[0][2] + self.0[0][3])
                / w,
            y: (vec.x * self.0[1][0] + vec.y * self.0[1][1] + vec.z * self.0[1][2] + self.0[1][3])
                / w,
            z: (vec.x * self.0[2][0] + vec.y * self.0[2][1] + vec.z * self.0[2][2] + self.0[2][3])
                / w,
        }
    }
}

#[allow(dead_code)]
impl Mat4 {
    pub fn rotation(roll: f32, pitch: f32, yaw: f32) -> Mat4 {
        // Rotation around the x-axis (roll)
        let rx = Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, roll.cos(), -roll.sin(), 0.0],
            [0.0, roll.sin(), roll.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Rotation around the y-axis (pitch)
        let ry = Mat4([
            [pitch.cos(), 0.0, pitch.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-pitch.sin(), 0.0, pitch.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Rotation around the z-axis (yaw)
        let rz = Mat4([
            [yaw.cos(), -yaw.sin(), 0.0, 0.0],
            [yaw.sin(), yaw.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Combine the rotations
        let r = rz * ry * rx;
        r
    }

    pub fn invert(&self) -> Option<Mat4> {
        let m = &self.0;

        let a2323 = m[2][2] * m[3][3] - m[2][3] * m[3][2];
        let a1323 = m[2][1] * m[3][3] - m[2][3] * m[3][1];
        let a1223 = m[2][1] * m[3][2] - m[2][2] * m[3][1];
        let a0323 = m[2][0] * m[3][3] - m[2][3] * m[3][0];
        let a0223 = m[2][0] * m[3][2] - m[2][2] * m[3][0];
        let a0123 = m[2][0] * m[3][1] - m[2][1] * m[3][0];
        let a2313 = m[1][2] * m[3][3] - m[1][3] * m[3][2];
        let a1313 = m[1][1] * m[3][3] - m[1][3] * m[3][1];
        let a1213 = m[1][1] * m[3][2] - m[1][2] * m[3][1];
        let a0313 = m[1][0] * m[3][3] - m[1][3] * m[3][0];
        let a0213 = m[1][0] * m[3][2] - m[1][2] * m[3][0];
        let a0113 = m[1][0] * m[3][1] - m[1][1] * m[3][0];
        let a2312 = m[1][2] * m[2][3] - m[1][3] * m[2][2];
        let a1312 = m[1][1] * m[2][3] - m[1][3] * m[2][1];
        let a1212 = m[1][1] * m[2][2] - m[1][2] * m[2][1];
        let a0312 = m[1][0] * m[2][3] - m[1][3] * m[2][0];
        let a0212 = m[1][0] * m[2][2] - m[1][2] * m[2][0];
        let a0112 = m[1][0] * m[2][1] - m[1][1] * m[2][0];

        let det = m[0][0] * (m[1][1] * a2323 - m[1][2] * a1323 + m[1][3] * a1223)
            - m[0][1] * (m[1][0] * a2323 - m[1][2] * a0323 + m[1][3] * a0223)
            + m[0][2] * (m[1][0] * a1323 - m[1][1] * a0323 + m[1][3] * a0123)
            - m[0][3] * (m[1][0] * a1223 - m[1][1] * a0223 + m[1][2] * a0123);

        if det.abs() < 1e-10 {
            return None; // The matrix is singular and cannot be inverted
        }

        let inv_det = 1.0 / det;

        let adjugate = [
            [
                (m[1][1] * a2323 - m[1][2] * a1323 + m[1][3] * a1223) * inv_det,
                (-m[0][1] * a2323 + m[0][2] * a1323 - m[0][3] * a1223) * inv_det,
                (m[0][1] * a2313 - m[0][2] * a1313 + m[0][3] * a1213) * inv_det,
                (-m[0][1] * a2312 + m[0][2] * a1312 - m[0][3] * a1212) * inv_det,
            ],
            [
                (-m[1][0] * a2323 + m[1][2] * a0323 - m[1][3] * a0223) * inv_det,
                (m[0][0] * a2323 - m[0][2] * a0323 + m[0][3] * a0223) * inv_det,
                (-m[0][0] * a2313 + m[0][2] * a0313 - m[0][3] * a0213) * inv_det,
                (m[0][0] * a2312 - m[0][2] * a0312 + m[0][3] * a0212) * inv_det,
            ],
            [
                (m[1][0] * a1323 - m[1][1] * a0323 + m[1][3] * a0123) * inv_det,
                (-m[0][0] * a1323 + m[0][1] * a0323 - m[0][3] * a0123) * inv_det,
                (m[0][0] * a1313 - m[0][1] * a0313 + m[0][3] * a0113) * inv_det,
                (-m[0][0] * a1312 + m[0][1] * a0312 - m[0][3] * a0112) * inv_det,
            ],
            [
                (-m[1][0] * a1223 + m[1][1] * a0223 - m[1][2] * a0123) * inv_det,
                (m[0][0] * a1223 - m[0][1] * a0223 + m[0][2] * a0123) * inv_det,
                (-m[0][0] * a1213 + m[0][1] * a0213 - m[0][2] * a0113) * inv_det,
                (m[0][0] * a1212 - m[0][1] * a0212 + m[0][2] * a0112) * inv_det,
            ],
        ];

        Some(Mat4(adjugate))
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.0[i][k] * other.0[k][j];
                }
            }
        }
        Mat4(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_matrix_invert() {
        let m = Mat4([
            [1.0, 2.0, 3.0, 5.0],
            [7.0, 11.0, 13.0, 17.0],
            [23.0, 27.0, 31.0, 37.0],
            [43.0, 47.0, 51.0, 53.0],
        ]);
        panic!("{:?} {:?}", m, m.invert().unwrap().invert().unwrap());
    }
}
