#[cfg(test)]
#[cfg(feature = "image")]
mod gaussian_blur_test {
    use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};
    use plutofilter_rs::*;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Test Gaussian Blur 0x0
    #[test]
    fn test_gaussian_blur_0x0() -> Result<()> {
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

        Surface::gaussian_blur(&mut base_image_surface, &mut output_surface, 0.0, 0.0);
        // eprintln!("Created gaussian blur");

        let output_path = get_resource_path(
            &["test_output_images", "gaussian_blur"],
            "gaussian_blur-0x0.png",
        );
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    /// Test Gaussian Blur 5x5
    #[test]
    fn test_gaussian_blur_5x5() -> Result<()> {
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

        Surface::gaussian_blur(&mut base_image_surface, &mut output_surface, 5.0, 5.0);
        // eprintln!("Created gaussian blur");

        let output_path = get_resource_path(
            &["test_output_images", "gaussian_blur"],
            "gaussian_blur-5x5.png",
        );
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    /// Test Gaussian Blur 10x10
    #[test]
    fn test_gaussian_blur_10x10() -> Result<()> {
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

        Surface::gaussian_blur(&mut base_image_surface, &mut output_surface, 10.0, 10.0);
        // eprintln!("Created gaussian blur");

        let output_path = get_resource_path(
            &["test_output_images", "gaussian_blur"],
            "gaussian_blur-10x10.png",
        );
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }
}
