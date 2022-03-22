mod args;
mod image;
use args::Args;
use crate::image::find_image_from_path;

fn main() {
    let args = Args::new();
    println!("{:?}", args);

    let (image_1, image_1_format) =
        find_image_from_path(args.image_1);
    let (image_2, image_2_format) =
        find_image_from_path(args.image_2); 
}
