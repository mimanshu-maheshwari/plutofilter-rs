#[cfg(test)]
#[cfg(feature = "image")]
mod color_transform_saturate_test {
    use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};
    use plutofilter_rs::*;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn color_transform_saturate_test_0() -> Result<()> {
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

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");

        Surface::color_transform_saturate(&mut base_image_surface, &mut output_surface, 0.0);
        // eprintln!("Created gaussian blur");

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "000.png",
        );
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_saturate_test_1() -> Result<()> {
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

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");

        Surface::color_transform_saturate(&mut base_image_surface, &mut output_surface, 1.0);
        // eprintln!("Created gaussian blur");

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "100.png",
        );
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_saturate_test_4() -> Result<()> {
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

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");

        Surface::color_transform_saturate(&mut base_image_surface, &mut output_surface, 4.0);
        // eprintln!("Created gaussian blur");

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "400.png",
        );
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_saturate_test_05() -> Result<()> {
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

        let output_pixels = ImageBuffer::from_vec(
            base_image_width,
            base_image_height,
            vec![0u8; (4 * base_image_surface.width() * base_image_surface.height()) as usize],
        )
        .unwrap();
        let mut output_image = DynamicImage::ImageRgba8(output_pixels);
        let mut output_surface = Surface::from_image(&mut output_image);
        // eprintln!("Created output image surface");

        Surface::color_transform_saturate(&mut base_image_surface, &mut output_surface, 0.50);
        // eprintln!("Created gaussian blur");

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "050.png",
        );
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }
}
