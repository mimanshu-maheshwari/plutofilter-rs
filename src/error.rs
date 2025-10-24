use std::fmt::Display;

#[derive(Debug)]
pub enum SurfaceError {
    StrideLessThanWidth,
    InvalidPixelLength,
    InvalidStride,
    InvalidWidth,
    InvalidHeight,
}

impl Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SurfaceError::*;
        match self {
            StrideLessThanWidth => write!(f, "Stride should be greater than the width"),
            InvalidPixelLength => write!(f, "Number of pixels should be more than stride * height"),
            InvalidHeight => write!(f, "Invalid Height"),
            InvalidStride => write!(f, "Invalid width"),
            InvalidWidth => write!(f, "Invalid height"),
        }
    }
}
