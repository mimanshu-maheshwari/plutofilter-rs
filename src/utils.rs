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
