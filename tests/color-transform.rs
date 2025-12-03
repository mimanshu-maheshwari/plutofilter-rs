#[cfg(feature = "image")]
mod brightness_test;
#[cfg(feature = "image")]
mod contrast_test;
#[cfg(feature = "image")]
mod grayscale_test;
#[cfg(feature = "image")]
mod hue_rotate_test;
#[cfg(feature = "image")]
mod invert_test;
#[cfg(feature = "image")]
mod opacity_test;
#[cfg(feature = "image")]
mod saturate_test;
#[cfg(feature = "image")]
mod sepia_test;

#[cfg(test)]
#[cfg(feature = "image")]
mod color_transform_test {
    use plutofilter_rs::{ImageEditor, get_resource_path};

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn color_transform_test_original() -> Result<()> {
        const ORIGINAL: [f32; 20] = [
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
        ];
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform(ORIGINAL);
        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "original.png");
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn color_transform_test_grayscale() -> Result<()> {
        const GRAYSCALE: [f32; 20] = [
            0.2126, 0.7152, 0.0722, 0.0, 0.0, 0.2126, 0.7152, 0.0722, 0.0, 0.0, 0.2126, 0.7152,
            0.0722, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        ];

        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform(GRAYSCALE);
        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "grayscale.png");
        editor.save_to(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_test_sepia() -> Result<()> {
        const SEPIA: [f32; 20] = [
            0.393, 0.769, 0.189, 0.0, 0.0, 0.349, 0.686, 0.168, 0.0, 0.0, 0.272, 0.534, 0.131, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        ];

        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform(SEPIA);
        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "sepia.png");
        editor.save_to(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }

    #[test]
    fn color_transform_test_contrast() -> Result<()> {
        const CONTRAST: [f32; 20] = [
            1.75, 0.0, 0.0, 0.0, -0.375, 0.0, 1.75, 0.0, 0.0, -0.375, 0.0, 0.0, 1.75, 0.0, -0.375,
            0.0, 0.0, 0.0, 1.0, 0.0,
        ];
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        editor = editor.color_transform(CONTRAST);
        let output_path =
            get_resource_path(&["test_output_images", "color_transform"], "contrast.png");
        editor.save_to(output_path)?;
        // eprintln!("Saved files");
        Ok(())
    }
}
