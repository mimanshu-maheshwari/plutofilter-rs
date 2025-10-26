use image::{GenericImageView, ImageBuffer, ImageReader, Rgba};
use plutofilter_rs::*;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let base_file = "/res/original_images/zhang-hanyn.jpg";
    let base_image = ImageReader::open(base_file)
        .map_err(|e| {
            println!("{base_file} not found, {e}");
            e
        })?
        .decode()?;
    let (base_image_width, base_image_height) = base_image.dimensions();
    let mut base_image_raw = base_image
        .into_rgba32f()
        .as_raw()
        .iter()
        .map(|&p| p as u32)
        .collect::<Vec<_>>();
    let mut base_image_surface = Surface::make(
        base_image_raw.as_mut_slice(),
        base_image_width as usize,
        base_image_height as usize,
        base_image_width as usize,
    )?;

    let blend_image = "/res/original_images/royal-purple.png";
    let blend_image = ImageReader::open(blend_image)
        .map_err(|e| {
            println!("{blend_image} not found, {e}");
            e
        })?
        .decode()?;
    let (blend_image_width, blend_image_height) = blend_image.dimensions();
    let mut blend_image_raw = blend_image
        .into_rgba32f()
        .as_raw()
        .iter()
        .map(|&p| p as u32)
        .collect::<Vec<_>>();
    let mut blend_image_surface = Surface::make(
        blend_image_raw.as_mut_slice(),
        blend_image_width as usize,
        blend_image_height as usize,
        blend_image_width as usize,
    )?;
    let mut output_pixels = vec![base_image_width * base_image_height];
    let output_pixels = output_pixels.as_mut_slice();
    let mut output_surface = Surface::make(
        output_pixels,
        base_image_width as usize,
        base_image_height as usize,
        base_image_width as usize,
    )?;
    Surface::blend(
        &mut base_image_surface,
        &mut blend_image_surface,
        &mut output_surface,
        BlendMode::Normal,
    );
    let mut output_image = ImageBuffer::<Rgba<u8>, _>::new(base_image_width, base_image_height);
    let output_pixels = base_image_surface
        .pixels()
        .iter()
        .cloned()
        .map(|p| {
            Rgba([
                ((p >> 24) & 0xFF) as u8,
                ((p >> 16) & 0xFF) as u8,
                ((p >> 8) & 0xFF) as u8,
                (p & 0xFF) as u8,
            ])
        })
        .collect::<Vec<_>>();
    for (y, x, pixel) in output_image.enumerate_pixels_mut() {
        *pixel = output_pixels[(y * base_image_width + x) as usize];
    }
    output_image.save("/res/test_output_images/blend_normal.jpg")?;
    Ok(())
}
