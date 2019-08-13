use super::pyxel::*;
use super::cmd::*;

// The Tab struct.
pub struct Tab {
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

impl Tab {
	
	pub fn new(w: isize, h: isize) -> Tab {
		Tab {
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
	
	// Return respective values
	pub fn pyxels(&self) -> &Vec<Pyxel> { &self.pyxels }
	
	pub fn dark(&self) -> bool { self.dark }
	
	pub fn pyxel_scale(&self) -> f64 { self.pyxel_scale }
	
	pub fn origin(&self) -> (f64,f64) { self.origin }
	
	pub fn text(&self) -> String { self.text.0.to_owned() }
	
	pub fn width(&self) -> f64 { self.width }
	
	pub fn height(&self) -> f64 { self.height }
	
	// Set respective values
	pub fn add_pyxel(&mut self, a: Pyxel) {
		self.pyxels.push(a);
	}
	
	pub fn set_dark(&mut self, b: bool) {
		self.dark = b;
	}
	
	pub fn set_pyxel_scale(&mut self, a: f64) {
		self.pyxel_scale = a;
	}
	
	pub fn move_origin(&mut self, a: (f64, f64)) {
		self.origin.0 += a.0;
		self.origin.1 += a.1;
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
	
	pub fn set_height(&mut self, h: isize) {
		let mut h = h;
		if h == 0 {
			h = 1;
		}
		self.height = h as f64;
	}
	
}

// A list of Tabs
pub struct TabList {
	tabs: Vec<Tab>,
}

impl TabList {

	pub fn new() -> TabList {
		TabList {
			tabs: Vec::new(),
		}
	}
	
	pub fn get_tab(&mut self, i: usize) -> Option<&mut Tab> {
		if self.tabs.len() > i {
			Some(&mut self.tabs[i])
		} else {
			None
		}
	}
	
	pub fn add_tab(&mut self, w: isize, h: isize) {
		self.tabs.push(Tab::new(w,h));
	}
	
	pub fn close_tab(&mut self, tab: usize) {
		self.tabs.remove(tab);
	}
	
}