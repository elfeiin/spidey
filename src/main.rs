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
	let mut mouse = Mouse {left: false};
	
	
	while let Some(e) = window.next() {
		if let Some(mut screen) = screen_list.get_screen(curr_screen) {
			if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
				mouse.left = true;
			}
			if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
				mouse.left = false;
			}
			if let Some(a) = e.mouse_relative_args() {
				if mouse.left {
					screen.move_origin(a[0], a[1]);
				}
			}
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

struct Mouse {
	left: bool,
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