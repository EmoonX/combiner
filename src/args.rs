use std::env::args;
use std::process;

/// Holds input and output images' paths as `String`s.
#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String
}  

impl Args {
    /// Get three args from command line (skip 0th, i.e program name).
    ///
    /// Execution is aborted on wrong number of arguments.
    pub fn new() -> Self {
        if args().len() != 4 {
            println!("Usage: combiner first_image second_image output");
            println!("   Ex: combiner images/fcc_glyph.png \
                images/pro.png images/output.png");
            process::exit(1);
        }
        Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3)
        }
    }
}

/// Get nth argument from `args()` array.
fn get_nth_arg(n: usize) -> String {
    args().nth(n).unwrap()
}
