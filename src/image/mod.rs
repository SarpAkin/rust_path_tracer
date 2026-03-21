use std::{fs::File, io::BufWriter, path::Path};

use bytemuck::cast_slice;

pub mod pixel;
pub use pixel::encode_pixel;

pub struct Image {
	width: u32,
	height: u32,
	pixels: Vec<u32>,
}

impl Image {
	pub fn width(&self) -> u32 { self.width }
	pub fn height(&self) -> u32 { self.height }

	pub fn new(width: u32, height: u32, initial_color: u32) -> Self {
		Self {
			width, //
			height,
			pixels: (0..(width * height)).map(|_| initial_color).collect(),
		}
	}

	pub fn from_vec(width: u32, height: u32, pixels:Vec<u32>) -> Self {
		Self {
			width, //
			height,
			pixels
		}
	}

	fn to_index(&self, x: u32, y: u32) -> usize { (x + y * self.width) as usize }

	pub fn set_pixel(&mut self, x: u32, y: u32, pixel: u32) {
		let i = self.to_index(x, y);
		self.pixels[i] = pixel;
	}

	pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
		let i = self.to_index(x, y);
		self.pixels[i]
	}

	pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> &u32 {
		let i = self.to_index(x, y);
		&mut self.pixels[i]
	}

	pub fn write_to_png(&self, path: &Path) -> eyre::Result<()> {
		let file = File::create(path)?;
		let ref mut w = BufWriter::new(file);

		let mut encoder = png::Encoder::new(w, self.width, self.height); // Width is 2 pixels and height is 1.
		encoder.set_color(png::ColorType::Rgba);
		encoder.set_depth(png::BitDepth::Eight);

		let mut writer = encoder.write_header()?;

		writer.write_image_data(cast_slice(&self.pixels))?;

		Ok(())
	}
}
