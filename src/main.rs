mod args;
mod img;

use args::Args;
use crate::img::{
    find_image_from_path, standardize_size, get_image_dimensions,
    FloatingImage, ImageDataErrors
};

fn main() -> Result<(), ImageDataErrors> {
    // Print command line arguments
    let args = Args::new();
    println!("{:?}", args);

    // Read images from arg paths
    let (image_1, format_1) = find_image_from_path(args.image_1);
    let (image_2, format_2) = find_image_from_path(args.image_2);
    
    // Assert both images have the same format
    if format_1 != format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    // Make both images have equal size
    standardize_size(&image_1, &image_2);

    // Create output image object from dimensions and name
    let (width, height) = get_image_dimensions(image_1);
    let output = FloatingImage::new(width, height, args.output);
    
    Ok(())
}
