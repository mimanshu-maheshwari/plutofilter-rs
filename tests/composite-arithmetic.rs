#[cfg(test)]
#[cfg(feature = "image")]
mod color_transform_test {
    use plutofilter_rs::{ImageEditor, get_resource_path};
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn composite_arithmatic_test_1() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_1.png",
        );
        editor = editor.composite_arithmetic(blend_image, 0.0, 1.0, 1.0, 0.0);
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_arithmatic_test_2() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_2.png",
        );
        editor = editor.composite_arithmetic(blend_image, 0.5, 0.5, 0.5, 0.0);
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_arithmatic_test_3() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
        editor = editor.composite_arithmetic(blend_image, 1.0, 0.0, 0.0, 0.0);
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_3.png",
        );
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_arithmatic_test_4() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
        editor = editor.composite_arithmetic(blend_image, 1.0, 0.0, 0.0, 0.5);
        let output_path = get_resource_path(
            &["test_output_images", "composite_arithmatic"],
            "test_4.png",
        );
        editor.save_to(output_path)?;
        Ok(())
    }
}
