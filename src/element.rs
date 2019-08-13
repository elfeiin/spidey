// A rectangle with various visible properties. Kinda like a div, I guess
#[derive(Debug, Clone)]
pub struct Element {
	id: usize,
	tag: String,
	pos: Vec2,
	dim: Vec2,
	color: [f32; 4],
	border: Border,
	text: String,
	text_color: [f32; 4],
}

impl Element {

	pub fn new(id: usize, tag: String) -> Element {
		Element {
			id: id,
			tag: tag,
			pos: Vec2::new(),
			dim: Vec2::new(),
			color: [1.0; 4],
			border: Border::new(),
			text: String::new(),
			text_color: [0.0, 0.0, 0.0, 1.0],
		}
	}
	
	// Of the element struct:
	
	// Returns the id
	pub fn id(&self) -> usize { self.id }
	
	// Retruns the tag
	pub fn tag(&self) -> &String { &self.tag }
	
	// Returns the position
	pub fn pos(&self) -> Vec2 { self.pos }
	
	//Returns the dim
	pub fn dim(&self) -> Vec2 { self.dim }
	
	// Returns the color
	pub fn color(&self) -> [f32; 4] { self.color }
	
	// Returns the border
	pub fn border(&self) -> Border { self.border }
	
	// Returns the text
	pub fn text(&self) -> String { self.text.clone() }
	
	// Returns the text color
	pub fn text_color(&self) -> [f32; 4] { self.text_color }
	
	// Sets the position
	pub fn with_pos(mut self, x: f64, y: f64) -> Self {
		self.pos.x = x;
		self.pos.y = y;
		self
	}
	
	// Sets the x pos alone
	pub fn with_x_pos(mut self, x: f64) -> Self {
		self.pos.x = x;
		self
	}
	
	// Sets the y pos alone
	pub fn with_y_pos(mut self, y: f64) -> Self {
		self.pos.y = y;
		self
	}
	
	// Sets the dimensions
	pub fn with_dim(mut self, x: f64, y: f64) -> Self {
		self.dim.x = x;
		self.dim.y = y;
		self
	}
	
	// Sets the x dim alone
	pub fn with_x_dim(mut self, x: f64) -> Self {
		self.dim.x = x;
		self
	}
	
	// Sets the y dim alone
	pub fn with_y_dim(mut self, y: f64) -> Self {
		self.dim.y = y;
		self
	}
	
	// Sets the color
	pub fn with_color(mut self, color: [f32; 4]) -> Self {
		self.color = color;
		self
	}
	
	// sets which border sides are visible
	pub fn with_border_sides(mut self, top: Option<[f32; 4]>) -> Self {
		self.border.sides = [top, top, top, top];
		self
	}
	
	pub fn with_text(mut self, text: String) -> Self {
		self.text = text;
		self
	}
	
	pub fn with_text_color(mut self, color: [f32; 4]) -> Self {
		self.text_color = color;
		self
	}
	
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
	x: f64,
	y: f64,
}

impl Vec2 {
	
	pub fn new() -> Vec2 {
		Vec2 {
			x: 0f64,
			y: 0f64,
		}
	}
	
	pub fn x(&self) -> f64 {
		self.x
	}
	
	pub fn y(&self) -> f64 {
		self.y
	}
	
}

#[derive(Debug, Copy, Clone)]
pub struct Border {
	sides: [Option<[f32; 4]>; 4], //top bot lef rig
}

impl Border {
	
	pub fn new() -> Border {
		Border {
			sides: [Some([0.0, 0.0, 0.0, 1.0]); 4],
		}
	}
	
	pub fn sides(&self) -> [Option<[f32; 4]>; 4] {
		self.sides
	}
	
}
