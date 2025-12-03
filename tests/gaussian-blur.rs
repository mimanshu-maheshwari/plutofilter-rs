#[cfg(test)]
#[cfg(feature = "image")]
mod gaussian_blur_test {
    use plutofilter_rs::{ImageEditor, get_resource_path};
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Test Gaussian Blur 0x0
    #[test]
    fn test_gaussian_blur_0x0() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.gaussian_blur(0.0, 0.0);

        let output_path = get_resource_path(
            &["test_output_images", "gaussian_blur"],
            "gaussian_blur-0x0.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Gaussian Blur 5x5
    #[test]
    fn test_gaussian_blur_5x5() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.gaussian_blur(5.0, 5.0);

        let output_path = get_resource_path(
            &["test_output_images", "gaussian_blur"],
            "gaussian_blur-5x5.png",
        );
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Gaussian Blur 10x10
    #[test]
    fn test_gaussian_blur_10x10() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.gaussian_blur(10.0, 10.0);

        let output_path = get_resource_path(
            &["test_output_images", "gaussian_blur"],
            "gaussian_blur-10x10.png",
        );

        editor.save_to(output_path)?;

        Ok(())
    }
}
