use super::pointer::*;
// The Canvas struct. Has width and height. Has [rgba] buffer.
pub struct Canvas {
	width: isize,
	height: isize,
	buffer: Vec<[u8;4]>,
}
impl Canvas {
	pub fn new(width: isize, height: isize) -> Self {
		Canvas {
			width,
			height,
			buffer: vec![[0u8;4];width.abs() as usize * height.abs() as usize],
		}
	}
	// Set some 2d location in the buffer to given [rgba]
	pub fn put(&mut self, p: &Pointer, hex: [u8;4]) -> &Self {
		let index = p.y() as usize * self.width() as usize + p.x() as usize;
		self.buffer[index] = hex;
		self
	}
	// Resets the canvas buffer
	pub fn blank(&mut self) -> &Self {
		self.buffer = vec![[0u8;4];self.width() as usize * self.height() as usize];
		self
	}
	// These two change the width and height of the canvas, with a &Pointer to update in addition
	pub fn set_width(&mut self, p: &mut Pointer, w: usize) -> &Self {
		let mut w = w;
		if w == 0 {
			w = 1;
		}
		let mut new_buffer = vec!();
		for i in (0..self.height() as usize).step_by(w) {
			for d in 0..w {
				new_buffer.push(self.buffer[i+d]);
			}
		}
		self.buffer = new_buffer.to_vec();
		self.width = w as isize;
		p.blank(self);
		self
	}
	pub fn set_height(&mut self, p: &mut Pointer, h: usize) -> &Self {
		let mut h = h;
		if h == 0 {
			h = 1;
		}
		self.buffer.resize(h*self.width() as usize, [0u8;4]);
		self.height = h as isize;
		p.blank(self);
		self
	}
	pub fn width(&self) -> isize {
		self.width
	}
	pub fn height(&self) -> isize {
		self.height
	}
	pub fn buffer(&self) -> &Vec<[u8;4]> {
		&self.buffer
	}
}