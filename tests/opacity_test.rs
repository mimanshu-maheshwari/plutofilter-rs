#[cfg(test)]
mod color_transform_opacity_test {
    use plutofilter_rs::{ImageEditor, get_resource_path};

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn color_transform_opacity_test_0() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_opacity(0.0);
        let output_path = get_resource_path(
            &["test_output_images", "color_transform_opacity"],
            "000.png",
        );
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn color_transform_opacity_test_25() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_opacity(0.25);
        let output_path = get_resource_path(
            &["test_output_images", "color_transform_opacity"],
            "025.png",
        );
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn color_transform_opacity_test_50() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_opacity(0.50);
        let output_path = get_resource_path(
            &["test_output_images", "color_transform_opacity"],
            "050.png",
        );
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn color_transform_opacity_test_75() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_opacity(0.75);
        let output_path = get_resource_path(
            &["test_output_images", "color_transform_opacity"],
            "075.png",
        );
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn color_transform_opacity_test_100() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform_opacity(1.0);
        let output_path = get_resource_path(
            &["test_output_images", "color_transform_opacity"],
            "100.png",
        );
        editor.save_to(output_path)?;
        Ok(())
    }
}
