use crate::{ColorChannel, Surface};
use std::{cell::RefCell, f32::consts::PI, rc::Rc};

#[allow(clippy::excessive_precision)]
const KERNEL_FACTOR: f32 = 1.8799712059732503;

/// Helper function to find file in `res` folder in root directory
pub fn get_resource_path(dirs: &[&str], filename: &str) -> std::path::PathBuf {
    let mut manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir = manifest_dir.join("res");
    for &dir in dirs {
        manifest_dir = manifest_dir.join(dir);
    }
    if !manifest_dir.exists() {
        let _ = std::fs::create_dir_all(&manifest_dir).map_err(|err| {
            eprint!("error while create_dir_all: {err}");
        });
    }

    manifest_dir.join(filename)
}

#[inline(always)]
pub(crate) fn alpha(pixel: &u32, channel: ColorChannel) -> u32 {
    unpack_pixel(pixel, channel)[3]
}

/// return r,g,b,a from a u32
#[inline(always)]
pub(crate) fn unpack_pixel(pixel: &u32, channel: ColorChannel) -> [u32; 4] {
    match channel {
        ColorChannel::ARGB32 => {
            let [a, r, g, b] = pixel.to_le_bytes();
            [r as u32, g as u32, b as u32, a as u32]
        }
        ColorChannel::RGBA32 => {
            let [r, g, b, a] = pixel.to_le_bytes();
            [r as u32, g as u32, b as u32, a as u32]
        }
    }
}

#[inline(always)]
pub(crate) fn get_pixel_mut<'a>(surface: &'a mut Surface, x: u32, y: u32) -> &'a mut u32 {
    // let surface_stride = surface.stride;
    &mut surface.pixels[(y * surface.stride + x) as usize]
    // .get_mut(y * surface.stride + x)
    // .expect("Invalid index while getting mutable reference to pixel")
}

#[inline(always)]
pub(crate) fn get_pixel<'a>(surface: &'a Surface, x: u32, y: u32) -> &'a u32 {
    // let surface_stride = surface.stride;
    &surface.pixels[(y * surface.stride + x) as usize]
    // .get(y * surface.stride + x)
    // .expect("Invalid index while getting reference to pixel")
}

#[inline(always)]
pub(crate) fn load_pixel(input: &mut Surface, x: u32, y: u32, channel: ColorChannel) -> [u32; 4] {
    let pixel = get_pixel(input, x, y);
    unpack_pixel(pixel, channel)
}

#[inline(always)]
pub(crate) fn init_load_pixel(
    input: &mut Surface,
    x: u32,
    y: u32,
    channel: ColorChannel,
) -> [u32; 4] {
    load_pixel(input, x, y, channel)
}
#[inline(always)]
pub(crate) fn pack_pixel(r: u32, g: u32, b: u32, a: u32) -> u32 {
    u32::from_le_bytes([r as u8, g as u8, b as u8, a as u8])
    // (r << 24) | (g << 16) | (b << 8) | (a)
}

// #[inline(always)]
// pub(crate) fn pack_pixel(r: u32, g: u32, b: u32, a: u32) -> u32 {
//     (a << 24) | (r << 16) | (g << 8) | (b)
// }
#[inline(always)]
pub(crate) fn store_pixel(output: &mut Surface, x: u32, y: u32, r: u32, g: u32, b: u32, a: u32) {
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
    if *a != 0 {
        *r = (255 * *r) / *a;
        *g = (255 * *g) / *a;
        *b = (255 * *b) / *a;
    } else {
        *r = 0;
        *g = 0;
        *b = 0;
    }
}

// #[inline(always)]
// pub(crate) fn clamp(v: u32, lo: u32, hi: u32) -> u32 {
//     if v < lo {
//         lo
//     } else if v > hi {
//         hi
//     } else {
//         v
//     }
// }

#[inline(always)]
pub(crate) fn clamp_pixel(pixel: u32) -> u32 {
    pixel.clamp(0, 255) as u8 as u32
}

#[inline(always)]
pub(crate) fn overlap_surface3(a: &mut Surface, b: &mut Surface, c: &mut Surface) {
    let mut _width = a.width;
    let mut _height = a.height;
    if b.width < _width {
        _width = b.width;
    }
    if c.width < _width {
        _width = c.width;
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
    (x, y): (u32, u32),
    (r, g, b, a): (u32, u32, u32, u32),
    k: u32,
) {
    store_pixel(output, x, y, r / k, g / k, b / k, a / k);
}

pub(crate) fn calc_kernel_size(std_deviation: f32) -> u32 {
    f32::floor(std_deviation * KERNEL_FACTOR + 0.5) as u32
}

pub(crate) fn box_blur(
    input: Rc<RefCell<&mut Surface>>,
    output: Rc<RefCell<&mut Surface>>,
    intermediate: &mut [u32],
    mut kernel_width: u32,
    mut kernel_height: u32,
) {
    let (mut offset, mut pixel);
    let (mut sum_r, mut sum_g, mut sum_b, mut sum_a);

    let channel = input.borrow().channel;
    let output_width = output.borrow().width;
    let output_height = output.borrow().height;
    if kernel_width > 0 {
        kernel_width = kernel_width.min(output_width);
        for y in 0..output_height {
            sum_r = 0;
            sum_g = 0;
            sum_b = 0;
            sum_a = 0;

            for x in 0..kernel_width {
                let index = (x % kernel_width) as usize;

                intermediate[index] = *get_pixel(*input.borrow_mut(), x, y);
                pixel = intermediate[index];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                // offset = x - kernel_width / 2;
                offset = x.wrapping_sub(kernel_width.wrapping_div(2));
                // if offset >= 0 && offset < output.borrow().width {
                if offset < output_width {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        (offset, y),
                        (sum_r, sum_g, sum_b, sum_a),
                        kernel_width,
                    );
                }
            }

            for x in kernel_width..output_width {
                let index = (x % kernel_width) as usize;
                pixel = intermediate[index];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;
                intermediate[index] = *get_pixel(*input.borrow(), x, y);
                pixel = intermediate[index];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                offset = x - kernel_width / 2;
                blur_store_pixel(
                    *output.borrow_mut(),
                    (offset, y),
                    (sum_r, sum_g, sum_b, sum_a),
                    kernel_width,
                );
            }

            for x in output_width..(output_width + kernel_width) {
                let index = (x % kernel_width) as usize;
                pixel = intermediate[index];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;

                offset = x - kernel_width / 2;
                // if offset >= 0 && offset < output.borrow().width {
                if offset < output_width {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        (offset, y),
                        (sum_r, sum_g, sum_b, sum_a),
                        kernel_width,
                    );
                }
            }
        }
    }

    if kernel_height > 0 {
        kernel_height = kernel_height.min(output_height);
        for x in 0..output_width {
            sum_r = 0;
            sum_g = 0;
            sum_b = 0;
            sum_a = 0;

            for y in 0..kernel_height {
                let index = (y % kernel_height) as usize;
                intermediate[index] = *get_pixel(*input.borrow(), x, y);
                pixel = intermediate[index];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                // offset = y - kernel_height / 2;
                offset = y.wrapping_sub(kernel_height.wrapping_div(2));
                // if offset >= 0 && offset < output.borrow().height {
                if offset < output_height {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        (x, offset),
                        (sum_r, sum_g, sum_b, sum_a),
                        kernel_height,
                    );
                }
            }

            for y in kernel_height..output_height {
                let index = (y % kernel_height) as usize;
                pixel = intermediate[index];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;
                intermediate[index] = *get_pixel(*input.borrow(), x, y);
                pixel = intermediate[index];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;

                offset = y - kernel_height / 2;
                blur_store_pixel(
                    *output.borrow_mut(),
                    (x, offset),
                    (sum_r, sum_g, sum_b, sum_a),
                    kernel_height,
                );
            }

            for y in output_height..(output_height + kernel_height) {
                pixel = intermediate[(y % kernel_height) as usize];
                let [r, g, b, a] = unpack_pixel(&pixel, channel);

                sum_r -= r;
                sum_g -= g;
                sum_b -= b;
                sum_a -= a;

                offset = y - kernel_height / 2;
                // if offset >= 0 && offset < output.borrow().height {
                if offset < output_height {
                    blur_store_pixel(
                        *output.borrow_mut(),
                        (x, offset),
                        (sum_r, sum_g, sum_b, sum_a),
                        kernel_height,
                    );
                }
            }
        }
    }
}

#[inline(always)]
pub(crate) fn clamp_and_store_pixel(
    output: &mut Surface,
    x: u32,
    y: u32,
    r: u32,
    g: u32,
    b: u32,
    a: u32,
) {
    let r = clamp_pixel(r);
    let g = clamp_pixel(g);
    let b = clamp_pixel(b);
    let a = clamp_pixel(a);
    store_pixel(output, x, y, r, g, b, a);
}

#[inline(always)]
fn div255(x: u32) -> u32 {
    (x + (x >> 8) + 0x80) >> 8
}

pub fn blend_normal_op(s: u32, d: u32, sa: u32, _da: u32) -> u32 {
    s + div255(d * (255 - sa))
}

pub(crate) fn blend_normal(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_normal_op(sr, dr, sa, da);
            let g = blend_normal_op(sg, dg, sa, da);
            let b = blend_normal_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_multiply_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    div255(s * d + s * (255 - da) + d * (255 - sa))
}

pub(crate) fn blend_multiply(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_multiply_op(sr, dr, sa, da);
            let g = blend_multiply_op(sg, dg, sa, da);
            let b = blend_multiply_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_screen_op(s: u32, d: u32, _sa: u32, _da: u32) -> u32 {
    s + d - div255(s * d)
}

pub(crate) fn blend_screen(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_screen_op(sr, dr, sa, da);
            let g = blend_screen_op(sg, dg, sa, da);
            let b = blend_screen_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_overlay_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    let temp = s * (255 - da) + d * (255 - sa);
    if 2 * d <= da {
        div255(2 * s * d + temp)
    } else {
        div255(sa * da - 2 * (da - d) * (sa - s) + temp)
    }
}

pub(crate) fn blend_overlay(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_overlay_op(sr, dr, sa, da);
            let g = blend_overlay_op(sg, dg, sa, da);
            let b = blend_overlay_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_darken_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    let sda = s * da;
    let dsa = d * sa;
    if sda < dsa {
        s + d - div255(dsa)
    } else {
        d + s - div255(sda)
    }
}
pub(crate) fn blend_darken(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_darken_op(sr, dr, sa, da);
            let g = blend_darken_op(sg, dg, sa, da);
            let b = blend_darken_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_lighten_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    let sda = s * da;
    let dsa = d * sa;
    if sda > dsa {
        s + d - div255(dsa)
    } else {
        d + s - div255(sda)
    }
}

pub(crate) fn blend_lighten(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_lighten_op(sr, dr, sa, da);
            let g = blend_lighten_op(sg, dg, sa, da);
            let b = blend_lighten_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_color_dodge_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    if d == 0 {
        div255(s * (255 - da))
    } else if s == sa || da * (sa - s) < d * sa {
        div255(sa * da + s * (255 - da) + d * (255 - sa))
    } else {
        div255(sa * ((d * sa) / (sa - s)) + s * (255 - da) + d * (255 - sa))
    }
}

pub(crate) fn blend_color_dodge(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_color_dodge_op(sr, dr, sa, da);
            let g = blend_color_dodge_op(sg, dg, sa, da);
            let b = blend_color_dodge_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_color_burn_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    if d == da {
        div255(sa * da + s * (255 - da) + d * (255 - sa))
    } else if s == 0 {
        div255(d * (255 - sa))
    } else if da * s < (da - d) * sa {
        div255(s * (255 - da) + d * (255 - sa))
    } else {
        div255(sa * (da - ((da - d) * sa / s)) + s * (255 - da) + d * (255 - sa))
    }
}
pub(crate) fn blend_color_burn(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_color_burn_op(sr, dr, sa, da);
            let g = blend_color_burn_op(sg, dg, sa, da);
            let b = blend_color_burn_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_hard_light_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    let tmp = s * (255 - da) + d * (255 - sa);
    if 2 * s <= sa {
        div255(2 * s * d + tmp)
    } else {
        div255(sa * da - 2 * (da - d) * (sa - s) + tmp)
    }
}

pub(crate) fn blend_hard_light(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_hard_light_op(sr, dr, sa, da);
            let g = blend_hard_light_op(sg, dg, sa, da);
            let b = blend_hard_light_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_soft_light_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    let s2 = s << 1;
    // let d_np = if da != 0 { (255 * d) / da } else { 0 };
    let d_np = if da != 0 {
        255u32.wrapping_mul(d).wrapping_div(da)
    } else {
        0
    };
    // let temp = (s * (255 - da) + d * (255 - sa)) * 255;
    let temp = s
        .wrapping_mul(255u32.wrapping_sub(da))
        .wrapping_add(d.wrapping_mul(255u32.wrapping_sub(sa)))
        .wrapping_mul(255);

    if s2 < sa {
        // (d * (sa * 255 + (s2 - sa) * (255 - d_np)) + temp) / 65025
        d.wrapping_mul(
            sa.wrapping_mul(255)
                .wrapping_add(s2.wrapping_sub(sa).wrapping_mul(255u32.wrapping_sub(d_np)))
                .wrapping_add(temp),
        )
        .wrapping_div(65025)
    } else if 4 * d <= da {
        (d * sa * 255
            + da * (s2 - sa) * ((((16 * d_np - 12 * 255) * d_np + 3 * 65025) * d_np) / 65025)
            + temp)
            / 65025
    } else {
        ((d * sa * 255 + da * (s2 - sa) * (f32::sqrt(d_np as f32 * 255.0) as u32) - d_np) + temp)
            / 65025
    }
}
pub(crate) fn blend_soft_light(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_soft_light_op(sr, dr, sa, da);
            let g = blend_soft_light_op(sg, dg, sa, da);
            let b = blend_soft_light_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_difference_op(s: u32, d: u32, sa: u32, da: u32) -> u32 {
    let sda = s * da;
    let dsa = d * sa;

    if sda < dsa {
        d + s - 2 * div255(sda)
    } else {
        s + d - 2 * div255(dsa)
    }
}
pub(crate) fn blend_difference(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_difference_op(sr, dr, sa, da);
            let g = blend_difference_op(sg, dg, sa, da);
            let b = blend_difference_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub fn blend_exclusion_op(s: u32, d: u32, _sa: u32, _da: u32) -> u32 {
    div255(255 * (s + d) - 2 * s * d)
}

pub(crate) fn blend_exclusion(input1: &mut Surface, input2: &mut Surface, output: &mut Surface) {
    for y in 0..output.height {
        for x in 0..output.width {
            let [sr, sg, sb, sa] = init_load_pixel(input1, x, y, input1.channel);
            let [dr, dg, db, da] = init_load_pixel(input2, x, y, input2.channel);
            let r = blend_exclusion_op(sr, dr, sa, da);
            let g = blend_exclusion_op(sg, dg, sa, da);
            let b = blend_exclusion_op(sb, db, sa, da);
            let a = sa + da - div255(sa * da);
            clamp_and_store_pixel(output, x, y, r, g, b, a);
        }
    }
}

pub(crate) fn composite_over(in1: &mut Surface, in2: &mut Surface, out: &mut Surface) {
    for y in 0..out.height {
        for x in 0..out.width {
            let [sr, sg, sb, sa] = init_load_pixel(in1, x, y, in1.channel);
            let [dr, dg, db, da] = init_load_pixel(in2, x, y, in2.channel);

            let inv_sa = 255 - sa;

            let r = sr + div255(dr * inv_sa);
            let g = sg + div255(dg * inv_sa);
            let b = sb + div255(db * inv_sa);
            let a = sa + div255(da * inv_sa);

            store_pixel(out, x, y, r, g, b, a);
        }
    }
}

pub(crate) fn composite_in(in1: &mut Surface, in2: &mut Surface, out: &mut Surface) {
    for y in 0..out.height {
        for x in 0..out.width {
            let [sr, sg, sb, sa] = init_load_pixel(in1, x, y, in1.channel);

            let da = alpha(get_pixel(in2, x, y), in2.channel);

            let r = div255(sr * da);
            let g = div255(sg * da);
            let b = div255(sb * da);
            let a = div255(sa * da);

            store_pixel(out, x, y, r, g, b, a);
        }
    }
}

pub(crate) fn composite_out(in1: &mut Surface, in2: &mut Surface, out: &mut Surface) {
    for y in 0..out.height {
        for x in 0..out.width {
            let [sr, sg, sb, sa] = init_load_pixel(in1, x, y, in1.channel);

            let inv_da = 255 - alpha(get_pixel(in2, x, y), in2.channel);

            let r = div255(sr * inv_da);
            let g = div255(sg * inv_da);
            let b = div255(sb * inv_da);
            let a = div255(sa * inv_da);

            store_pixel(out, x, y, r, g, b, a);
        }
    }
}

pub(crate) fn composite_atop(in1: &mut Surface, in2: &mut Surface, out: &mut Surface) {
    for y in 0..out.height {
        for x in 0..out.width {
            let [sr, sg, sb, sa] = init_load_pixel(in1, x, y, in1.channel);
            let [dr, dg, db, da] = init_load_pixel(in2, x, y, in2.channel);

            let inv_sa = 255 - sa;

            let r = div255(sr * da) + div255(dr * inv_sa);
            let g = div255(sg * da) + div255(dg * inv_sa);
            let b = div255(sb * da) + div255(db * inv_sa);

            store_pixel(out, x, y, r, g, b, da);
        }
    }
}

pub(crate) fn composite_xor(in1: &mut Surface, in2: &mut Surface, out: &mut Surface) {
    for y in 0..out.height {
        for x in 0..out.width {
            let [sr, sg, sb, sa] = init_load_pixel(in1, x, y, in1.channel);
            let [dr, dg, db, da] = init_load_pixel(in2, x, y, in2.channel);

            let inv_sa = 255 - sa;
            let inv_da = 255 - da;
            let r = div255(sr * inv_da) + div255(dr * inv_sa);
            let g = div255(sg * inv_da) + div255(dg * inv_sa);
            let b = div255(sb * inv_da) + div255(db * inv_sa);
            let a = div255(sa * inv_da) + div255(da * inv_sa);

            store_pixel(out, x, y, r, g, b, a);
        }
    }
}
