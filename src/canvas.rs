use super::types::*;
use std::fs::File;
use std::io::prelude::*;

pub struct Canvas {
	name: Option<String>, // What we save the file as
	location: Option<String>, // Where we save the file
	dim: Vec2, // How big a canvas is
	pos: Vec2, // For drawing poirposes
	primary: (Color, Tone), // What color gets put on left click (or primary button)
	primary_color4: Color4,
	secondary: (Color, Tone), // Opposite of that ^
	secondary_color4: Color4,
	r#default: (Color, Tone), // What gets put on unset pyxel when the canvas resizes
	pyxels: Vec<Vec<(Color, Tone)>>, // A 2D array that contains all the pyxel data
}

impl Canvas {
	pub fn event(&mut self, event: Event) -> Result<(), CanvasError> {
		match event {
			Event::PutPrimary(v) => {
				self.put(false, v)
			},
			Event::PutSecondary(v) => {
				self.put(true, v)
			},
			Event::RollPrimary(dir) => {
				self.roll(false, dir);
				Ok(())
			},
			Event::RollSecondary(dir) => {
				self.roll(true, dir);
				Ok(())
			},
			Event::ChangePrimaryColor4(c) => {
				self.change_color4(false, c);
				Ok(())
			},
			Event::ChangeSecondaryColor4(c) => {
				self.change_color4(true, c);
				Ok(())
			},
			Event::SwapAries => {
				self.swap_aries();
				Ok(())
			},
			Event::Resize(v) => {
				self.resize(v);
				Ok(())
			},
			Event::Move(v) => {
				self.r#move(v);
				Ok(())
			},
			Event::ChangeDefault(t) => {
				self.change_default(t);
				Ok(())
			},
			Event::ChangeName(s) => {
				self.change_name(s);
				Ok(())
			},
			Event::ChangeSaveLocation(s) => {
				self.change_location(s);
				Ok(())
			},
			Event::Save(o) => {
				self.save(o)
			}
		}
	}
	
	fn put(&mut self, second: bool, pos: Vec2) -> Result<(), CanvasError> {
		if pos.x <= self.dim.x || pos.y <= self.dim.y {
			let (x, y) = (pos.x as usize, pos.y as usize);
			if self.pyxels.len() > x {
				if self.pyxels[x].len() > y {
					self.pyxels[x][y] = if second { self.secondary } else { self.primary };
					return Ok(());
				}
			}
		}
		Err(CanvasError::OutOfBounds)
	}
	
	fn roll(&mut self, second: bool, dir: Direction) {
		let to_roll = if second { &mut self.secondary } else { &mut self.primary };
		match dir {
			Direction::Up => {
				*to_roll = (Color::from_u8(to_roll.0 as u8 + 1), to_roll.1);
			},
			Direction::Down => {
				*to_roll = (Color::from_u8((to_roll.0 as u8).wrapping_sub(1)), to_roll.1);
			},
			Direction::Left => {
				*to_roll = (to_roll.0, Tone::from_u8(to_roll.1 as u8 + 1));
			},
			Direction::Right => {
				*to_roll = (to_roll.0, Tone::from_u8((to_roll.1 as u8).wrapping_sub(1)));
			},
		};
	}
	
	fn change_color4(&mut self, second: bool, color: Color4) {
		*if second { &mut self.secondary_color4 } else { &mut self.primary_color4 } = color;
	}
	
	fn swap_aries(&mut self) { // Swap secondary and primary
		let temp = self.primary;
		let temp_color = self.primary_color4;
		self.primary = self.secondary;
		self.primary_color4 = self.secondary_color4;
		self.secondary = temp;
		self.secondary_color4 = temp_color;
	}
	
	fn resize(&mut self, size: Vec2) {
		self.dim = size;
		self.pyxels.resize(size.x, vec![(Color::Unset, Tone::Normal); size.y]);
		for v in self.pyxels.iter_mut() {
			v.resize(size.y, (Color::Unset, Tone::Normal));
		}
	}
	
	fn r#move(&mut self, delta: Vec2) {
		self.pos += delta;
	}
	
	fn change_default(&mut self, new: (Color, Tone)) {
		self.r#default = new;
	}
	
	fn change_name(&mut self, name: String) {
		self.name = Some(name);
	}
	
	fn change_location(&mut self, location: String) {
		self.location = Some(location);
	}
	
	fn save(&self, f: Option<Format>) -> Result<(), CanvasError> {
		Ok(())
	}
}