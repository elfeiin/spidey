// Needs to be able to save the canvas, in whatever form
use std::io;
mod canvas;
mod cmd;
mod pointer;
mod parser;
mod display;
fn main() {
	// The canvas we will draw on
	let mut main_canvas = canvas::Canvas::new(4, 4);
	// The 2d pointer we will use to draw on the canvas
	let mut main_pointer = pointer::Pointer::new(&main_canvas);
	loop {
		// Uses terminal input for now
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("failed to read line");
		input = input.trim().to_string();
		if (input == "q") || (input ==  "qq") || (input ==  "quit") || (input ==  "exit") {
			break
		}
		// Modify canvas size (terminal input)
		if input.len() > 4 {
			let split = input.split(" ").collect::<Vec<&str>>();
			let num = parser::parse_num(&split[1]).0;
			match split[0] {
				"width" => {main_canvas.set_width(&mut main_pointer,num);continue},
				"height" => {main_canvas.set_height(&mut main_pointer,num);continue},
				_ => ()
			}
		}
		// Reinitialize the canvas buffer to black ([0_u8;4])
		main_canvas.blank();
		// Reinitialize the pointer: Good as new()!
		main_pointer.blank(&main_canvas);
		// parser::parse_text() returns a vec of type cmd::Command
		let mut commands = parser::parse_text(&input);
		// Bring it all together and run the commands
		cmd::run(&mut commands, &mut main_canvas, &mut main_pointer);
		// Display an abbreviation in terminal
		display::display_canvas(&main_canvas);
	}
}