use super::canvas::*;
// Err um WIP
pub fn display_canvas(c: &Canvas) {
	for y in c.buffer.iter() {
		for x in y.iter() {
			match x {
				[255,0,0,0] => print!("r "),
				[255,255,0,0] => print!("y "),
				[0,255,0,0] => print!("g "),
				[0,255,255,0] => print!("c "),
				[0,0,255,0] => print!("b "),
				[255,0,255,0] => print!("m "),
				[255,255,255,0] => print!("r "),
				_ => print!("."),
			}
		}
		println!();
	}
}