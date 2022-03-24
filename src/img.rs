use image::{io::Reader, DynamicImage, ImageFormat, GenericImageView, imageops::FilterType::Triangle};

/// Enum holding image data error type.
#[derive(Debug)]
pub enum ImageDataErrors {
    DifferentImageFormats
}  

/// Read image from path and return both the image data and its format.
pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    // Build reader by opening image
    let reader = Reader::open(path).unwrap();

    // `.format()` must be called first, as `.decode()` moves `reader`
    let format = reader.format().unwrap();
    let image = reader.decode().unwrap();

    (image, format)
}

/// Resize biggest image to fit smallest's size.
pub fn standardize_size(image_1: &DynamicImage, image_2: &DynamicImage) {
    let smallest = get_smallest_image(image_1, image_2);
    if smallest == image_1 {
        image_2.resize_exact(
            smallest.width(), smallest.height(), Triangle);
    } else {
        image_1.resize_exact(
            smallest.width(), smallest.height(), Triangle);
    }
}

/// Compare images by total number of pixels.
/// Return reference to smallest one.
fn get_smallest_image<'a>(image_1: &'a DynamicImage, image_2: &'a DynamicImage)
        -> &'a DynamicImage {
    let (dim_1, dim_2) = (image_1.dimensions(), image_2.dimensions());
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    if pix_1 < pix_2 { image_1 } else { image_2 }
}