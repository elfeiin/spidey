// The Pyxel. Purposely spelled with a Y to differentiate it from normal pixels.
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
	// Returns a new Pyxel. Takes an array of 4 u8s and two f64s as args
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
	
	// Returns the channels formatted as an array of u8
	pub fn color_u8(&self) -> [u8;4] {
		[self.r(),self.g(),self.b(),self.a()]
	}
	
	// Returns the channels formatted as an array of f32
	pub fn color_f32(&self) -> [f32;4] {
		[
			self.r() as f32/255.0,
			self.g() as f32/255.0,
			self.b() as f32/255.0,
			self.a() as f32/255.0,
		]
	}
	
	// Returns respective values
	pub fn r(&self) -> u8 { self.r }
	
	pub fn g(&self) -> u8 { self.g }
	
	pub fn b(&self) -> u8 { self.b }
	
	pub fn a(&self) -> u8 { self.a }
	
	pub fn x(&self) -> f64 { self.x }
	
	pub fn y(&self) -> f64 { self.y }
	
}