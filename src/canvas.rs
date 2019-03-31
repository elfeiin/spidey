use super::pointer::*;
// The Canvas struct. Has width and height. Has [rgba] buffer.
pub struct Canvas {
	pub width: isize,
	pub height: isize,
	pub buffer: Vec<Vec<[u8;4]>>,
}
impl Canvas {
	// Set some 2d location in the buffer to given [rgba]
	pub fn put(&mut self, p: &Pointer, hex: [u8;4]) -> &Self {
		self.buffer[p.y as usize][p.x as usize] = hex;
		self
	}
	// Resets the canvas buffer
	pub fn blank(&mut self) -> &Self {
		self.buffer = vec![vec![[0u8;4];self.width.abs() as usize];self.height.abs() as usize];
		self
	}
	// These two change the width and height of the canvas, with a &Pointer to update in addition
	pub fn width(&mut self, p: &mut Pointer, w: usize) -> &Self {
		if w == 0 {
			let w = 1;
		}
		self.width = w as isize;
		for v in self.buffer.iter_mut() {
			v.resize(w, [0u8;4]);
		}
		p.blank(self);
		self
	}
	pub fn height(&mut self, p: &mut Pointer, h: usize) -> &Self {
		if h == 0 {
			let h = 1;
		}
		self.height = h as isize;
		self.buffer.resize(h, vec![[0u8;4];self.width as usize]);
		p.blank(self);
		self
	}
}
pub fn new(w: isize, h: isize) -> Canvas {
	Canvas {
		width: w,
		height: h,
		buffer: vec![vec![[0u8;4];w.abs() as usize];h.abs() as usize],
	}
}