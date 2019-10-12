use std::ops::Add; use std::ops::Sub; use std::ops::AddAssign;

#[derive(Debug, Copy, Clone)]
pub enum Color {
	Red,
	Yellow,
	Green,
	Cyan,
	Blue,
	Magenta,
	Black,
	White,
	Unset,
	Color4,
}

impl Color {
	pub fn from_u8(n: u8) -> Color {
		match n {
			0 => Color::Red,
			1 => Color::Yellow,
			2 => Color::Green,
			3 => Color::Cyan,
			4 => Color::Blue,
			5 => Color::Magenta,
			6 => Color::White,
			7 => Color::Black,
			_ => Color::Red,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Tone {
	Dark,
	Normal,
	Light,
}

impl Tone {
	pub fn from_u8(n: u8) -> Tone {
		match n {
			0 => Tone::Dark,
			1 => Tone::Normal,
			2 => Tone::Light,
			_ => Tone::Normal,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
	pub x: usize,
	pub y: usize,
}

impl Add for Vec2 {
	type Output = Self;
	
	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl Sub for Vec2 {
	type Output = Self;
	
	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl AddAssign for Vec2 {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

pub enum Event {
	PutPrimary(Vec2),
	PutSecondary(Vec2),
	RollPrimary(Direction), // Roll primary the given direction
	RollSecondary(Direction),
	ChangePrimaryColor4(Color4),
	ChangeSecondaryColor4(Color4),
	SwapAries,
	Resize(Vec2),
	Move(Vec2),
	ChangeDefault((Color, Tone)),
	ChangeName(String),
	ChangeSaveLocation(String),
	Save(Option<Format>), // Save file with format. If None, then tries to guess or defaults to .bmp (but preveserves name)
}

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

pub type Color4 = [f64; 4];

pub enum CanvasError {
	OutOfBounds,
	SaveLocationNotSet,
	SaveNameNotSet,
	FileExists,
	DirectoryDoesNotExist,
}

pub enum Format {
	BMP,
	PNG,
	TIF,
	TIFF,
	GIF,
	JPG
}