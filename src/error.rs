use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShapeMatchError {
    #[error("Failed to load image: {0}")]
    ImageLoadError(#[from] image::ImageError),

    #[error("OpenCV operation failed: {0}")]
    OpenCvError(#[from] opencv::Error),

    #[error("No valid matches found")]
    NoMatchesFound,

    #[error("Image not found in cache: {0}")]
    ImageNotCached(String),
}
