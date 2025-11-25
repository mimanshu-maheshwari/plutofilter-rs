#[cfg(test)]
#[cfg(feature = "image")]
mod color_transform_test {
    use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};
    use plutofilter_rs::*;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn composite_arithmatic_test_1() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
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

        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
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
        Surface::composite_arithmetic(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            0.0,
            1.0,
            1.0,
            0.0,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_1.png",
        );
        output_image.save(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_arithmatic_test_2() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
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

        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
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
        Surface::composite_arithmetic(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            0.5,
            0.5,
            0.5,
            0.0,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_2.png",
        );
        output_image.save(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_arithmatic_test_3() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
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

        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
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
        Surface::composite_arithmetic(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            1.0,
            0.0,
            0.0,
            0.0,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_3.png",
        );
        output_image.save(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_arithmatic_test_4() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
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

        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
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
        Surface::composite_arithmetic(
            &mut base_image_surface,
            &mut blend_image_surface,
            &mut output_surface,
            1.0,
            0.0,
            0.0,
            0.5,
        );
        // eprintln!("Created blend output");
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_4.png",
        );
        output_image.save(output_path)?;
        Ok(())
    }
}
