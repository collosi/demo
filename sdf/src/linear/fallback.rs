#![cfg(not(target_arch = "wasm32"))]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vec4(pub(crate) [f32; 4]);

pub const fn f32x4(x: f32, y: f32, z: f32, w: f32) -> [f32; 4] {
    [x, y, z, w]
}
pub fn f32x4_extract_lane<const N: usize>(a: [f32; 4]) -> f32 {
    a[N]
}
pub fn f32x4_replace_lane<const N: usize>(a: [f32; 4], v: f32) -> [f32; 4] {
    let mut b = a;
    b[N] = v;
    b
}
pub fn f32x4_neg(a:[f32;4]) -> [f32; 4] {
    [-a[0], -a[1], -a[2], -a[3]]
}
pub fn f32x4_splat(f: f32) -> [f32; 4] {
        [f, f, f, f]
}
pub fn f32x4_mul(a1: [f32; 4], a2: [f32; 4]) -> [f32; 4] {
    [a1[0] * a2[0], a1[1] * a2[1], a1[2] * a2[2], a1[3] * a2[3]]
}
pub fn f32x4_add(a1: [f32; 4], a2: [f32; 4]) -> [f32; 4] {
    [a1[0] + a2[0], a1[1] + a2[1], a1[2] + a2[2], a1[3] + a2[3]]
}
pub fn f32x4_agg(f: [f32; 4]) -> f32 {
    let x = f32x4_extract_lane::<0>(f);
    let y = f32x4_extract_lane::<1>(f);
    let z = f32x4_extract_lane::<2>(f);
    let w = f32x4_extract_lane::<3>(f);
    x + y + z + w
}
// row-major
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Mat4(pub(crate) [[f32; 4]; 4]);
