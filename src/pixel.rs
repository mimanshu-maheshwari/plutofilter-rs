#[repr(transparent)]
pub struct Pixel(u32);

impl From<u32> for Pixel {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Pixel> for u32 {
    fn from(value: Pixel) -> Self {
        value.0
    }
}
