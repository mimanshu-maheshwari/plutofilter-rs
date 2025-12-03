#[cfg(test)]
#[cfg(feature = "image")]
mod blend_modes_tests {
    use plutofilter_rs::{BlendMode, ImageEditor, get_resource_path};
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Test Blend mode `Normal`
    #[test]
    fn test_normal() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Normal);
        let output_path = get_resource_path(&["test_output_images", "blend"], "normal.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `Multiply`
    #[test]
    fn test_multiply() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Multiply);
        let output_path = get_resource_path(&["test_output_images", "blend"], "multiply.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `Screen`
    #[test]
    fn test_screen() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Screen);
        let output_path = get_resource_path(&["test_output_images", "blend"], "screen.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `Overlay`
    #[test]
    fn test_overlay() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Overlay);
        let output_path = get_resource_path(&["test_output_images", "blend"], "overlay.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `Darken`
    #[test]
    fn test_darken() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Darken);
        let output_path = get_resource_path(&["test_output_images", "blend"], "darken.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `Lighten`
    #[test]
    fn test_lighten() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Lighten);
        let output_path = get_resource_path(&["test_output_images", "blend"], "lighten.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `ColorDodge`
    #[test]
    fn test_color_dodge() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::ColorDodge);
        let output_path = get_resource_path(&["test_output_images", "blend"], "color_dodge.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `ColorBurn`
    #[test]
    fn test_color_burn() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::ColorBurn);
        let output_path = get_resource_path(&["test_output_images", "blend"], "color_burn.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `HardLight`
    #[test]
    fn test_hard_light() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::HardLight);
        let output_path = get_resource_path(&["test_output_images", "blend"], "hard_light.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `SoftLight`
    #[test]
    fn test_soft_light() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::SoftLight);
        let output_path = get_resource_path(&["test_output_images", "blend"], "soft_light.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `Difference`
    #[test]
    fn test_difference() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Difference);
        let output_path = get_resource_path(&["test_output_images", "blend"], "difference.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    /// Test Blend mode `Exclusion`
    #[test]
    fn test_exclusion() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "royal-purple.png");

        editor = editor.blend(blend_image, BlendMode::Exclusion);
        let output_path = get_resource_path(&["test_output_images", "blend"], "exclusion.png");
        editor.save_to(output_path)?;

        Ok(())
    }
}
