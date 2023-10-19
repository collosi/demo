extern "C" {
    #[allow(unused)]
    fn output(str: *const std::ffi::c_char);
}

#[allow(unused)]
macro_rules! outputln {
        () => {
                output(b"".as_ptr() as *const std::ffi::c_char);
            };
        ($($arg:tt)*) => {{
                let s = format!($($arg)*);
                let cs = std::ffi::CString::new(s).unwrap_or_default();
                unsafe {output(cs.as_ptr() as *const std::ffi::c_char);}
            }};
    }

#[allow(unused_variables)]
#[no_mangle]
fn set_dimensions(
    dpi: i32,
    min_width: i32,
    min_height: i32,
    preferred_width: i32,
    preferred_height: i32,
    max_width: i32,
    max_height: i32,
) -> (i32, i32) {
    let (width, height) =
        if preferred_width < STATIC_WIDTH as i32 && preferred_height < STATIC_HEIGHT as i32 {
            (preferred_width, preferred_height)
        } else {
            (STATIC_WIDTH as i32, STATIC_HEIGHT as i32)
        };
    outputln!("selected {width} {height}");
    (width, height)
}

const STATIC_WIDTH: usize = 1024;
const STATIC_HEIGHT: usize = 768;
static mut PIX_BUF: [[u8; 4]; STATIC_WIDTH * STATIC_HEIGHT] =
    [[0; 4]; STATIC_WIDTH * STATIC_HEIGHT];

pub fn set_px(x: usize, y: usize, width: usize, r: u8, g: u8, b: u8, a: u8) {
    unsafe {
        PIX_BUF[y * width + x] = [r, g, b, a];
    }
}

fn dot(v1: [f32; 3], v2: [f32; 3]) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

fn mul(v1: [f32; 3], f: f32) -> [f32; 3] {
    [v1[0] * f, v1[1] * f, v1[2] * f]
}

fn add(v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

fn normalize(v1: [f32; 3]) -> [f32; 3] {
    let mag = dot(v1, v1).sqrt();
    if mag == 0.0 {
        v1
    } else {
        mul(v1, 1.0 / mag)
    }
}

const EPSILON: f32 = 0.0001;
const MAX_MARCHING_STEPS: usize = 255;
const MIN_DIST: f32 = 0.0;
const MAX_DIST: f32 = 100.0;

fn shortest_distance_to_surface(eye: [f32; 3], direction: [f32; 3], start: f32, end: f32) -> f32 {
    let mut depth = start;
    for _ in 0..MAX_MARCHING_STEPS {
        let view_ray = add(eye, mul(direction, depth));
        let dist = sdf_sphere(view_ray[0], view_ray[1], view_ray[2]);
        // outputln!("{depth} {view_ray:?} {dist}");
        if dist < EPSILON {
            return depth;
        }
        depth += dist;
        if depth >= end {
            return end;
        }
    }
    return end;
}

/**
 * Return the normalized direction to march in from the eye point for a single pixel.
 *
 * fieldOfView: vertical field of view in degrees
 * size: resolution of the output image
 * fragCoord: the x,y coordinate of the pixel in the output image
 */
fn ray_direction(field_of_view: f32, width: f32, height: f32, x: f32, y: f32) -> [f32; 3] {
    let z = height / (field_of_view.to_radians() / 2.0).tan();
    normalize([x - width / 2.0, y - height / 2.0, -z])
}

#[allow(unused_variables)]
#[no_mangle]
pub fn render(time: i32, width: i32, height: i32) -> i32 {
    // for x in 0..(width as usize) {
    //     for y in 0..(height as usize) {
    //         match (((x / 10) & 1) == 1, ((y / 10) & 1) == 1) {
    //             (false, false) => set_px(x, y, width as usize, 255, 255, 255, 255),
    //             (true, false) => set_px(x, y, width as usize, 100, 100, 255, 255),
    //             (false, true) => set_px(x, y, width as usize, 100, 100, 255, 255),
    //             (true, true) => set_px(x, y, width as usize, 255, 255, 255, 255),
    //         }
    //     }
    //}
    //     outputln!("width {width} height {height}")
    // for x in 0..(width as usize) {
    //     for y in 0..(height as usize) {
    //         if x > 50 {
    //             set_px(x, y, width as usize, 255, 255, 255, 255);
    //         }else{
    //             set_px(x, y, width as usize, 100, 100, 255, 255);
    //         }
    //     }
    // }

    let light_direction = [1.0, 1.0, -1.0]; // Top-right-front light
    let eye = [0.0, 0.0, 5.0];
    for i in 0..width as usize {
        for j in 0..height as usize {
            let dir = ray_direction(45.0, width as f32, height as f32, i as f32, j as f32);
            let dist = shortest_distance_to_surface(eye, dir, MIN_DIST, MAX_DIST);

            if dist > MAX_DIST - EPSILON {
                set_px(i, j, width as usize, 0, 0, 0, 255);
            } else {
                let p = add(eye, mul(dir, dist));
                let normal = gradient(p[0], p[1], p[2]);
                let intensity = (dot(p, normal).max(0.0) * 255.0) as u8;
                // outputln!("{p:?} {normal:?} {intensity}");
                set_px(i, j, width as usize, intensity, 0, 0, 255);
            }
        }
    }

    unsafe { PIX_BUF.as_ptr() as i32 }
}

const CUBE_SIZE: f32 = 0.5;
const SPHERE_RADIUS: f32 = 1.0;

fn sdf_cube(x: f32, y: f32, z: f32) -> f32 {
    let dx = x.abs() - CUBE_SIZE;
    let dy = y.abs() - CUBE_SIZE;
    let dz = z.abs() - CUBE_SIZE;
    let outside = dx.max(dy.max(dz));
    let inside = dx.min(0.0).max(dy.min(0.0).max(dz.min(0.0)));
    outside + inside
}

fn sdf_sphere(x: f32, y: f32, z: f32) -> f32 {
    let len = (x * x + y * y + z * z).sqrt();
    len - SPHERE_RADIUS
}

fn sdf_cube_minus_sphere(x: f32, y: f32, z: f32) -> f32 {
    let cube_sdf = sdf_cube(x, y, z);
    let sphere_sdf = sdf_sphere(x, y, z);
    cube_sdf.max(-sphere_sdf)
}

fn gradient(x: f32, y: f32, z: f32) -> [f32; 3] {
    const EPS: f32 = 0.001;
    let df = |x, y, z| sdf_sphere(x, y, z);
    // let df = |x, y, z| sdf_cube_minus_sphere(x, y, z);
    let dx = df(x + EPS, y, z) - df(x-EPS, y, z);
    let dy = df(x, y + EPS, z) - df(x, y-EPS, z);
    let dz = df(x, y, z + EPS) - df(x, y, z-EPS);
    normalize([dx,dy,dz])
}
