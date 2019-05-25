use piston_window::*;
mod screen;
use screen::*;
mod parser;
mod cmd;
mod pointer;
use pointer::*;
mod ui_draw;
mod pyxel;
mod mouse;

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Spidey!", [800, 600])
	.exit_on_esc(true)
	.build()
	.unwrap();
	
	let mut curr_screen = 0usize;
	let mut main_parser = parser::Parser::new();
	let mut main_pointer = Pointer::new();
	let mut screen_list = screen::ScreenList::new();
	let mut mouse = mouse::Mouse::new();
	
	screen_list.add_screen(21,13);
	
	if let Some(mut screen) = screen_list.get_screen(curr_screen) {
		screen.set_text("7b14r7b\n7b14r7b\n7b14r7b\n7b14r\n21r\n21r\n21r".to_string());
	}
	
	while let Some(e) = window.next() {
		if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
			mouse.set_left(true);
		}
		if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
			mouse.set_left(false);
		}
		if let Some(a) = e.mouse_cursor_args() {
			mouse.set_pos((a[0],a[1]));
		}
		if let Some(a) = e.mouse_relative_args() {
			mouse.set_d_pos((a[0],a[1]));
		}
		if let Some(mut screen) = screen_list.get_screen(curr_screen) {
			if mouse.left() {
				screen.move_origin(mouse.d_pos());
			}
			if screen.text_changed() {
				screen.set_cmds(main_parser.parse(&screen.text()).cmds());
				main_pointer.blank(screen.width(), screen.height());
				cmd::run(&mut main_pointer, &mut screen);
			}
			ui_draw::draw(&mut window, e, &screen);
		} else {
			ui_draw::blank(&mut window, e);
		}
		mouse.reset();
	}
}