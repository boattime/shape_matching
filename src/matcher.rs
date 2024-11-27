use crate::{
    error::ShapeMatchError,
    processing,
    types::{ImageData, ImageMatch},
};
use rayon::prelude::*;
use std::collections::HashMap;

/// Main struct for comparing images with in-memory cache
#[derive(Debug)]
pub struct ShapeMatcher {
    images: HashMap<String, ImageData>,
    canny_threshold1: f64,
    canny_threshold2: f64,
}

impl ShapeMatcher {
    pub fn new(image_paths: &[String]) -> Result<Self, ShapeMatchError> {
        let mut library = Self {
            images: HashMap::new(),
            canny_threshold1: 100.0,
            canny_threshold2: 200.0,
        };

        let results: Vec<Result<(String, ImageData), ShapeMatchError>> = image_paths
            .par_iter()
            .map(|path| {
                let contours = processing::process_image(
                    path,
                    library.canny_threshold1,
                    library.canny_threshold2,
                )?;
                Ok((
                    path.clone(),
                    ImageData {
                        contours,
                        path: path.clone(),
                    },
                ))
            })
            .collect();

        for result in results {
            match result {
                Ok((path, data)) => {
                    library.images.insert(path, data);
                }
                Err(e) => eprintln!("Failed to process image: {}", e),
            }
        }

        Ok(library)
    }

    pub fn find_most_similar(&self, source_path: &str) -> Result<ImageMatch, ShapeMatchError> {
        let source_contours =
            processing::process_image(source_path, self.canny_threshold1, self.canny_threshold2)?;

        self.images
            .par_iter()
            .map(|(_, image_data)| {
                let similarity =
                    processing::compare_shapes(&source_contours, &image_data.contours)?;
                Ok::<ImageMatch, ShapeMatchError>(ImageMatch {
                    path: image_data.path.clone(),
                    similarity,
                })
            })
            .filter_map(Result::ok)
            .min_by(|a, b| a.similarity.partial_cmp(&b.similarity).unwrap())
            .ok_or(ShapeMatchError::NoMatchesFound)
    }

    pub fn add_image(&mut self, path: String) -> Result<(), ShapeMatchError> {
        let contours =
            processing::process_image(&path, self.canny_threshold1, self.canny_threshold2)?;
        self.images
            .insert(path.clone(), ImageData { contours, path });
        Ok(())
    }

    pub fn remove_image(&mut self, path: &str) {
        self.images.remove(path);
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }

    pub fn get_image_paths(&self) -> Vec<String> {
        self.images.keys().cloned().collect()
    }
}
