#[cfg(test)]
mod color_transform_grayscale_test {
    use plutofilter_rs::{ImageEditor, get_resource_path};
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn color_transform_grayscale_test_0() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_grayscale(0.0);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_grayscale"],
            "000.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn color_transform_grayscale_test_25() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_grayscale(0.25);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_grayscale"],
            "025.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn color_transform_grayscale_test_50() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_grayscale(0.50);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_grayscale"],
            "050.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn color_transform_grayscale_test_75() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_grayscale(0.75);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_grayscale"],
            "075.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn color_transform_grayscale_test_100() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_grayscale(1.0);

        let output_path = get_resource_path(
            &["test_output_images", "color_transform_grayscale"],
            "100.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }
}
