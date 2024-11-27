use crate::error::ShapeMatchError;
use opencv::{core, imgproc, prelude::*};

pub(crate) fn process_image(
    path: &str,
    canny_threshold1: f64,
    canny_threshold2: f64,
) -> Result<core::Vector<core::Vector<core::Point>>, ShapeMatchError> {
    let img = load_image(path)?;

    // Convert to grayscale if needed
    let mut gray = Mat::default();
    if img.channels() > 1 {
        imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    } else {
        gray = img;
    }

    // Apply Gaussian blur
    let mut blurred = Mat::default();
    imgproc::gaussian_blur(
        &gray,
        &mut blurred,
        core::Size::new(5, 5),
        0.0,
        0.0,
        core::BORDER_DEFAULT,
    )?;

    // Apply Canny edge detection
    let mut edges = Mat::default();
    imgproc::canny(
        &blurred,
        &mut edges,
        canny_threshold1,
        canny_threshold2,
        3,
        false,
    )?;

    // Find contours
    let mut contours = core::Vector::<core::Vector<core::Point>>::new();
    imgproc::find_contours(
        &edges,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::new(0, 0),
    )?;

    Ok(contours)
}

fn load_image(path: &str) -> Result<Mat, ShapeMatchError> {
    let img = opencv::imgcodecs::imread(path, opencv::imgcodecs::IMREAD_GRAYSCALE)?;

    if img.empty() {
        return Err(ShapeMatchError::ImageLoadError(
            image::ImageError::Unsupported(image::error::UnsupportedError::from_format_and_kind(
                image::error::ImageFormatHint::Unknown,
                image::error::UnsupportedErrorKind::Format(image::error::ImageFormatHint::Unknown),
            )),
        ));
    }

    Ok(img)
}

pub(crate) fn compare_shapes(
    contours1: &core::Vector<core::Vector<core::Point>>,
    contours2: &core::Vector<core::Vector<core::Point>>,
) -> Result<f64, ShapeMatchError> {
    if contours1.is_empty() || contours2.is_empty() {
        return Ok(f64::INFINITY);
    }

    let largest_contour1 = find_largest_contour(contours1)?;
    let largest_contour2 = find_largest_contour(contours2)?;

    Ok(imgproc::match_shapes(
        &largest_contour1,
        &largest_contour2,
        imgproc::CONTOURS_MATCH_I1,
        0.0,
    )?)
}

fn find_largest_contour(
    contours: &core::Vector<core::Vector<core::Point>>,
) -> Result<core::Vector<core::Point>, ShapeMatchError> {
    let mut largest_area = 0.0;
    let mut largest_idx = 0;

    for i in 0..contours.len() {
        let area = imgproc::contour_area(&contours.get(i)?, false)?;
        if area > largest_area {
            largest_area = area;
            largest_idx = i;
        }
    }

    if contours.is_empty() {
        return Ok(core::Vector::new());
    }

    Ok(contours.get(largest_idx)?)
}
