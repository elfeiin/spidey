use piston_window::*;
mod parser;
mod cmd;
mod pointer;
fn main() {
	let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
	let mut pointer = pointer::Pointer::new(16,16);
	let mut previous_input = String::new();
	let mut to_draw: Vec<([f32;4],[f64;4])> = vec!();
	while let Some(e) = window.next() {
		let input = String::from("7[ r b g ]");
		if input != previous_input {
			to_draw = cmd::run(&mut parser::parse(&input), &mut pointer);
		}
		pointer.blank();
		window.draw_2d(&e, |c, g| {
			clear([1.0; 4], g);
			for (color, pos) in to_draw.iter() {
				rectangle(*color, *pos, c.transform, g);
			}
		});
		previous_input = input;
	}
}