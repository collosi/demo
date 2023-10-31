#![allow(dead_code)]

use core::ops::*;

#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::*;

mod fallback;
#[cfg(not(target_arch = "wasm32"))]
pub use fallback::*;

#[cfg(target_arch = "wasm32")]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vec4(pub(crate) v128);

#[cfg(target_arch = "wasm32")]
fn f32x4_agg(f: v128) -> f32 {
    let x = f32x4_extract_lane::<0>(f);
    let y = f32x4_extract_lane::<1>(f);
    let z = f32x4_extract_lane::<2>(f);
    let w = f32x4_extract_lane::<3>(f);
    x + y + z + w
}

impl Vec4 {
    pub const fn zero() -> Vec4 {
        Vec4(f32x4(0.0, 0.0, 0.0, 0.0))
    }

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4(f32x4(x, y, z, w))
    }
    pub const fn new3(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4::new(x,y,z,0.0)
    }
    fn agg(self) -> f32 {
        let (x, y, z, w) = self.extract();
        x + y + z + w
    }
    pub fn extract(self) -> (f32, f32, f32, f32) {
        (
            f32x4_extract_lane::<0>(self.0),
            f32x4_extract_lane::<1>(self.0),
            f32x4_extract_lane::<2>(self.0),
            f32x4_extract_lane::<3>(self.0),
        )
    }
    pub fn dot(self, v2: Vec4) -> f32 {
        let m = f32x4_mul(self.0, v2.0);
        let x = f32x4_extract_lane::<0>(m);
        let y = f32x4_extract_lane::<1>(m);
        let z = f32x4_extract_lane::<2>(m);
        let w = f32x4_extract_lane::<3>(m);
        x + y + z + w
    }

    pub fn normalize(self) -> Vec4 {
        let mag = self.dot(self).sqrt();
        let fac = f32x4_splat(1.0 / mag);
        Vec4(f32x4_mul(self.0, fac))
    }
}

impl std::fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z, w) = (*self).extract();
        f.debug_struct("Vec4")
            .field("x", &x)
            .field("y", &y)
            .field("z", &z)
            .field("w", &w)
            .finish()
    }
}
impl From<(f32, f32, f32)> for Vec4 {
    fn from((x, y, z): (f32, f32, f32)) -> Vec4 {
        Vec4::new(x, y, z, 0.0)
    }
}
impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, f: f32) -> Vec4 {
        let s = f32x4_splat(f);
        Vec4(f32x4_mul(self.0, s))
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, v: Vec4) -> Vec4 {
        Vec4(f32x4_add(self.0, v.0))
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, vec: Vec4) -> Vec4 {
        (&self) * vec 
    }
}


impl Mul<Vec4> for &Mat4 {
    type Output = Vec4;

    fn mul(self, vec: Vec4) -> Vec4 {
        let f4 = f32x4_replace_lane::<3>(vec.0, 1.0);
        let v0 = f32x4_mul(f4, self.0[0]);
        let v1 = f32x4_mul(f4, self.0[1]);
        let v2 = f32x4_mul(f4, self.0[2]);
        // let v3 = f32x4_mul(f4, self.0[3]);
        Vec4(f32x4(Vec4(v0).agg(), Vec4(v1).agg(), Vec4(v2).agg(), 0.0))
    }
}

impl Neg for Mat4 {
    type Output = Mat4;
    fn neg(self) -> Self::Output {
        Mat4([
            f32x4_neg(self.0[0]),
            f32x4_neg(self.0[1]),
            f32x4_neg(self.0[2]),
            f32x4_neg(self.0[3]),
        ])
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Mat4;

    fn add(self, m2: Mat4) -> Mat4 {
        Mat4([
            f32x4_add(self.0[0], m2.0[0]),
            f32x4_add(self.0[1], m2.0[1]),
            f32x4_add(self.0[2], m2.0[2]),
            f32x4_add(self.0[3], m2.0[3]),
        ])
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, m2: Mat4) -> Mat4 {
        let m2t = m2.transpose();
        let m00 = f32x4_agg(f32x4_mul(self.0[0], m2t.0[0]));
        let m01 = f32x4_agg(f32x4_mul(self.0[0], m2t.0[1]));
        let m02 = f32x4_agg(f32x4_mul(self.0[0], m2t.0[2]));
        let m03 = f32x4_agg(f32x4_mul(self.0[0], m2t.0[3]));
        let m10 = f32x4_agg(f32x4_mul(self.0[1], m2t.0[0]));
        let m11 = f32x4_agg(f32x4_mul(self.0[1], m2t.0[1]));
        let m12 = f32x4_agg(f32x4_mul(self.0[1], m2t.0[2]));
        let m13 = f32x4_agg(f32x4_mul(self.0[1], m2t.0[3]));
        let m20 = f32x4_agg(f32x4_mul(self.0[2], m2t.0[0]));
        let m21 = f32x4_agg(f32x4_mul(self.0[2], m2t.0[1]));
        let m22 = f32x4_agg(f32x4_mul(self.0[2], m2t.0[2]));
        let m23 = f32x4_agg(f32x4_mul(self.0[2], m2t.0[3]));
        let m30 = f32x4_agg(f32x4_mul(self.0[3], m2t.0[0]));
        let m31 = f32x4_agg(f32x4_mul(self.0[3], m2t.0[1]));
        let m32 = f32x4_agg(f32x4_mul(self.0[3], m2t.0[2]));
        let m33 = f32x4_agg(f32x4_mul(self.0[3], m2t.0[3]));
        Mat4([
            f32x4(m00, m01, m02, m03),
            f32x4(m10, m11, m12, m13),
            f32x4(m20, m21, m22, m23),
            f32x4(m30, m31, m32, m33),
        ])
    }
}

// row-major
#[cfg(target_arch = "wasm32")]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Mat4(pub(crate) [v128; 4]);

impl Mat4 {
    pub const fn identity() -> Mat4 {
        Mat4([
            f32x4(1.0, 0.0, 0.0, 0.0),
            f32x4(0.0, 1.0, 0.0, 0.0),
            f32x4(0.0, 0.0, 1.0, 0.0),
            f32x4(0.0, 0.0, 0.0, 1.0),
        ])
    }
    pub fn translation(dx: f32, dy: f32, dz: f32) -> Mat4 {
        Mat4([
            f32x4(1.0, 0.0, 0.0, dx),
            f32x4(0.0, 1.0, 0.0, dy),
            f32x4(0.0, 0.0, 1.0, dz),
            f32x4(0.0, 0.0, 0.0, 1.0),
        ])
    }
    pub fn transpose(self) -> Mat4 {
        let m00 = f32x4_extract_lane::<0>(self.0[0]);
        let m01 = f32x4_extract_lane::<1>(self.0[0]);
        let m02 = f32x4_extract_lane::<2>(self.0[0]);
        let m03 = f32x4_extract_lane::<3>(self.0[0]);
        let m10 = f32x4_extract_lane::<0>(self.0[1]);
        let m11 = f32x4_extract_lane::<1>(self.0[1]);
        let m12 = f32x4_extract_lane::<2>(self.0[1]);
        let m13 = f32x4_extract_lane::<3>(self.0[1]);
        let m20 = f32x4_extract_lane::<0>(self.0[2]);
        let m21 = f32x4_extract_lane::<1>(self.0[2]);
        let m22 = f32x4_extract_lane::<2>(self.0[2]);
        let m23 = f32x4_extract_lane::<3>(self.0[2]);
        let m30 = f32x4_extract_lane::<0>(self.0[3]);
        let m31 = f32x4_extract_lane::<1>(self.0[3]);
        let m32 = f32x4_extract_lane::<2>(self.0[3]);
        let m33 = f32x4_extract_lane::<3>(self.0[3]);
        Mat4([
            f32x4(m00, m10, m20, m30),
            f32x4(m01, m11, m21, m31),
            f32x4(m02, m12, m22, m32),
            f32x4(m03, m13, m23, m33),
        ])
    }
    pub fn rotation(roll: f32, pitch: f32, yaw: f32) -> Mat4 {
        let sin_alpha = roll.sin();
        let cos_alpha = roll.cos();
        let sin_beta = pitch.sin();
        let cos_beta = pitch.cos();
        let sin_gamma = yaw.sin();
        let cos_gamma = yaw.cos();

        let m00 = cos_gamma * cos_beta;
        let m01 = cos_gamma * sin_beta * sin_alpha - sin_gamma * cos_alpha;
        let m02 = cos_gamma * sin_beta * cos_alpha + sin_gamma * sin_alpha;

        let m10 = sin_gamma * cos_beta;
        let m11 = sin_gamma * sin_beta * sin_alpha + cos_gamma * cos_alpha;
        let m12 = sin_gamma * sin_beta * cos_alpha - cos_gamma * sin_alpha;

        let m20 = -sin_beta;
        let m21 = cos_beta * sin_alpha;
        let m22 = cos_beta * cos_alpha;

        Mat4([
            f32x4(m00, m01, m02, 0.0),
            f32x4(m10, m11, m12, 0.0),
            f32x4(m20, m21, m22, 0.0),
            f32x4(0.0, 0.0, 0.0, 1.0),
        ])
    }
    #[rustfmt::skip]
    pub fn inverse(self) -> Option<Mat4> {
        let a:[[f32;4];4] = self.into();
        let det_m00 = a[1][1]*a[2][2]*a[3][3] + a[1][2]*a[2][3]*a[3][1] + a[1][3]*a[2][1]*a[3][2] - a[1][3]*a[2][2]*a[3][1] - a[1][2]*a[2][1]*a[3][3] - a[1][1]*a[2][3]*a[3][2];
        let det_m01 = a[1][0]*a[2][2]*a[3][3] + a[1][2]*a[2][3]*a[3][0] + a[1][3]*a[2][0]*a[3][2] - a[1][3]*a[2][2]*a[3][0] - a[1][2]*a[2][0]*a[3][3] - a[1][0]*a[2][3]*a[3][2];
        let det_m02 = a[1][0]*a[2][1]*a[3][3] + a[1][1]*a[2][3]*a[3][0] + a[1][3]*a[2][0]*a[3][1] - a[1][3]*a[2][1]*a[3][0] - a[1][1]*a[2][0]*a[3][3] - a[1][0]*a[2][3]*a[3][1];
        let det_m03 = a[1][0]*a[2][1]*a[3][2] + a[1][1]*a[2][2]*a[3][0] + a[1][2]*a[2][0]*a[3][1] - a[1][2]*a[2][1]*a[3][0] - a[1][1]*a[2][0]*a[3][2] - a[1][0]*a[2][2]*a[3][1];

        let det_m10 = a[0][1]*a[2][2]*a[3][3] + a[0][2]*a[2][3]*a[3][1] + a[0][3]*a[2][1]*a[3][2] - a[0][3]*a[2][2]*a[3][1] - a[0][2]*a[2][1]*a[3][3] - a[0][1]*a[2][3]*a[3][2];
        let det_m11 = a[0][0]*a[2][2]*a[3][3] + a[0][2]*a[2][3]*a[3][0] + a[0][3]*a[2][0]*a[3][2] - a[0][3]*a[2][2]*a[3][0] - a[0][2]*a[2][0]*a[3][3] - a[0][0]*a[2][3]*a[3][2];
        let det_m12 = a[0][0]*a[2][1]*a[3][3] + a[0][1]*a[2][3]*a[3][0] + a[0][3]*a[2][0]*a[3][1] - a[0][3]*a[2][1]*a[3][0] - a[0][1]*a[2][0]*a[3][3] - a[0][0]*a[2][3]*a[3][1];
        let det_m13 = a[0][0]*a[2][1]*a[3][2] + a[0][1]*a[2][2]*a[3][0] + a[0][2]*a[2][0]*a[3][1] - a[0][2]*a[2][1]*a[3][0] - a[0][1]*a[2][0]*a[3][2] - a[0][0]*a[2][2]*a[3][1];
        
        let det_m20 = a[0][1]*a[1][2]*a[3][3] + a[0][2]*a[1][3]*a[3][1] + a[0][3]*a[1][1]*a[3][2] - a[0][3]*a[1][2]*a[3][1] - a[0][2]*a[1][1]*a[3][3] - a[0][1]*a[1][3]*a[3][2];
        let det_m21 = a[0][0]*a[1][2]*a[3][3] + a[0][2]*a[1][3]*a[3][0] + a[0][3]*a[1][0]*a[3][2] - a[0][3]*a[1][2]*a[3][0] - a[0][2]*a[1][0]*a[3][3] - a[0][0]*a[1][3]*a[3][2];
        let det_m22 = a[0][0]*a[1][1]*a[3][3] + a[0][1]*a[1][3]*a[3][0] + a[0][3]*a[1][0]*a[3][1] - a[0][3]*a[1][1]*a[3][0] - a[0][1]*a[1][0]*a[3][3] - a[0][0]*a[1][3]*a[3][1];
        let det_m23 = a[0][0]*a[1][1]*a[3][2] + a[0][1]*a[1][2]*a[3][0] + a[0][2]*a[1][0]*a[3][1] - a[0][2]*a[1][1]*a[3][0] - a[0][1]*a[1][0]*a[3][2] - a[0][0]*a[1][2]*a[3][1];

        let det_m30 = a[0][1]*a[1][2]*a[2][3] + a[0][2]*a[1][3]*a[2][1] + a[0][3]*a[1][1]*a[2][2] - a[0][3]*a[1][2]*a[2][1] - a[0][2]*a[1][1]*a[2][3] - a[0][1]*a[1][3]*a[2][2];
        let det_m31 = a[0][0]*a[1][2]*a[2][3] + a[0][2]*a[1][3]*a[2][0] + a[0][3]*a[1][0]*a[2][2] - a[0][3]*a[1][2]*a[2][0] - a[0][2]*a[1][0]*a[2][3] - a[0][0]*a[1][3]*a[2][2];
        let det_m32 = a[0][0]*a[1][1]*a[2][3] + a[0][1]*a[1][3]*a[2][0] + a[0][3]*a[1][0]*a[2][1] - a[0][3]*a[1][1]*a[2][0] - a[0][1]*a[1][0]*a[2][3] - a[0][0]*a[1][3]*a[2][1];
        let det_m33 = a[0][0]*a[1][1]*a[2][2] + a[0][1]*a[1][2]*a[2][0] + a[0][2]*a[1][0]*a[2][1] - a[0][2]*a[1][1]*a[2][0] - a[0][1]*a[1][0]*a[2][2] - a[0][0]*a[1][2]*a[2][1];

        let det = a[0][0]*det_m00
                 -a[1][0]*det_m10
                 +a[2][0]*det_m20
                 -a[3][0]*det_m30;
        if det == 0.0 {
            return None;
        }
        println!("det:{det}");
        let a00 = det_m00 / det;
        let a01 = -det_m01 / det;
        let a02 = det_m02 / det;
        let a03 = -det_m03 / det;

        let a10 = -det_m10 / det;
        let a11 = det_m11 / det;
        let a12 = -det_m12 / det;
        let a13 = det_m13 / det;

        let a20 = det_m20 / det;
        let a21 = -det_m21 / det;
        let a22 = det_m22 / det;
        let a23 = -det_m23 / det;

        let a30 = -det_m30 / det;
        let a31 = det_m31 / det;
        let a32 = -det_m32 / det;
        let a33 = det_m33 / det;
        Some(Mat4([
            f32x4(a00,a01,a02,a03),
            f32x4(a10,a11,a12,a13),
            f32x4(a20,a21,a22,a23),
            f32x4(a30,a31,a32,a33),
        ]))
    }
}

impl std::fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for i in 0..4 {
            let row = self.0[i];
            let (x, y, z, w) = Vec4(row).extract();
            writeln!(f, "[ {:.5} {:.5} {:.5} {:.5} ]", x, y, z, w)?;
        }
        writeln!(f, "]")
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(a: [[f32; 4]; 4]) -> Mat4 {
        Mat4([
            f32x4(a[0][0], a[0][1], a[0][2], a[0][3]),
            f32x4(a[1][0], a[1][1], a[1][2], a[1][3]),
            f32x4(a[2][0], a[2][1], a[2][2], a[2][3]),
            f32x4(a[3][0], a[3][1], a[3][2], a[3][3]),
        ])
    }
}

impl From<Mat4> for [[f32; 4]; 4] {
    fn from(m: Mat4) -> [[f32; 4]; 4] {
        let m00 = f32x4_extract_lane::<0>(m.0[0]);
        let m01 = f32x4_extract_lane::<1>(m.0[0]);
        let m02 = f32x4_extract_lane::<2>(m.0[0]);
        let m03 = f32x4_extract_lane::<3>(m.0[0]);
        let m10 = f32x4_extract_lane::<0>(m.0[1]);
        let m11 = f32x4_extract_lane::<1>(m.0[1]);
        let m12 = f32x4_extract_lane::<2>(m.0[1]);
        let m13 = f32x4_extract_lane::<3>(m.0[1]);
        let m20 = f32x4_extract_lane::<0>(m.0[2]);
        let m21 = f32x4_extract_lane::<1>(m.0[2]);
        let m22 = f32x4_extract_lane::<2>(m.0[2]);
        let m23 = f32x4_extract_lane::<3>(m.0[2]);
        let m30 = f32x4_extract_lane::<0>(m.0[3]);
        let m31 = f32x4_extract_lane::<1>(m.0[3]);
        let m32 = f32x4_extract_lane::<2>(m.0[3]);
        let m33 = f32x4_extract_lane::<3>(m.0[3]);
        [
            [m00, m10, m20, m30],
            [m01, m11, m21, m31],
            [m02, m12, m22, m32],
            [m03, m13, m23, m33],
        ]
    }
}

#[cfg(test)]
mod test {
    use super::fallback::*;
    use super::*;
    use nalgebra::MatrixXx4;

    const TOLERANCE: f32 = 2e-3;

    fn matrices_approx_equal(a: Mat4, b: Mat4, tolerance: f32) -> bool {
        let a = a.0;
        let b = b.0;
        for i in 0..4 {
            for j in 0..4 {
                if (a[i][j] - b[i][j]).abs() > tolerance {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_identity_matrix() {
        let identity: Mat4 = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into();

        let result = identity.inverse().unwrap();
        assert!(matrices_approx_equal(result, identity, TOLERANCE));
    }

    #[test]
    fn test_non_invertible_matrix() {
        let non_invertible: Mat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 6.0, 8.0],
            [3.0, 6.0, 9.0, 12.0],
            [4.0, 8.0, 12.0, 16.0],
        ]
        .into();

        assert!(non_invertible.inverse().is_none());
    }

    #[test]
    fn test_invertible_matrices() {
        let (orig, inverse): (Mat4, Mat4) = (
            [
                [0.0, 0.0, -1.0, 2.0],
                [0.0, 1.0, 0.0, 0.0],
                [9.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into(),
            [
                [0.0, 0.0, 1.0 / 9.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0, 2.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into(),
        );
        let result = orig.inverse().unwrap();
        assert!(
            matrices_approx_equal(result, inverse, TOLERANCE),
            "{result:?} =/= {inverse:?}"
        );
        assert!(false);
    }

    fn g(rng: &mut impl rand::Rng) -> f32 {
        rng.gen_range(-20.0..20.0)
    }
    #[test]
    fn test_random_matrix() {
        for i in 0..100 {
            let mut rnd = rand::thread_rng();
            let r = &mut rnd;
            let m: Mat4 = [
                [g(r), g(r), g(r), g(r)],
                [g(r), g(r), g(r), g(r)],
                [g(r), g(r), g(r), g(r)],
                [g(r), g(r), g(r), g(r)],
            ]
            .into();
            let nmatrix = nalgebra::Matrix4::new(
                m.0[0][0], m.0[0][1], m.0[0][2], m.0[0][3], m.0[1][0], m.0[1][1], m.0[1][2],
                m.0[1][3], m.0[2][0], m.0[2][1], m.0[2][2], m.0[2][3], m.0[3][0], m.0[3][1],
                m.0[3][2], m.0[3][3],
            );
            println!("m {:?}", m);
            println!("ndet: {}", nmatrix.determinant());
            println!("ninvert {:?}", nmatrix.try_inverse());
            match m.inverse() {
                Some(inv) => {
                    println!("invert {:?}", inv);
                    println!(
                        "ninvert-ninvert {:?}",
                        nmatrix.try_inverse().unwrap().try_inverse()
                    );
                    let ii = inv.inverse().unwrap();
                    let residual = m + (-ii);
                    assert!(
                        matrices_approx_equal(m, ii, TOLERANCE),
                        "residual {residual:?} {i}"
                    )
                }
                None => {}
            }
        }
    }
}
