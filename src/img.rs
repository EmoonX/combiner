use image::{
    DynamicImage, ImageFormat, GenericImageView,
    io::Reader, imageops::FilterType, ColorType
};

/// Represents a floating image with specified size, data and name.
/// 
/// Data is represented as a vector buffer of `u8`s.
pub struct FloatingImage {
        width:  u32,
        height: u32,      
        data:   Vec<u8>,
    pub name:   String
} 

/// Enum holding image data error type.
#[derive(Debug)]
pub enum ImageDataErrors {
    /// Image format mismatch (e.g a .jpg and .png for input).
    DifferentImageFormats,

    /// Output data is bigger than buffer's capacity (956 * 956 * 4 bytes).
    BufferTooSmall
}  

/// Read image from path and return both the image data and its format.
pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    // Build reader by opening image
    let reader = match Reader::open(&path) {
        Ok(r) => r,
        Err(_) => panic!("Problem opening file: {}", path)
    };
    // `.format()` must be called first, as `.decode()` moves `reader`
    let format = reader.format().unwrap();
    let image = reader.decode().unwrap();
    (image, format)
}

/// Resize biggest image to fit smallest's size.
pub fn standardize_size(image_1: &DynamicImage, image_2: &DynamicImage) {
    let smallest = get_smallest_image(image_1, image_2);
    let width = smallest.width();
    let height = smallest.height();
    let filter = FilterType::Triangle;
    if smallest == image_1 {
        image_2.resize_exact(width, height, filter);
    } else {
        image_1.resize_exact(width, height, filter);
    }
}

/// Compare images by total number of pixels.
/// 
/// Return reference to the smallest one.
fn get_smallest_image<'a>(image_1: &'a DynamicImage, image_2: &'a DynamicImage)
        -> &'a DynamicImage {
    let (dim_1, dim_2) = (image_1.dimensions(), image_2.dimensions());
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    if pix_1 < pix_2 { image_1 } else { image_2 }
}

/// Get image dimensions in a (width, height) tuple
pub fn get_image_dimensions(image: &DynamicImage) -> (u32, u32) {
    (image.width(), image.height())
}

impl FloatingImage {
    /// Creates a new `FloatingImage`with underlying data buffer.
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = 956 * 956 * 4;
        let buffer = Vec::with_capacity(buffer_capacity);
        FloatingImage { width, height, data: buffer, name }
    }

    /// Move given data to data buffer.
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }

    /// Save output image with given format and attributes.
    pub fn write_to_file(&mut self, format: ImageFormat) {
        image::save_buffer_with_format(
            &self.name, &self.data, self.width, self.height,
            ColorType::Rgba8, format
        ).unwrap();
    }
}

/// Combine two images by using the alternating pixels algorithm.
/// 
/// Return a `Vec<u8>` containing result's data.
pub fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();
    alternate_pixels(vec_1, vec_2)
}

/// Build a vector of RGBA pixels by picking alternating values from
/// given vectors (0..=3 from the first one, 4..=7 from the second,
/// 8..=11 from the third, and so on).
fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0; vec_1.len()];
    for i in (0..vec_1.len()).step_by(4) {
        for j in i..=(i+3) {
            combined_data[j] = {
                if i % 8 == 0 { vec_1[j] } else { vec_2[j] }
            };
        }
    }
    combined_data
}
