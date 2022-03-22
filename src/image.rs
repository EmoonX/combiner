use image::{io::Reader, DynamicImage, ImageFormat};

/// Read image from path and return both the image data and its format.
pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    // Build reader by opening image
    let reader = Reader::open(path).unwrap();

    // `.format()` must be called first, as `.decode()` moves `reader`
    let format = reader.format().unwrap();
    let image = reader.decode().unwrap();

    (image, format)
}
