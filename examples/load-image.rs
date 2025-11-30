use image::ImageResult;
use plutofilter_rs::{ImageEditor, get_resource_path};

fn main() -> ImageResult<()> {
    let base_file = get_resource_path(&["original_images"], "test-image.jpg");
    let editor = ImageEditor::open(&base_file);
    let output_path = get_resource_path(&["test_output_images", "example"], "test-image.jpg");
    editor
        .color_transform_contrast_inplace(0.97)
        .color_transform_hue_rotate_inplace(330.0)
        .color_transform_saturate_inplace(1.11)
        .save_to(&output_path)
}
