use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use crate::Surface;
pub(crate) const KERNEL_FACTOR: f32 = 1.8799712059732503;
pub(crate) const MAX_KERNEL_SIZE: usize = 512;

#[inline(always)]
pub(crate) fn alpha(pixel: &u32) -> u32 {
    ((pixel) >> 24) & 0xFF
}

#[inline(always)]
pub(crate) fn red(pixel: &u32) -> u32 {
    ((pixel) >> 16) & 0xFF
}

#[inline(always)]
pub(crate) fn green(pixel: &u32) -> u32 {
    ((pixel) >> 8) & 0xFF
}

#[inline(always)]
pub(crate) fn blue(pixel: &u32) -> u32 {
    pixel & 0xFF
}

#[inline(always)]
pub(crate) fn unpack_pixel(pixel: &u32, r: &mut u32, g: &mut u32, b: &mut u32, a: &mut u32) {
    *r = red(pixel);
    *g = green(pixel);
    *b = blue(pixel);
    *a = alpha(pixel);
}

#[inline(always)]
pub(crate) fn get_pixel_mut<'a>(surface: &'a mut Surface, x: usize, y: usize) -> &'a mut u32 {
    // let surface_stride = surface.stride;
    surface
        .pixels
        .get_mut(y * surface.stride as usize + x)
        .expect("Invalid index while getting mutable pointer to pixel")
}

#[inline(always)]
pub(crate) fn get_pixel<'a>(surface: &'a Surface, x: usize, y: usize) -> &'a u32 {
    // let surface_stride = surface.stride;
    surface
        .pixels
        .get(y * surface.stride + x)
        .expect("Invalid index while getting pointer to pixel")
}
#[inline(always)]
pub(crate) fn load_pixel(
    input: &mut Surface,
    x: usize,
    y: usize,
    r: &mut u32,
    g: &mut u32,
    b: &mut u32,
    a: &mut u32,
) {
    let pixel = get_pixel(input, x, y);
    unpack_pixel(pixel, r, g, b, a)
}
#[inline(always)]
pub(crate) fn init_load_pixel(
    input: &mut Surface,
    x: usize,
    y: usize,
    // r: &mut u32,
    // g: &mut u32,
    // b: &mut u32,
    // a: &mut u32,
) -> (u32, u32, u32, u32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut a = 0;
    load_pixel(input, x, y, &mut r, &mut g, &mut b, &mut a);
    (r, g, b, a)
    // load_pixel(input, x, y, r, g, b, a)
}

#[inline(always)]
pub(crate) fn pack_pixel(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (a << 24) | (r << 16) | (g << 8) | (b)
}
#[inline(always)]
pub(crate) fn store_pixel(
    output: &mut Surface,
    x: usize,
    y: usize,
    r: u32,
    g: u32,
    b: u32,
    a: u32,
) {
    *get_pixel_mut(output, x, y) = pack_pixel(r, g, b, a)
}

#[inline(always)]
pub(crate) fn premultiply_pixel(r: &mut u32, g: &mut u32, b: &mut u32, a: &mut u32) {
    *r = (*r * (*a + 1)) >> 8;
    *g = (*g * (*a + 1)) >> 8;
    *b = (*b * (*a + 1)) >> 8;
}

#[inline(always)]
pub(crate) fn unpremultiply_pixel(r: &mut u32, g: &mut u32, b: &mut u32, a: &mut u32) {
    if a != &0 {
        *r = (255 * *r) / *a;
        *g = (255 * *g) / *a;
        *b = (255 * *b) / *a;
    } else {
        *r = 0;
        *g = 0;
        *b = 0;
        *a = 0;
    }
}
#[inline(always)]
pub(crate) fn clamp(v: u32, lo: u32, hi: u32) -> u32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

#[inline(always)]
pub(crate) fn clamp_pixel(pixel: u32) -> u32 {
    clamp(pixel, 0, 255)
}

#[inline(always)]
pub(crate) fn overlap_surface3(a: &mut Surface, b: &mut Surface, c: &mut Surface) {
    let mut _width = a.width;
    let mut _height = a.height;
    if c.width < _width {
        _width = c.width;
    }
    if b.width < _width {
        _width = b.width;
    }
    if b.height < _height {
        _height = b.height;
    }
    if c.height < _height {
        _height = c.height;
    }
    a.width = _width;
    b.width = _width;
    c.width = _width;

    a.height = _height;
    b.height = _height;
    c.height = _height;
}

pub(crate) fn overlap_surface(a: &mut Surface, b: &mut Surface) {
    let mut _width = a.width;
    let mut _height = a.height;
    if b.width < _width {
        _width = b.width;
    }
    if b.height < _height {
        _height = b.height;
    }
    b.width = _width;
    a.width = b.width;
    b.height = _height;
    a.height = b.height;
}

pub(crate) fn deg2rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub(crate) fn srgb_to_linear_rgb(r: &mut u32, g: &mut u32, b: &mut u32) {
    *r = SRGB_TO_LINEAR_RGB_TABLE[*r as usize];
    *g = SRGB_TO_LINEAR_RGB_TABLE[*g as usize];
    *b = SRGB_TO_LINEAR_RGB_TABLE[*b as usize];
}

#[rustfmt::skip]
const SRGB_TO_LINEAR_RGB_TABLE: [u32; 256] = [
    0,   0,   0,   0,   0,   0,  0,    1,   1,   1,   1,   1,   1,   1,   1,   1,
    1,   1,   2,   2,   2,   2,  2,    2,   2,   2,   3,   3,   3,   3,   3,   3,
    4,   4,   4,   4,   4,   5,  5,    5,   5,   6,   6,   6,   6,   7,   7,   7,
    8,   8,   8,   8,   9,   9,  9,   10,  10,  10,  11,  11,  12,  12,  12,  13,
    13,  13,  14,  14,  15,  15,  16,  16,  17,  17,  17,  18,  18,  19,  19,  20,
    20,  21,  22,  22,  23,  23,  24,  24,  25,  25,  26,  27,  27,  28,  29,  29,
    30,  30,  31,  32,  32,  33,  34,  35,  35,  36,  37,  37,  38,  39,  40,  41,
    41,  42,  43,  44,  45,  45,  46,  47,  48,  49,  50,  51,  51,  52,  53,  54,
    55,  56,  57,  58,  59,  60,  61,  62,  63,  64,  65,  66,  67,  68,  69,  70,
    71,  72,  73,  74,  76,  77,  78,  79,  80,  81,  82,  84,  85,  86,  87,  88,
    90,  91,  92,  93,  95,  96,  97,  99, 100, 101, 103, 104, 105, 107, 108, 109,
    111, 112, 114, 115, 116, 118, 119, 121, 122, 124, 125, 127, 128, 130, 131, 133,
    134, 136, 138, 139, 141, 142, 144, 146, 147, 149, 151, 152, 154, 156, 157, 159,
    161, 163, 164, 166, 168, 170, 171, 173, 175, 177, 179, 181, 183, 184, 186, 188,
    190, 192, 194, 196, 198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218, 220,
    222, 224, 226, 229, 231, 233, 235, 237, 239, 242, 244, 246, 248, 250, 253, 255,
];

pub(crate) fn liner_rgb_to_srgb(r: &mut u32, g: &mut u32, b: &mut u32) {
    *r = LINEAR_RGB_TO_SRGB_TABLE[*r as usize];
    *g = LINEAR_RGB_TO_SRGB_TABLE[*g as usize];
    *b = LINEAR_RGB_TO_SRGB_TABLE[*b as usize];
}
#[rustfmt::skip]
const LINEAR_RGB_TO_SRGB_TABLE: [u32;256] = [
    0,  13,  22,  28,  34,  38,  42,  46,  50,  53,  56,  59,  61,  64,  66,  69,
    71,  73,  75,  77,  79,  81,  83,  85,  86,  88,  90,  92,  93,  95,  96,  98,
    99, 101, 102, 104, 105, 106, 108, 109, 110, 112, 113, 114, 115, 117, 118, 119,
    120, 121, 122, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136,
    137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 148, 149, 150, 151,
    152, 153, 154, 155, 155, 156, 157, 158, 159, 159, 160, 161, 162, 163, 163, 164,
    165, 166, 167, 167, 168, 169, 170, 170, 171, 172, 173, 173, 174, 175, 175, 176,
    177, 178, 178, 179, 180, 180, 181, 182, 182, 183, 184, 185, 185, 186, 187, 187,
    188, 189, 189, 190, 190, 191, 192, 192, 193, 194, 194, 195, 196, 196, 197, 197,
    198, 199, 199, 200, 200, 201, 202, 202, 203, 203, 204, 205, 205, 206, 206, 207,
    208, 208, 209, 209, 210, 210, 211, 212, 212, 213, 213, 214, 214, 215, 215, 216,
    216, 217, 218, 218, 219, 219, 220, 220, 221, 221, 222, 222, 223, 223, 224, 224,
    225, 226, 226, 227, 227, 228, 228, 229, 229, 230, 230, 231, 231, 232, 232, 233,
    233, 234, 234, 235, 235, 236, 236, 237, 237, 238, 238, 238, 239, 239, 240, 240,
    241, 241, 242, 242, 243, 243, 244, 244, 245, 245, 246, 246, 246, 247, 247, 248,
    248, 249, 249, 250, 250, 251, 251, 251, 252, 252, 253, 253, 254, 254, 255, 255,
];

pub(crate) fn blur_store_pixel(
    output: &mut Surface,
    x: usize,
    y: usize,
    r: u32,
    g: u32,
    b: u32,
    a: u32,
    k: usize,
) {
    let k = k as u32;
    store_pixel(output, x, y, r / k, g / k, b / k, a / k);
}

pub(crate) fn calc_kernel_size(std_deviation: f32) -> i32 {
    f32::floor(std_deviation * KERNEL_FACTOR + 0.5) as i32
}

pub(crate) fn box_blur(
    input: Rc<RefCell<&mut Surface>>,
    output: Rc<RefCell<&mut Surface>>,
    intermediate: &mut [u32],
    mut kernel_width: usize,
    mut kernel_height: usize,
) {
    let (mut offset, mut pixel);
    let (mut sum_r, mut sum_g, mut sum_b, mut sum_a);
    let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);

    if kernel_width > 0 {
        kernel_width = kernel_width.min(output.borrow().width);
        for y in 0..output.borrow().height {
            sum_r = 0;
            sum_g = 0;
            sum_b = 0;
            sum_a = 0;

            for x in 0..kernel_width {
                let index = x % kernel_width;

                intermediate[index] = *get_pixel(*input.borrow_mut(), x, y);
                pixel = intermediate[index];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                offset = x - kernel_width / 2;
                // if offset >= 0 && offset < output.borrow().width {
                if offset < output.borrow().width {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        offset,
                        y,
                        sum_r,
                        sum_g,
                        sum_b,
                        sum_a,
                        kernel_width,
                    );
                }
            }

            for x in kernel_width..output.borrow().width {
                let index = x % kernel_width;
                pixel = intermediate[index];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;
                intermediate[index] = *get_pixel(*input.borrow(), x, y);
                pixel = intermediate[index];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                offset = x - kernel_width / 2;
                blur_store_pixel(
                    *output.borrow_mut(),
                    offset,
                    y,
                    sum_r,
                    sum_g,
                    sum_b,
                    sum_a,
                    kernel_width,
                );
            }

            for x in output.borrow().width..(output.borrow().width + kernel_width) {
                let index = x % kernel_width;
                pixel = intermediate[index];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;

                offset = x - kernel_width / 2;
                // if offset >= 0 && offset < output.borrow().width {
                if offset < output.borrow().width {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        offset,
                        y,
                        sum_r,
                        sum_g,
                        sum_b,
                        sum_a,
                        kernel_width,
                    );
                }
            }
        }
    }

    if kernel_height > 0 {
        kernel_height = kernel_height.min(output.borrow().height);
        for x in 0..output.borrow().width {
            sum_r = 0;
            sum_g = 0;
            sum_b = 0;
            sum_a = 0;

            for y in 0..kernel_height {
                let index = y % kernel_height;
                intermediate[index] = *get_pixel(*input.borrow(), x, y);
                pixel = intermediate[index];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                offset = y - kernel_height / 2;
                // if offset >= 0 && offset < output.borrow().height {
                if offset < output.borrow().height {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        x,
                        offset,
                        sum_r,
                        sum_g,
                        sum_b,
                        sum_a,
                        kernel_height,
                    );
                }
            }

            for y in kernel_height..output.borrow().height {
                let index = y % kernel_height;
                pixel = intermediate[index];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;
                intermediate[index] = *get_pixel(*input.borrow(), x, y);
                pixel = intermediate[index];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                offset = y - kernel_height / 2;
                blur_store_pixel(
                    *output.borrow_mut(),
                    x,
                    offset,
                    sum_r,
                    sum_g,
                    sum_b,
                    sum_a,
                    kernel_height,
                );
            }

            for y in output.borrow().height..(output.borrow().height + kernel_height) {
                pixel = intermediate[y % kernel_height];
                unpack_pixel(&pixel, &mut r, &mut g, &mut b, &mut a);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;

                offset = y - kernel_height / 2;
                // if offset >= 0 && offset < output.borrow().height {
                if offset < output.borrow().height {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        x,
                        offset,
                        sum_r,
                        sum_g,
                        sum_b,
                        sum_a,
                        kernel_height,
                    );
                }
            }
        }
    }
}
