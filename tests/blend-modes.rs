#[cfg(test)]
#[cfg(feature = "image")]
mod blend_modes_tests {
    use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};
    use plutofilter_rs::*;
    use std::path::PathBuf;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Helper function to find file in res folder
    fn get_resource_path(filename: &str) -> PathBuf {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir.join("res").join(filename) // Assuming a 'resources' directory at crate root
    }

    /// Test Blend mode `Normal`
    #[test]
    fn test_normal() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Normal,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/normal.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `Multiply`
    #[test]
    fn test_multiply() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Multiply,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/multiply.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `Screen`
    #[test]
    fn test_screen() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Screen,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/screen.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `Overlay`
    #[test]
    fn test_overlay() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Overlay,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/overlay.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `Darken`
    #[test]
    fn test_darken() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Darken,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/darken.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `Lighten`
    #[test]
    fn test_lighten() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Lighten,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/lighten.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `ColorDodge`
    #[test]
    fn test_color_dodge() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::ColorDodge,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/color_dodge.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `ColorBurn`
    #[test]
    fn test_color_burn() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::ColorBurn,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/color_burn.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `HardLight`
    #[test]
    fn test_hard_light() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::HardLight,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/hard_light.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `SoftLight`
    #[test]
    fn test_soft_light() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::SoftLight,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/soft_light.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `Difference`
    #[test]
    fn test_difference() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Difference,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/difference.png");
        output_image.save(output_path)?;
        Ok(())
    }

    /// Test Blend mode `Exclusion`
    #[test]
    fn test_exclusion() -> Result<()> {
        let base_file = get_resource_path("original_images/zhang-hanyun.jpg");
        let mut base_image = DynamicImage::ImageRgba8(
            ImageReader::open(&base_file)
                .map_err(|e| {
                    println!("{base_file:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let (base_image_width, base_image_height) = base_image.dimensions();
        let mut base_image_surface = Surface::from_image(&mut base_image);
        // eprintln!("Created base image surface");

        let blend_image = get_resource_path("original_images/royal-purple.png");
        let mut blend_image = DynamicImage::ImageRgba8(
            ImageReader::open(&blend_image)
                .map_err(|e| {
                    println!("{blend_image:?} not found, {e}");
                    e
                })?
                .decode()?
                .into_rgba8(),
        );
        let mut blend_image_surface = Surface::from_image(&mut blend_image);
        // eprintln!("Created blend image surface");

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");
        Surface::blend(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            BlendMode::Exclusion,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path("test_output_images/blend/exclusion.png");
        output_image.save(output_path)?;
        Ok(())
    }
}
