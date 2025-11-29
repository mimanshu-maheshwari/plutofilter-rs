use image::{DynamicImage, ImageReader};
use plutofilter_rs::*;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let base_file = get_resource_path(&["original_images"], "rgb-test.png");

    let mut base_image = DynamicImage::ImageRgba8(
        ImageReader::open(&base_file)
            .map_err(|e| {
                println!("{base_file:?} not found, {e}");
                e
            })?
            .decode()?
            .into_rgba8(),
    );
    let _base_image_surface = Surface::from_image(&mut base_image);

    Ok(())
}
