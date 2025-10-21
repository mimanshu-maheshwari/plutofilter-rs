use std::f32::consts::PI;

use crate::Surface;

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
pub(crate) fn get_pixel<'a>(surface: &'a mut Surface, x: u16, y: u16) -> &'a mut u32 {
    // let surface_stride = surface.stride;
    surface
        .pixels
        .get_mut(y as usize * surface.stride as usize + x as usize)
        .unwrap()
}
#[inline(always)]
pub(crate) fn load_pixel(
    input: &mut Surface,
    x: u16,
    y: u16,
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
    x: u16,
    y: u16,
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
pub(crate) fn pack_pixel(r: &u32, g: &u32, b: &u32, a: &u32) -> u32 {
    (a << 24) | (r << 16) | (g << 8) | (b)
}
#[inline(always)]
pub(crate) fn store_pixel(
    output: &mut Surface,
    x: u16,
    y: u16,
    r: &mut u32,
    g: &mut u32,
    b: &mut u32,
    a: &mut u32,
) {
    *get_pixel(output, x, y) = pack_pixel(r, g, b, a)
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

#[rustfmt::skip]
const SRGB_TO_LINEAR_RGB_TABLE: [u8; 256] = [
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
