//! PlutoFilter implementation

mod error;
// mod pixel;
mod utils;

use std::{cell::RefCell, rc::Rc};

use crate::{error::SurfaceError, utils::*};

///
/// Blend modes for combining source and backdrop surfaces.
///
pub enum BlendMode {
    /// Standard alpha compositing (source over backdrop)
    Normal,
    /// Multiplies source and backdrop, producing a darker result
    Multiply,
    /// Inverts, multiplies, then inverts result, producing a lighter result
    Screen,
    /// Multiplies or screens depending on backdrop tone
    Overlay,
    /// Chooses the darker of source and backdrop
    Darken,
    /// Chooses the lighter of source and backdrop
    Lighten,
    /// Brightens backdrop in proportion to source
    ColorDodge,
    /// Darkens backdrop in proportion to source
    ColorBurn,
    /// Multiplies or screens depending on source tone
    HardLight,
    /// Darkens or lightens depending on source tone
    SoftLight,
    ///  Subtracts darker from lighter color
    Difference,
    /// Similar to difference but with lower contrast
    Exclusion,
}

///
/// Compositing operators for combining source and backdrop surfaces.
///
pub enum CompositeOperator {
    /// Display source over backdrop
    Over,
    /// Keep only portions of source within backdrop
    In,
    /// Keep only portions of source outside backdrop
    Out,
    /// Display source over backdrop, preserving backdrop alpha
    Atop,
    /// Keep non‑overlapping parts of both source and backdrop
    Xor,
}

/// Represents a 2D image surface in ARGB32 premultiplied format.
///
/// Each pixel is a 32-bit unsigned integer with channels ordered as: alpha, red, green, blue.
/// The red, green, and blue channels are premultiplied by the alpha channel.
/// That is, red = red * alpha / 255, and similarly for green and blue.
///
/// The pixel data is stored in row-major order. Each row begins at a multiple of `stride`.
pub struct Surface<'a> {
    /// Pointer to the pixel buffer.
    ///
    /// Must point to at least `stride * height` elements in ARGB32 premultiplied format.
    pub(crate) pixels: &'a mut [u32],

    /// The width of the surface in pixels.
    pub(crate) width: usize,

    /// The height of the surface in pixels.
    pub(crate) height: usize,

    /// The number of pixels per row.
    ///
    /// Must be greater than or equal to `width`.
    pub(crate) stride: usize,
}

impl<'a> Surface<'a> {
    /// Creates a surface from a raw pixel buffer.
    ///
    /// # Arguments
    /// * `pixels` - Pointer to the pixel buffer in ARGB32 premultiplied format.
    /// * `width`  - The width of the surface in pixels.
    /// * `height` - The height of the surface in pixels.
    /// * `stride` - The number of pixels per row (must be greater than or equal to width).
    /// # Returns [Surface] representing the given pixel buffer.
    ///
    /// # Example
    /// ```
    /// use plutofilter_rs::Surface;
    /// let stride  = 320;
    /// let width   = 300;
    /// let height  = 280;
    /// let pixels  = vec![0; 320 * 280];
    /// let surface = Surface::make(&pixels, width, height, stride).expect("ERROR: Failed to make surface struct.");
    /// ```
    ///
    pub fn make(
        pixels: &'a mut [u32],
        width: usize,
        height: usize,
        stride: usize,
    ) -> Result<Self, SurfaceError> {
        if pixels.len() < stride as usize * height as usize {
            Err(SurfaceError::InvalidPixelLength)
        } else if stride < width {
            Err(SurfaceError::StrideLessThanWidth)
        } else {
            Ok(Self {
                pixels,
                width,
                height,
                stride,
            })
        }
    }

    /// Creates a subregion of an existing surface.
    ///
    /// # Arguments
    /// * `surface` - The source surface.
    /// * `x` - The horizontal offset of the subregion, in pixels.
    /// * `y` - The vertical offset of the subregion, in pixels.
    /// * `width` - The width of the subregion in pixels.
    /// * `height` - The height of the subregion in pixels.
    ///
    /// # Returns
    /// [Surface] referencing the specified subregion, clipped to the bounds of the source surface.
    ///
    pub fn make_sub(
        &'a mut self,
        mut x: usize,
        mut y: usize,
        mut width: usize,
        mut height: usize,
    ) -> Result<Self, SurfaceError> {
        if x > self.width {
            x = self.width;
        }

        if y > self.height {
            y = self.height;
        }

        if x + width > self.width {
            width = self.width - x;
        }

        if y + height > self.height {
            height = self.height - y;
        }
        let pixels = &mut (self.pixels[(y as usize * self.stride as usize + x as usize)..]);
        Self::make(pixels, width, height, self.stride)
    }

    ///
    /// Applies a 5x4 color transformation matrix to each pixel in the input surface.
    ///
    /// The transformation is applied in-place from `in` to `out`, using a matrix that operates
    /// on premultiplied ARGB channels. Each output pixel is computed as:
    ///
    ///    [R']   [ m0  m1  m2  m3  m4 ]   [R]
    ///    [G'] = [ m5  m6  m7  m8  m9 ] x [G]
    ///    [B']   [m10 m11 m12 m13 m14]   [B]
    ///    [A']   [m15 m16 m17 m18 m19]   [A]
    ///
    /// The matrix must be provided in row-major order as a flat array of 20 floats.
    /// The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input`  - The input surface (read-only if different from out).
    /// * `output` - The output surface.
    /// * `matrix` - A 5x4 color matrix represented as a 20-element float array.
    ///
    pub fn color_transform(input: &mut Self, output: &mut Self, matrix: [f32; 20]) {
        overlap_surface(input, output);
        for y in 0..output.height {
            for x in 0..output.width {
                let (mut r, mut g, mut b, mut a) = init_load_pixel(input, x, y);
                unpremultiply_pixel(&mut r, &mut g, &mut b, &mut a);

                let rr = r as f32 * matrix[0]
                    + g as f32 * matrix[1]
                    + b as f32 * matrix[2]
                    + a as f32 * matrix[3]
                    + matrix[4] * 255.0;
                let gg = r as f32 * matrix[5]
                    + g as f32 * matrix[6]
                    + b as f32 * matrix[7]
                    + a as f32 * matrix[8]
                    + matrix[9] * 255.0;
                let bb = r as f32 * matrix[10]
                    + g as f32 * matrix[11]
                    + b as f32 * matrix[12]
                    + a as f32 * matrix[13]
                    + matrix[14] * 255.0;
                let aa = r as f32 * matrix[15]
                    + g as f32 * matrix[16]
                    + b as f32 * matrix[17]
                    + a as f32 * matrix[18]
                    + matrix[19] * 255.0;
                r = clamp_pixel(rr as u32);
                g = clamp_pixel(gg as u32);
                b = clamp_pixel(bb as u32);
                a = clamp_pixel(aa as u32);

                premultiply_pixel(&mut r, &mut g, &mut b, &mut a);
                store_pixel(output, x, y, r, g, b, a);
            }
        }
    }

    /// Adjusts the opacity of each pixel by a uniform amount.
    ///
    /// Multiplies the alpha channel by the given amount. The result is clamped to the range [0, 255].
    /// The input and output surfaces may refer to the same buffer.
    ///
    /// # Arguments
    /// * `input`  - The input surface.
    /// * `output` - The output surface.
    /// * `amount` - The opacity multiplier (0 for fully transparent, 1 for unchanged).
    ///
    pub fn color_transform_opacity(input: &mut Self, output: &mut Self, amount: f32) {
        let matrix = [
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, amount, 0.0,
        ];

        Self::color_transform(input, output, matrix);
    }

    ///
    /// Adjusts the brightness of each pixel by a uniform amount.
    ///
    /// Multiplies the color channels (red, green, blue) by the given amount. The result is clamped to the range [0, 255].
    /// The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input`  - The input surface.
    /// * `output` - The output surface.
    /// * `amount` - The brightness multiplier (1 for unchanged, <1 to darken, >1 to brighten).
    ///
    pub fn color_transform_brightness(input: &mut Self, output: &mut Self, amount: f32) {
        let matrix = [
            amount, 0.0, 0.0, 0.0, 0.0, 0.0, amount, 0.0, 0.0, 0.0, 0.0, 0.0, amount, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 0.0,
        ];
        Self::color_transform(input, output, matrix);
    }

    ///
    /// Inverts the color channels of each pixel by a uniform amount.
    ///
    ///Linearly interpolates between the original color and its inverse based on the given amount.
    ///The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    ///
    /// # Arguments
    /// * `input`  - The input surface.
    /// * `output` - The output surface.
    /// * `amount` - The inversion amount (0 for unchanged, 1 for fully inverted).
    ///
    pub fn color_transform_invert(input: &mut Self, output: &mut Self, amount: f32) {
        let scale = 1.0 - 2.0 * amount;
        let matrix = [
            scale, 0.0, 0.0, 0.0, amount, 0.0, scale, 0.0, 0.0, amount, 0.0, 0.0, scale, 0.0,
            amount, 0.0, 0.0, 0.0, 1.0, 0.0,
        ];

        Self::color_transform(input, output, matrix);
    }

    ///
    /// Adjusts the contrast of each pixel by a uniform amount.
    ///
    ///Scales the color channels (red, green, blue) away from or toward the midpoint (0.5) by the given amount.
    ///The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input` - The input surface.
    /// * `output` - The output surface.
    /// * `amount` - The contrast multiplier (1 for unchanged, <1 to reduce contrast, >1 to increase contrast).
    ///
    pub fn color_transform_contrast(input: &mut Self, output: &mut Self, amount: f32) {
        let offset = (1.0 - amount) * 0.5;
        let matrix = [
            amount, 0.0, 0.0, 0.0, offset, 0.0, amount, 0.0, 0.0, offset, 0.0, 0.0, amount, 0.0,
            offset, 0.0, 0.0, 0.0, 1.0, 0.0,
        ];

        Self::color_transform(input, output, matrix);
    }

    ///
    /// Adjusts the saturation of each pixel by a uniform amount.
    ///
    ///Modifies the intensity of color while preserving luminance. A value of 0 produces a fully desaturated result.
    ///The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input`  - The input surface.
    /// * `output` - The output surface.
    /// * `amount` - The saturation multiplier (1 for unchanged, 0 for fully desaturated, >1 to increase saturation).
    ///
    pub fn color_transform_saturate(input: &mut Self, output: &mut Self, amount: f32) {
        let matrix = [
            0.213 + 0.787 * amount,
            0.715 - 0.715 * amount,
            0.072 - 0.072 * amount,
            0.0,
            0.0,
            0.213 - 0.213 * amount,
            0.715 + 0.285 * amount,
            0.072 - 0.072 * amount,
            0.0,
            0.0,
            0.213 - 0.213 * amount,
            0.715 - 0.715 * amount,
            0.072 + 0.928 * amount,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
        ];

        Self::color_transform(input, output, matrix);
    }

    ///
    /// Converts each pixel toward grayscale by a uniform amount.
    ///
    ///Reduces the influence of color while preserving luminance. A value of 1 produces a fully grayscale result.
    ///The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// *`input` - The input surface.
    /// *`output` - The output surface.
    /// *`amount` -  The grayscale amount (0 for unchanged, 1 for fully grayscale).
    ///
    pub fn color_transform_grayscale(input: &mut Self, output: &mut Self, amount: f32) {
        let inv_amount = 1.0 - amount;
        let matrix = [
            inv_amount + amount * 0.2126,
            amount * 0.7152,
            amount * 0.0722,
            0.0,
            0.0,
            amount * 0.2126,
            inv_amount + amount * 0.7152,
            amount * 0.0722,
            0.0,
            0.0,
            amount * 0.2126,
            amount * 0.7152,
            inv_amount + amount * 0.0722,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
        ];

        Self::color_transform(input, output, matrix);
    }

    ///
    /// Applies a sepia tone to each pixel by a uniform amount.
    ///
    ///Shifts the colors toward warm brown tones, simulating the appearance of old photographs.
    ///A value of 1 produces a fully sepia-toned result. The alpha channel is not affected.
    ///The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input` - The input surface.
    /// * `output` - The output surface.
    /// * `amount` - The sepia amount (0 for unchanged, 1 for fully sepia).
    ///
    pub fn color_transform_sepia(input: &mut Self, output: &mut Self, amount: f32) {
        let inv_amount = 1.0 - amount;
        let matrix = [
            0.393 + 0.607 * inv_amount,
            0.769 - 0.769 * inv_amount,
            0.189 - 0.189 * inv_amount,
            0.0,
            0.0,
            0.349 - 0.349 * inv_amount,
            0.686 + 0.314 * inv_amount,
            0.168 - 0.168 * inv_amount,
            0.0,
            0.0,
            0.272 - 0.272 * inv_amount,
            0.534 - 0.534 * inv_amount,
            0.131 + 0.869 * inv_amount,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
        ];

        Self::color_transform(input, output, matrix);
    }

    ///
    /// Rotates the hue of each pixel by a given angle.
    ///
    ///Shifts the hue component of the color while preserving luminance and saturation.
    ///The angle is specified in degrees and wraps around automatically.
    ///The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input` - The input surface.
    /// * `output` - The output surface.
    /// * `angle` - The hue rotation angle in degrees (0 for unchanged, 360 for full rotation).
    ///
    pub fn color_transform_hue_rotate(input: &mut Self, output: &mut Self, angle: f32) {
        let a1 = f32::cos(deg2rad(angle));
        let a2 = f32::sin(deg2rad(angle));
        let matrix = [
            0.213 + a1 * 0.787 - a2 * 0.213,
            0.715 - a1 * 0.715 - a2 * 0.715,
            0.072 - a1 * 0.072 + a2 * 0.928,
            0.0,
            0.0,
            0.213 - a1 * 0.213 + a2 * 0.143,
            0.715 + a1 * 0.285 + a2 * 0.140,
            0.072 - a1 * 0.072 - a2 * 0.283,
            0.0,
            0.0,
            0.213 - a1 * 0.213 - a2 * 0.787,
            0.715 - a1 * 0.715 + a2 * 0.715,
            0.072 + a1 * 0.928 + a2 * 0.072,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
        ];

        Self::color_transform(input, output, matrix);
    }

    ///
    ///Sets the alpha channel of each pixel based on its luminance.
    ///
    ///Replaces the alpha channel with the computed luminance of the color channels.
    ///The RGB channels are set to zero. Luminance is calculated using the formula:
    ///
    ///    alpha = 0.2126 * R + 0.7152 * G + 0.0722 * B
    ///
    ///The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input` - in The input surface.
    /// * `output` - out The output surface.
    ///
    pub fn color_transform_luminance_to_alpha(input: &mut Self, output: &mut Self) {
        overlap_surface(input, output);

        for y in 0..output.height {
            for x in 0..output.width {
                let (mut r, mut g, mut b, mut a) = init_load_pixel(input, x, y);
                unpremultiply_pixel(&mut r, &mut g, &mut b, &mut a);

                let l = r as f32 * 0.2125 + g as f32 * 0.7154 + b as f32 * 0.0721;

                store_pixel(output, x, y, 0, 0, 0, clamp_pixel(l as u32));
            }
        }
    }

    ///
    /// Converts the color channels from sRGB to linear RGB.
    ///
    ///Applies gamma correction to convert red, green, and blue channels from sRGB to linear space.
    ///The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input`  - in The input surface.
    /// * `output` - out The output surface.
    ///
    pub fn color_transform_srgb_to_linear_rgb(input: &mut Self, output: &mut Self) {
        overlap_surface(input, output);
        for y in 0..output.height {
            for x in 0..output.width {
                let (mut r, mut g, mut b, mut a) = init_load_pixel(input, x, y);
                unpremultiply_pixel(&mut r, &mut g, &mut b, &mut a);
                srgb_to_linear_rgb(&mut r, &mut g, &mut b);
                premultiply_pixel(&mut r, &mut g, &mut b, &mut a);
                store_pixel(output, x, y, r, g, b, a);
            }
        }
    }

    ///
    ///Converts the color channels from linear RGB to sRGB.
    ///
    ///Applies gamma encoding to convert red, green, and blue channels from linear space to sRGB.
    ///The alpha channel is not affected. The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input` - in The input surface.
    /// * `output` - out The output surface.
    ///
    pub fn color_transform_linear_rgb_to_srgb(input: &mut Self, output: &mut Self) {
        overlap_surface(input, output);
        for y in 0..output.height {
            for x in 0..output.width {
                let (mut r, mut g, mut b, mut a) = init_load_pixel(input, x, y);
                unpremultiply_pixel(&mut r, &mut g, &mut b, &mut a);
                liner_rgb_to_srgb(&mut r, &mut g, &mut b);
                premultiply_pixel(&mut r, &mut g, &mut b, &mut a);
                store_pixel(output, x, y, r, g, b, a);
            }
        }
    }

    ///
    /// Applies a Gaussian blur to the input surface.
    ///
    ///Performs separable convolution with a Gaussian kernel along the X and Y axes.
    ///The blur strength is controlled by the standard deviation parameters.
    ///
    ///The input and output surfaces may refer to the same buffer.
    /// # Arguments
    /// * `input` - The input surface.
    /// * `input` - The output surface.
    /// * `std_deviation_x` - The standard deviation of the blur along the X axis.
    /// * `std_deviation_y` - The standard deviation of the blur along the Y axis.
    ///
    pub fn gaussian_blur(
        input: &mut Self,
        output: &mut Self,
        std_deviation_x: f32,
        std_deviation_y: f32,
    ) {
        overlap_surface(input, output);
        let kernel_width = calc_kernel_size(std_deviation_x);
        let kernel_height = calc_kernel_size(std_deviation_y);
        if kernel_width <= 0 && kernel_height <= 0 {
            let size = output.width * output.height;
            for i in 0..size {
                output.pixels[i] = input.pixels[i];
            }
            return;
        }

        // TODO: handle this
        if kernel_height < 0 || kernel_width < 0 {
            panic!("kernel_height={kernel_height}, kernel_width={kernel_width} ")
        }

        let mut kernel_width = kernel_width as usize;
        let mut kernel_height = kernel_height as usize;

        if kernel_width > MAX_KERNEL_SIZE {
            kernel_width = MAX_KERNEL_SIZE;
        }
        if kernel_height > MAX_KERNEL_SIZE {
            kernel_height = MAX_KERNEL_SIZE;
        }

        let mut intermediate = [0; MAX_KERNEL_SIZE];

        let output: Rc<RefCell<_>> = Rc::new(RefCell::new(output));
        let input: Rc<RefCell<_>> = Rc::new(RefCell::new(input));
        box_blur(
            Rc::clone(&input),
            Rc::clone(&output),
            &mut intermediate,
            kernel_width,
            kernel_height,
        );
        box_blur(
            Rc::clone(&output),
            Rc::clone(&output),
            &mut intermediate,
            kernel_width,
            kernel_height,
        );
        box_blur(
            Rc::clone(&output),
            Rc::clone(&output),
            &mut intermediate,
            kernel_width,
            kernel_height,
        );
    }

    ///
    /// Blends two input surfaces using the specified blend mode.
    ///
    ///Applies the selected blend mode to combine `in1` (source) over `in2` (backdrop).
    ///The output surface may refer to either input.
    ///
    /// # Arguments
    /// * `in1` - The source surface.
    /// * `in2` - The backdrop surface.
    /// * `out` - The output surface.
    /// * `mode` - The blend mode to apply.
    ///
    pub fn blend(input1: &mut Self, input2: &mut Self, output: &mut Self, mode: BlendMode) {
        overlap_surface3(input1, input2, output);
        match mode {
            BlendMode::Normal => blend_normal(input1, input2, output),
            BlendMode::Multiply => blend_multiply(input1, input2, output),
            BlendMode::Screen => blend_screen(input1, input2, output),
            BlendMode::Overlay => blend_overlay(input1, input2, output),
            BlendMode::Darken => blend_darken(input1, input2, output),
            BlendMode::Lighten => blend_lighten(input1, input2, output),
            BlendMode::ColorDodge => blend_color_dodge(input1, input2, output),
            BlendMode::ColorBurn => blend_color_burn(input1, input2, output),
            BlendMode::HardLight => blend_hard_light(input1, input2, output),
            BlendMode::SoftLight => blend_soft_light(input1, input2, output),
            BlendMode::Difference => blend_difference(input1, input2, output),
            BlendMode::Exclusion => blend_exclusion(input1, input2, output),
        }
    }

    ///
    /// Composites two input surfaces using the specified operator.
    ///
    ///Applies the selected compositing rule to combine `in1` (source) over `in2` (backdrop).
    ///The output surface may refer to the same buffer as either input.
    ///
    /// # Arguments
    /// * `in1` - The source surface.
    /// * `in2` - The backdrop surface.
    /// * `out` - The output surface.
    /// * `op` - The compositing operator to apply.
    ///
    pub fn composite(in1: &mut Self, in2: &mut Self, out: &mut Self, op: CompositeOperator) {
        overlap_surface3(in1, in2, out);
        match op {
            CompositeOperator::Over => composite_over(in1, in2, out),
            CompositeOperator::In => composite_in(in1, in2, out),
            CompositeOperator::Out => composite_out(in1, in2, out),
            CompositeOperator::Atop => composite_atop(in1, in2, out),
            CompositeOperator::Xor => composite_xor(in1, in2, out),
        }
    }

    ///
    /// Composites two input surfaces using an arithmetic combination of their color components.
    ///
    ///Computes each output channel as:  
    ///result = k1 * in1 * in2 + k2 * in1 + k3 * in2 + k4
    ///The result is clamped to the valid range.
    ///
    ///The output surface may refer to the same buffer as either input.
    ///
    /// # Arguments
    /// * `in1` - The source surface.
    /// * `in2` - The backdrop surface.
    /// * `out` - The output surface.
    /// * `k1` - The coefficient for in1 * in2.
    /// * `k2` - The coefficient for in1.
    /// * `k3` - The coefficient for in2.
    /// * `k4` - The constant bias term.
    ///
    pub fn composite_arithmetic(
        in1: &mut Self,
        in2: &mut Self,
        out: &mut Self,
        k1: f32,
        k2: f32,
        k3: f32,
        k4: f32,
    ) {
        overlap_surface3(in1, in2, out);

        for y in 0..out.height {
            for x in 0..out.width {
                let (sr, sg, sb, sa) = init_load_pixel(in1, x, y);
                let (dr, dg, db, da) = init_load_pixel(in2, x, y);

                let (sr, sg, sb, sa) = (sr as f32, sg as f32, sb as f32, sa as f32);
                let (dr, dg, db, da) = (dr as f32, dg as f32, db as f32, da as f32);

                let fdr = k1 * ((sr * dr) / 255.0) + k2 * sr + k3 * dr + k4 * 255.0;
                let fdg = k1 * ((sg * dg) / 255.0) + k2 * sg + k3 * dg + k4 * 255.0;
                let fdb = k1 * ((sb * db) / 255.0) + k2 * sb + k3 * db + k4 * 255.0;
                let fda = k1 * ((sa * da) / 255.0) + k2 * sa + k3 * da + k4 * 255.0;

                let (fdr, fdg, fdb, fda) = (fdr as u32, fdg as u32, fdb as u32, fda as u32);

                clamp_and_store_pixel(out, x, y, fdr, fdg, fdb, fda);
            }
        }
    }
}
