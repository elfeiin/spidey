use piston_window::*;
mod screen;
use screen::*;
mod parser;
mod cmd;
mod pointer;
use pointer::*;
mod ui_draw;
mod pyxel;

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Spidey!", [800, 600])
	.exit_on_esc(true)
	.build()
	.unwrap();
	
	let mut curr_screen = 0usize;
	let mut main_pointer = Pointer::new();
	let mut main_parser = parser::Parser::new();
	let mut screen_list = ScreenList::new();
	let mut mouse = Mouse::new();
	
	
	while let Some(e) = window.next() {
		// if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
		// 	mouse.left = true;
		// }
		// if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
		// 	mouse.left = false;
		// }
		// if let Some(a) = e.mouse_relative_args() {
		// }
		if let Some(mut screen) = screen_list.get_screen(curr_screen) {
			if screen.text_changed() {
				let cmds = main_parser.parse(&screen.text()).cmds();
				main_pointer.blank();
				cmd::run(&cmds, &mut main_pointer, &mut screen);
			}
			ui_draw::draw(&mut window, e, &screen);
		} else {
			ui_draw::blank(&mut window, e);
		}
	}
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum UD {
	Up(bool),
	Down(bool),
}
struct Mouse {
	left: UD,
	x: f64,
	y: f64,
	dx: f64,
	dy: f64,
}
impl Mouse {
	pub fn new() -> Mouse {
		Mouse {
			left: UD::Up(false),
			x: 0.0,
			y: 0.0,
			dx: 0.0,
			dy: 0.0,
		}
	}
	pub fn set_left(&mut self, a: bool) {
		if a {
			if self.left() == UD::Down(true) {
				self.left = UD::Down(false);
			} else {
				self.left = UD::Down(true);
			}
		} else {
			if self.left() == UD::Up(true) {
				self.left = UD::Up(false);
			} else {
				self.left = UD::Up(true);
			}
		}
	}
	pub fn left(&self) -> UD {
		self.left
	}
	pub fn set_pos(&mut self, a: (f64, f64)) {
		self.x = a.0;
		self.y = a.1;
	}
	pub fn pos(&self) -> (f64, f64) {
		(self.x,self.y)
	}
	pub fn set_d_pos(&mut self, a: (f64, f64)) {
		self.dx = a.0;
		self.dy = a.1;
	}
	pub fn d_pos(&self) -> (f64, f64) {
		(self.dx,self.dy)
	}
}
struct ScreenList {
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