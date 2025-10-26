#[cfg(test)]
mod blend_modes_tests {
    use image::{GenericImageView, ImageBuffer, ImageReader, Rgba};
    use plutofilter_rs::*;
    use std::path::PathBuf;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn get_resource_path(filename: &str) -> PathBuf {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir.join("res").join(filename) // Assuming a 'resources' directory at crate root
    }
    #[test]
    fn test_normal() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let base_image = ImageReader::open(&base_file)
            .map_err(|e| {
                println!("{base_file:?} not found, {e}");
                e
            })?
            .decode()?;
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_raw = base_image
            .into_rgba32f()
            .as_raw()
            .chunks(4)
            .map(|px| {
                let r = (px[0] * 255.0).round() as u32;
                let g = (px[1] * 255.0).round() as u32;
                let b = (px[2] * 255.0).round() as u32;
                let a = (px[3] * 255.0).round() as u32;
                (r << 24) | (g << 16) | (b << 8) | a
            })
            .collect::<Vec<_>>();
        let mut base_image_surface = Surface::make(
            base_image_raw.as_mut_slice(),
            base_image_width as usize,
            base_image_height as usize,
            base_image_width as usize,
        )?;

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let blend_image = ImageReader::open(&blend_image)
            .map_err(|e| {
                println!("{blend_image:?} not found, {e}");
                e
            })?
            .decode()?;
        let (blend_image_width, blend_image_height) = blend_image.dimensions();
        let mut blend_image_raw = blend_image
            .into_rgba32f()
            .as_raw()
            .chunks(4)
            .map(|px| {
                let r = (px[0] * 255.0).round() as u32;
                let g = (px[1] * 255.0).round() as u32;
                let b = (px[2] * 255.0).round() as u32;
                let a = (px[3] * 255.0).round() as u32;
                (r << 24) | (g << 16) | (b << 8) | a
            })
            .collect::<Vec<_>>();
        let mut blend_image_surface = Surface::make(
            blend_image_raw.as_mut_slice(),
            blend_image_width as usize,
            blend_image_height as usize,
            blend_image_width as usize,
        )?;
        let mut output_pixels = vec![0; (base_image_width * base_image_height) as usize];
        let output_pixels = output_pixels.as_mut_slice();
        let mut output_surface = Surface::make(
            output_pixels,
            base_image_width as usize,
            base_image_height as usize,
            base_image_width as usize,
        )?;
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Normal,
        );

        // ---------------- Save result ----------------
        let mut output_image =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::new(base_image_width, base_image_width);

        for (i, pixel) in output_surface.pixels().iter().enumerate() {
            let y = i as u32 / base_image_width;
            let x = i as u32 % base_image_width;
            let p = *pixel;
            let rgba = Rgba([
                ((p >> 24) & 0xFF) as u8,
                ((p >> 16) & 0xFF) as u8,
                ((p >> 8) & 0xFF) as u8,
                (p & 0xFF) as u8,
            ]);
            *output_image.get_pixel_mut(x, y) = rgba;
        }

        let output_path = get_resource_path("test_output_images/blend_normal.png");
        output_image.save(&output_path)?;
        println!("Saved blended image to {:?}", output_path);

        Ok(())
    }

    // #[test]
    // fn test_multiply() {}
    // #[test]
    // fn test_screen() {}
    // #[test]
    // fn test_overlay() {}
    // #[test]
    // fn test_darken() {}
    // #[test]
    // fn test_lighten() {}
    // #[test]
    // fn test_color_dodge() {}
    // #[test]
    // fn test_color_burn() {}
    // #[test]
    // fn test_hard_light() {}
    // #[test]
    // fn test_soft_light() {}
    // #[test]
    // fn test_difference() {}
    // #[test]
    // fn test_exclusion() {}
}
