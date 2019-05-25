use super::pyxel::*;
use super::cmd::*;
pub struct Screen {
	pyxels: Vec<Pyxel>,
	dark: bool,
	pyxel_scale: f64,
	origin: (f64,f64),
	text_changed: bool,
	text: (String,bool),
	cmds: Vec<Command>,
	width: f64,
	height: f64,
}
impl Screen {
	pub fn new(w: isize, h: isize) -> Screen {
		Screen {
			pyxels: vec!(),
			dark: false,
			pyxel_scale: 20.0,
			origin: (0.0,0.0),
			text_changed: false,
			text: (String::new(),false),
			cmds: Vec::new(),
			width: w as f64,
			height: h as f64,
		}
	}
	pub fn add_pyxel(&mut self, a: Pyxel) {
		self.pyxels.push(a);
	}
	pub fn pyxels(&self) -> &Vec<Pyxel> {
		&self.pyxels
	}
	pub fn set_dark(&mut self, b: bool) {
		self.dark = b;
	}
	pub fn dark(&self) -> bool {
		self.dark
	}
	pub fn set_pyxel_scale(&mut self, a: f64) {
		self.pyxel_scale = a;
	}
	pub fn pyxel_scale(&self) -> f64 {
		self.pyxel_scale
	}
	pub fn move_origin(&mut self, a: (f64, f64)) {
		self.origin.0 += a.0;
		self.origin.1 += a.1;
	}
	pub fn origin(&self) -> (f64,f64) {
		self.origin
	}
	pub fn set_text(&mut self, a: String) {
		self.text_changed = true;
		self.text.0 = a;
	}
	pub fn text_changed(&mut self) -> bool {
		if self.text_changed {
			self.text_changed = false;
			true
		} else {
			false
		}
	}
	pub fn text(&self) -> String {
		self.text.0.to_owned()
	}
	pub fn set_cmds(&mut self, a: Vec<Command>) {
		self.cmds = a;
	}
	pub fn cmds(&self) -> Vec<Command> {
		self.cmds.to_vec()
	}
	pub fn set_width(&mut self, w: isize) {
		let mut w = w;
		if w == 0 {
			w = 1;
		}
		self.width = w as f64;
	}
	pub fn width(&self) -> f64 {
		self.width
	}
	pub fn set_height(&mut self, h: isize) {
		let mut h = h;
		if h == 0 {
			h = 1;
		}
		self.height = h as f64;
	}
	pub fn height(&self) -> f64 {
		self.height
	}
}
pub struct ScreenList {
	screens: Vec<Screen>,
}
impl ScreenList {
	pub fn new() -> ScreenList {
		ScreenList {
			screens: Vec::new(),
		}
	}
	pub fn get_screen(&mut self, i: usize) -> Option<&mut Screen> {
		if self.screens.len() > i {
			Some(&mut self.screens[i])
		} else {
			None
		}
	}
	pub fn add_screen(&mut self, w: isize, h: isize) {
		self.screens.push(Screen::new(w,h));
	}
	pub fn close_screen(&mut self, screen: usize) {
		self.screens.remove(screen);
	}
}