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
    if preferred_width < STATIC_WIDTH as i32 && preferred_height < STATIC_HEIGHT as i32 {
        (preferred_width, preferred_height)
    } else {
        (STATIC_WIDTH as i32, STATIC_HEIGHT as i32)
    }
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
    // }

    let light_direction = [1.0, 1.0, -1.0]; // Top-right-front light

    for i in 0..width as usize {
        for j in 0..height as usize {
            let x = (i as f32) / width as f32 - 0.5;
            let y = (j as f32) / height as f32 - 0.5;
            let z = 0.0;

            let normal = gradient(x, y, z);
            let dot_product = normal[0] * light_direction[0]
                + normal[1] * light_direction[1]
                + normal[2] * light_direction[2];

            let intensity = (dot_product.max(0.0) * 255.0) as u8; // Clamp to [0, 255]
            set_px(i, j, width as usize, intensity, 55, 55, 255);
            outputln!("{i}, {j} {width} {intensity}");
        }
        panic!();
    }

    unsafe { PIX_BUF.as_ptr() as i32 }
}

const CUBE_SIZE: f32 = 0.5;
const SPHERE_RADIUS: f32 = 0.3;

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
    let dx = df(x + EPS, y, z) - df(x, y, z);
    let dy = df(x, y + EPS, z) - df(x, y, z);
    let dz = df(x, y, z + EPS) - df(x, y, z);
    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    [dx / len, dy / len, dz / len]
}
