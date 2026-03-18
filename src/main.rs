use std::path::Path;

use crate::image::encode_pixel;

mod image;



fn main() {
    let image = image::Image::new(128, 128, encode_pixel(0.5, 0.3, 0.3, 1.0));

    image.write_to_png(Path::new("a.png")).unwrap();
}
