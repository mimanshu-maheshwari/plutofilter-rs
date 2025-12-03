#[cfg(test)]
#[cfg(feature = "image")]
mod color_transform_test {
    use plutofilter_rs::{CompositeOperator, ImageEditor, get_resource_path};
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn composite_over() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");

        editor = editor.composite(blend_image, CompositeOperator::Over);
        let output_path = get_resource_path(&["test_output_images", "composite"], "over.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn composite_in() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");

        editor = editor.composite(blend_image, CompositeOperator::In);
        let output_path = get_resource_path(&["test_output_images", "composite"], "in.png");
        editor.save_to(output_path)?;

        Ok(())
    }

    #[test]
    fn composite_out() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
        editor = editor.composite(blend_image, CompositeOperator::Out);
        let output_path = get_resource_path(&["test_output_images", "composite"], "out.png");
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_atop() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
        let output_path = get_resource_path(&["test_output_images", "composite"], "atop.png");
        editor = editor.composite(blend_image, CompositeOperator::Atop);
        editor.save_to(output_path)?;
        Ok(())
    }

    #[test]
    fn composite_xor() -> Result<()> {
        let base_file = get_resource_path(&["original_images"], "zhang-hanyun.jpg");
        let output_path = get_resource_path(&["test_output_images", "composite"], "xor.png");
        let mut editor = ImageEditor::open(base_file);
        let blend_image = get_resource_path(&["original_images"], "firebrick-circle.png");
        editor = editor.composite(blend_image, CompositeOperator::Xor);
        editor.save_to(output_path)?;
        Ok(())
    }
}
