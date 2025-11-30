//! PlutoFilter implementation

// TODO: input output issue ?

mod error;
mod surface;
mod utils;
pub use arena::ImageEditor;
pub use surface::{BlendMode, CompositeOperator, Surface};
pub use utils::get_resource_path;

pub mod arena {
    use std::{
        panic,
        path::{Path, PathBuf},
    };

    use image::{DynamicImage, ImageBuffer, ImageReader, ImageResult};

    use crate::{BlendMode, CompositeOperator, Surface};

    #[derive(Debug)]
    pub struct ImageEditor {
        input_image_path: PathBuf,
        input_image: DynamicImage,
        output_image: DynamicImage,
    }

    // TODO: Split this impl using typesafe builder pattern
    impl ImageEditor {
        // TODO: change to result type
        pub fn open(input_image_path: impl AsRef<Path>) -> Self {
            let input_image_path = input_image_path.as_ref().to_owned();
            let input_image = Self::open_image(&input_image_path);
            let output_image = Self::temp_output_image(input_image.width(), input_image.height());
            Self {
                input_image_path,
                input_image,
                output_image,
            }
        }

        fn temp_output_image(width: u32, height: u32) -> DynamicImage {
            let output_pixels =
                ImageBuffer::from_vec(width, height, vec![0u8; (4 * width * height) as usize])
                    .unwrap();
            DynamicImage::ImageRgba8(output_pixels)
        }

        // TODO: remove panics
        fn open_image(image_path: &Path) -> DynamicImage {
            if !image_path.exists() {
                panic!("Unable to find image path.");
            }
            let image_buffer = match ImageReader::open(image_path) {
                Ok(image) => match image.decode() {
                    Ok(image) => image,
                    Err(err) => panic!("Unable to decode image: {err}"),
                },
                Err(err) => panic!("Unable to open Image: {err}"),
            };
            let rgba8_image_buffer = image_buffer.into_rgba8();
            DynamicImage::ImageRgba8(rgba8_image_buffer)
        }

        pub fn save_to(self, output_path: impl AsRef<Path>) -> ImageResult<()> {
            let output_path = output_path.as_ref();
            self.output_image.save(output_path)
        }

        pub fn save(self) -> ImageResult<()> {
            self.output_image.save(&self.input_image_path)
        }

        pub fn color_transform(mut self, matrix: [f32; 20]) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform(&mut input_surface, &mut output_surface, matrix);
            self
        }

        pub fn color_transform_opacity(mut self, amount: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_opacity(&mut input_surface, &mut output_surface, amount);
            self
        }

        pub fn color_transform_brightness(mut self, amount: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_brightness(&mut input_surface, &mut output_surface, amount);
            self
        }

        pub fn color_transform_invert(mut self, amount: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_invert(&mut input_surface, &mut output_surface, amount);
            self
        }

        pub fn color_transform_contrast(mut self, amount: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_contrast(&mut input_surface, &mut output_surface, amount);
            self
        }

        pub fn color_transform_saturate(mut self, amount: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_saturate(&mut input_surface, &mut output_surface, amount);
            self
        }

        pub fn color_transform_grayscale(mut self, amount: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_grayscale(&mut input_surface, &mut output_surface, amount);
            self
        }

        pub fn color_transform_sepia(mut self, amount: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_sepia(&mut input_surface, &mut output_surface, amount);
            self
        }

        pub fn color_transform_hue_rotate(mut self, angle: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_hue_rotate(&mut input_surface, &mut output_surface, angle);
            self
        }

        pub fn color_transform_luminance_to_alpha(mut self) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_luminance_to_alpha(&mut input_surface, &mut output_surface);
            self
        }

        pub fn color_transform_srgb_to_linear_rgb(mut self) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_srgb_to_linear_rgb(&mut input_surface, &mut output_surface);
            self
        }

        pub fn color_transform_linear_rgb_to_srgb(mut self) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::color_transform_linear_rgb_to_srgb(&mut input_surface, &mut output_surface);
            self
        }

        pub fn gaussian_blur(mut self, std_deviation_x: f32, std_deviation_y: f32) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);
            Surface::gaussian_blur(
                &mut input_surface,
                &mut output_surface,
                std_deviation_x,
                std_deviation_y,
            );
            self
        }

        pub fn blend(mut self, blend_image: impl AsRef<Path>, mode: BlendMode) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);

            let mut blend_input_image = Self::open_image(blend_image.as_ref());
            let mut blend_surface = Surface::from_image(&mut blend_input_image);
            Surface::blend(
                &mut input_surface,
                &mut blend_surface,
                &mut output_surface,
                mode,
            );
            self
        }

        pub fn composite(
            mut self,
            composite_image: impl AsRef<Path>,
            op: CompositeOperator,
        ) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);

            let mut composite_input_image = Self::open_image(composite_image.as_ref());
            let mut composite_surface = Surface::from_image(&mut composite_input_image);
            Surface::composite(
                &mut input_surface,
                &mut composite_surface,
                &mut output_surface,
                op,
            );
            self
        }

        pub fn composite_arithmetic(
            mut self,
            composite_image: impl AsRef<Path>,
            k1: f32,
            k2: f32,
            k3: f32,
            k4: f32,
        ) -> Self {
            let mut input_surface = Surface::from_image(&mut self.input_image);
            let mut output_surface = Surface::from_image(&mut self.output_image);

            let mut composite_input_image = Self::open_image(composite_image.as_ref());
            let mut composite_surface = Surface::from_image(&mut composite_input_image);
            Surface::composite_arithmetic(
                &mut input_surface,
                &mut composite_surface,
                &mut output_surface,
                k1,
                k2,
                k3,
                k4,
            );
            self
        }
    }
}
