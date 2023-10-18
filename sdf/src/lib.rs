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
    for x in 0..(width as usize) {
        for y in 0..(height as usize) {
            match (((x / 10) & 1) == 1, ((y / 10) & 1) == 1) {
                (false, false) => set_px(x, y, width as usize, 255, 255, 255, 255),
                (true, false) => set_px(x, y, width as usize, 0, 0, 255, 255),
                (false, true) => set_px(x, y, width as usize, 0, 0, 255, 255),
                (true, true) => set_px(x, y, width as usize, 255, 255, 255, 255),
            }
        }
    }
    unsafe { PIX_BUF.as_ptr() as i32 }
}
