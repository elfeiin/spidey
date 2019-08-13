use piston_window::*;
mod tab;
use tab::*;
mod parser;
mod cmd;
mod pointer;
use pointer::*;
mod ui_draw;
mod pyxel;
mod mouse;
mod element;

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Spidey!", [800, 600])
		.exit_on_esc(true)
		.build()
		.unwrap();
	
	let mut curr_tab = 0usize;
	let mut main_parser = parser::Parser::new();
	let mut main_pointer = Pointer::new();
	let mut tab_list = tab::TabList::new();
	let mut mouse = mouse::Mouse::new();
	
	
	//This is kinda like my DOM
	let base_ui: Vec<element::Element> = vec![
		element::Element::new(0, "e1".to_string()).with_dim(60.0, 30.0).with_color([0.0, 1.0, 0.0, 1.0]).with_border_sides(Some([1.0,0.0,0.0,1.0])),
	];
	
	while let Some(e) = window.next() {
		// Will be moved to physics.rs eventually. Or maybe input.rs
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
		
		// Draw the tab if it exists, else draw base ui
		if let Some(mut tab) = tab_list.get_tab(curr_tab) {
			// Move the tab around
			if mouse.left() {
				tab.move_origin(mouse.d_pos());
			}
			
			// If anything happened, run the cmd parser
			if tab.text_changed() {
				tab.set_cmds(main_parser.parse(&tab.text()).cmds());
				main_pointer.blank(tab.width(), tab.height());
				cmd::run(&mut main_pointer, &mut tab);
			}
			
			// Draw the big pyxels
			ui_draw::draw(&mut window, &e, &tab);
			// Draw the base ui (menu, buttons, etc)
			ui_draw::draw_ui(&mut window, &e, &base_ui);
		} else {
			// Blank the tab
			ui_draw::blank(&mut window, &e);
			// Draw base ui (still)
			ui_draw::draw_ui(&mut window, &e, &base_ui);
		}
		mouse.reset();
	}
}
