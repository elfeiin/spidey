#[derive(Debug, Copy, Clone)]
pub struct Pyxel {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
	x: f64,
	y: f64,
}
impl Pyxel {
	pub fn new(color: [u8;4], x: f64, y: f64) -> Pyxel {
		Pyxel {
			r: color[0],
			g: color[1],
			b: color[2],
			a: color[3],
			x: x,
			y: y,
			
		}
	}
	pub fn color_u8(&self) -> [u8;4] {
		[self.r(),self.g(),self.b(),self.a()]
	}
	pub fn color_f32(&self) -> [f32;4] {
		[
			self.r() as f32/255.0,
			self.g() as f32/255.0,
			self.b() as f32/255.0,
			self.a() as f32/255.0,
		]
	}
	pub fn r(&self) -> u8 {
		self.r
	}
	pub fn g(&self) -> u8 {
		self.g
	}
	pub fn b(&self) -> u8 {
		self.b
	}
	pub fn a(&self) -> u8 {
		self.a
	}
	pub fn x(&self) -> f64 {
		self.x
	}
	pub fn y(&self) -> f64 {
		self.y
	}
}