#[cfg(test)]
unsafe fn output(str: *const std::ffi::c_char) {
    libc::puts(str);
}

#[cfg(not(test))]
extern "C" {
    #[allow(unused)]
    fn output(str: *const std::ffi::c_char);
}

mod linear;
use linear::*;

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

const DIMENSIONS :[[i32;2];3] = [[320, 240],[640,480],[0,0]];

#[allow(unused_variables)]
#[no_mangle]
fn get_dimensions(dpi: i32) -> i32{
    unsafe {
        DIMENSIONS.as_ptr() as i32
    }
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

const EPSILON: f32 = 0.0001;
const MAX_MARCHING_STEPS: usize = 255;
const MIN_DIST: f32 = 0.0;
const MAX_DIST: f32 = 100.0;

fn shortest_distance_to_surface(
    df: impl Fn(Vec3) -> f32,
    eye: Vec3,
    direction: Vec3,
    start: f32,
    end: f32,
) -> f32 {
    let mut depth = start;
    for _ in 0..MAX_MARCHING_STEPS {
        let view_ray = eye + (direction * depth);
        let dist = df(view_ray);
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
fn ray_direction(field_of_view: f32, width: f32, height: f32, x: f32, y: f32) -> Vec3 {
    let z = height / (field_of_view.to_radians() / 2.0).tan();
    Vec3::new(x - width / 2.0, y - height / 2.0, z).normalize()
}

fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    let h = h as f32 / 255.0 * 360.0; // Convert hue to [0, 360]
    let s = s as f32 / 255.0; // Convert saturation to [0, 1]
    let v = v as f32 / 255.0; // Convert value to [0, 1]

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = 
        if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

    let r = ((r_prime + m) * 255.0).round() as u8;
    let g = ((g_prime + m) * 255.0).round() as u8;
    let b = ((b_prime + m) * 255.0).round() as u8;

    (r, g, b)
}

#[allow(unused_variables)]
#[no_mangle]
pub fn render(time: f64, width: i32, height: i32) -> i32 {
    let t = time / 5000.0;
    let rotation = Mat4::rotation(t as f32, t as f32 / 2.0, t as f32 / 3.0);
    let eye: Vec3 = (0.0, 0.0, -5.0).into();
    let df = |v| {
        let mv = rotation * v;
        sdf_cube_minus_sphere(mv)
    };
    for i in 0..width as usize {
        for j in 0..height as usize {
            let dir = ray_direction(45.0, width as f32, height as f32, i as f32, j as f32);
            let dist = shortest_distance_to_surface(df, eye, dir, MIN_DIST, MAX_DIST);

            if dist > MAX_DIST - EPSILON {
                set_px(i, j, width as usize, 0, 0, 0, 255);
            } else {
                let p = eye + (dir * dist);
                let normal = gradient(df, p);
                let intensity = ((-dir.dot(normal)).max(0.0) * 255.0) as u8;
                let (r, g, b) = hsv_to_rgb((t*10.0 % 255.0) as u8, intensity, 100  );
                set_px(i, j, width as usize, r,g,b,255);
            }
        }
    }

    unsafe { PIX_BUF.as_ptr() as i32 }
}

const CUBE_SIZE: f32 = 0.5;
const SPHERE_RADIUS: f32 = 0.6;

fn sdf_cube(v: Vec3) -> f32 {
    let dx = v.x.abs() - CUBE_SIZE;
    let dy = v.y.abs() - CUBE_SIZE;
    let dz = v.z.abs() - CUBE_SIZE;
    let outside = dx.max(dy.max(dz));
    let inside = dx.min(0.0).max(dy.min(0.0).max(dz.min(0.0)));
    outside + inside
}

fn sdf_sphere(v: Vec3) -> f32 {
    let len = v.dot(v).sqrt();
    len - SPHERE_RADIUS
}

fn sdf_cube_minus_sphere(v: Vec3) -> f32 {
    let cube_sdf = sdf_cube(v);
    let sphere_sdf = sdf_sphere(v);
    cube_sdf.max(-sphere_sdf)
}

fn gradient(df: impl Fn(Vec3) -> f32, v: Vec3) -> Vec3 {
    const EPS: f32 = 0.001;
    let dx = df((v.x + EPS, v.y, v.z).into()) - df((v.x - EPS, v.y, v.z).into());
    let dy = df((v.x, v.y + EPS, v.z).into()) - df((v.x, v.y - EPS, v.z).into());
    let dz = df((v.x, v.y, v.z + EPS).into()) - df((v.x, v.y, v.z - EPS).into());
    Vec3::new(dx, dy, dz).normalize()
}

#[cfg(test)]
mod test {
    use super::*;
    // #[test]
    // fn test_shortest_distance() {
    //     let eye = [0.0, 0.0, -5.0];
    //     for i in 0..50 {
    //         let dir = ray_direction(45.0, 2.0, 2.0, 1.0 + (0.1 * i as f32), 1.0);
    //         let dist = shortest_distance_to_surface(eye, dbg!(dir), MIN_DIST, MAX_DIST);
    //         if dist > 10.0 {
    //             panic!();
    //         }
    //         let p = add(eye, mul(dir, dbg!(dist)));
    //         let normal = gradient(p[0], p[1], p[2]);
    //         let intensity = (dbg!(dot(dbg!(dir), dbg!(normal))).max(0.0) * 255.0) as u8;
    //     }
    //     panic!("");
    // }
}
