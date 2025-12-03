#[cfg(test)]
#[cfg(feature = "image")]
mod color_transform_saturate_test {
    use plutofilter_rs::{ImageEditor, get_resource_path};
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn color_transform_saturate_test_0() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_saturate(0.0);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "000.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn color_transform_saturate_test_1() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_saturate(1.0);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "100.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn color_transform_saturate_test_4() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_saturate(4.0);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "400.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn color_transform_saturate_test_05() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_saturate(0.50);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_saturate"],
            "050.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }
}
