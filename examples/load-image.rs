use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};
use plutofilter_rs::*;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// Helper function to find file in res folder
fn get_resource_path(filename: &str) -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("res").join(filename) // Assuming a 'resources' directory at crate root
}
fn main() -> Result<()> {
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

    let output_pixels = ImageBuffer::from_vec(
        base_image_width,
        base_image_height,
        vec![0u8; (4 * base_image_width * base_image_height) as usize],
    )
    .unwrap();
    let mut output_image = DynamicImage::ImageRgba8(output_pixels);
    let mut output_surface = Surface::from_image(&mut output_image);
    Surface::blend(
        &mut base_image_surface,
        &mut blend_image_surface,
        &mut output_surface,
        BlendMode::Normal,
    );
    let output_path = get_resource_path("test_output_images/blend_normal.png");
    output_image.save(output_path)?;
    Ok(())
}
