mod brightness_test;
mod contrast_test;
mod grayscale_test;
mod hue_rotate_test;
mod invert_test;
mod opacity_test;
mod saturate_test;
mod sepia_test;

#[cfg(test)]
#[cfg(feature = "image")]
mod color_transform_test {
    use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};
    use plutofilter_rs::*;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn color_transform_test_original() -> Result<()> {
        const ORIGINAL: [f32; 20] = [
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
        ];
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

        Surface::color_transform(&mut base_image_surface, &mut output_surface, ORIGINAL);
        // eprintln!("Created gaussian blur");

        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "original.png");
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_test_grayscale() -> Result<()> {
        const GRAYSCALE: [f32; 20] = [
            0.2126, 0.7152, 0.0722, 0.0, 0.0, 0.2126, 0.7152, 0.0722, 0.0, 0.0, 0.2126, 0.7152,
            0.0722, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        ];
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

        Surface::color_transform(&mut base_image_surface, &mut output_surface, GRAYSCALE);
        // eprintln!("Created gaussian blur");

        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "grayscale.png");
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_test_sepia() -> Result<()> {
        const SEPIA: [f32; 20] = [
            0.393, 0.769, 0.189, 0.0, 0.0, 0.349, 0.686, 0.168, 0.0, 0.0, 0.272, 0.534, 0.131, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        ];

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

        Surface::color_transform(&mut base_image_surface, &mut output_surface, SEPIA);
        // eprintln!("Created gaussian blur");

        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "sepia.png");
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_test_contrast() -> Result<()> {
        const CONTRAST: [f32; 20] = [
            1.75, 0.0, 0.0, 0.0, -0.375, 0.0, 1.75, 0.0, 0.0, -0.375, 0.0, 0.0, 1.75, 0.0, -0.375,
            0.0, 0.0, 0.0, 1.0, 0.0,
        ];
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

        Surface::color_transform(&mut base_image_surface, &mut output_surface, CONTRAST);
        // eprintln!("Created gaussian blur");

        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "contrast.png");
        output_image.save(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }
}
