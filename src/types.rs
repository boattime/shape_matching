use opencv::core;

/// Represents a matched image with its similarity score
#[derive(Debug, Clone)]
pub struct ImageMatch {
    /// Path to the matched image
    pub path: String,
    /// Similarity score (lower means more similar)
    pub similarity: f64,
}

/// Cached image data containing processed contours
#[derive(Debug)]
pub(crate) struct ImageData {
    pub contours: core::Vector<core::Vector<core::Point>>,
    pub path: String,
}
