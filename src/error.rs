use std::fmt::Display;

#[derive(Debug)]
pub enum SurfaceError {
    InvalidStrideOrWidth,
    InvalidPixelLength,
}

impl Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SurfaceError::*;
        match self {
            InvalidStrideOrWidth => write!(f, "Stride should be greater than the width"),
            InvalidPixelLength => write!(f, "Number of pixels should be more than stride * height"),
        }
    }
}
