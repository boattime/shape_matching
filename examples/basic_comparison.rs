use shape_matching::ShapeMatcher;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;

    let make_path = |name: &str| -> String {
        current_dir
            .join("examples")
            .join("images")
            .join(name)
            .to_string_lossy()
            .into_owned()
    };

    // Initialize with some test images
    let image_paths = vec![
        make_path("circle.png"),
        make_path("star.png"),
        make_path("square.png"),
        make_path("triangle.png"),
    ];

    // Create and initialize the library
    println!("Initializing image library...");
    let matcher = ShapeMatcher::new(&image_paths)?;
    println!("Loaded {} images into matcher", matcher.image_count());

    // Try to find the most similar image
    let source_image = make_path("test_star.png");
    println!("\nFinding most similar image to: {}", source_image);

    match matcher.find_most_similar(&source_image) {
        Ok(match_result) => {
            println!("\nFound best match:");
            println!("Path: {}", match_result.path);
            println!("Similarity score: {:.4}", match_result.similarity);
        }
        Err(e) => eprintln!("Error finding match: {}", e),
    }

    Ok(())
}
