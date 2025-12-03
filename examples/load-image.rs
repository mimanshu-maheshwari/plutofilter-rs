use image::ImageResult;
#[cfg(feature = "image")]
fn main() -> ImageResult<()> {
    use plutofilter_rs::{ImageEditor, get_resource_path};

    let base_file = get_resource_path(&["original_images"], "test-image.jpg");
    let editor = ImageEditor::open(&base_file);
    let output_path = get_resource_path(&["test_output_images", "example"], "test-image.jpg");
    editor
        .color_transform_contrast_inplace(0.97)
        .color_transform_hue_rotate_inplace(330.0)
        .color_transform_saturate_inplace(1.11)
        .save_to(&output_path)
}
#[cfg(not(feature = "image"))]
fn main() -> Result<(), SurfaceError> {
    use plutofilter_rs::ColorChannel;
    use plutofilter_rs::Surface;
    use plutofilter_rs::SurfaceError;

    let (width, height) = (100, 100);
    let mut input_image_vec = vec![0xFFAA22; (width * height) as usize];
    let mut output_image_vec = vec![0xAAFF22; (width * height) as usize];
    let mut input_surface = Surface::make(&mut input_image_vec, width, height, width, None)?;
    let mut output_surface = Surface::make(&mut output_image_vec, width, height, width, None)?;
    Surface::color_transform_opacity(&mut input_surface, &mut output_surface, 0.5);
    Ok(())
}
