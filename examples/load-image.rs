use plutofilter_rs::{ImageEditor, get_resource_path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
    let mut editor = ImageEditor::open(base_file);
    const SEPIA: [f32; 20] = [
        0.393, 0.769, 0.189, 0.0, 0.0, 0.349, 0.686, 0.168, 0.0, 0.0, 0.272, 0.534, 0.131, 0.0,
        0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
    ];
    editor = editor.color_transform(SEPIA);
    let output_path = get_resource_path(&["test_output_images", "color_transform"], "sepia.png");
    editor.save_to(output_path)?;

    Ok(())
}

#[allow(dead_code)]
fn test_rgb_test_image() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let base_file = plutofilter_rs::get_resource_path(&["original_images"], "rgb-test.png");

    let mut base_image = image::DynamicImage::ImageRgba8(
        image::ImageReader::open(&base_file)
            .map_err(|e| {
                println!("{base_file:?} not found, {e}");
                e
            })?
            .decode()?
            .into_rgba8(),
    );
    let _base_image_surface = plutofilter_rs::Surface::from_image(&mut base_image);

    Ok(())
}
