// Needs to be able to save the canvas, in whatever form
use std::io;
mod canvas;
mod cmd;
mod pointer;
mod parser;

enum Message {
	Close,
	Draw(Vec<cmd::Command>),
	Width(usize),
	Height(usize),
	Continue,
}

fn main() {
	let mut main_canvas = canvas::Canvas::new(4, 4);
	let mut main_pointer = pointer::Pointer::new(&main_canvas);
	loop {
		let mut input = String::new();
		
		io::stdin().read_line(&mut input).expect("failed to read line");
		input = input.trim().to_string();
		match handle_input(input) {
			Message::Close => break,
			Message::Draw(mut commands) => {
				main_canvas.blank();
				main_pointer.blank(&main_canvas);
				cmd::run(&mut commands, &mut main_canvas, &mut main_pointer);
			},
			Message::Width(n) => {
				main_canvas.set_width(&mut main_pointer, n);
			},
			Message::Height(n) => {
				main_canvas.set_height(&mut main_pointer, n);
			},
			Message::Continue => (),
		}
		draw(&main_canvas);
	}
}

fn handle_input(i: String) -> Message {
	let split = i.split(" ").collect::<Vec<&str>>();
	let n = parser::parse_num(&split[1]).0;
	match split[0] {
		"q" => Message::Close,
		"qq" => Message::Close,
		"quit" => Message::Close,
		"exit" => Message::Close,
		"width" => Message::Width(n),
		"height" => Message::Height(n),
		"" => Message::Continue,
		" " => Message::Continue,
		"continue" => Message::Continue,
		_ => Message::Draw(parser::parse_text(&i)),
	}
}

fn draw(c: &canvas::Canvas) {
	
}