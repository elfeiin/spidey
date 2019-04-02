use super::canvas::*;
// Err um WIP
pub fn display_canvas(c: &Canvas) {
	println!();
	let w = c.width() as usize;
	for i in (0..c.buffer().len()).step_by(w) {
		for d in 0..w {
			match c.buffer()[i + d] {
				[255,0,0,0] => print!("r "),
				[255,255,0,0] => print!("y "),
				[0,255,0,0] => print!("g "),
				[0,255,255,0] => print!("c "),
				[0,0,255,0] => print!("b "),
				[255,0,255,0] => print!("m "),
				[255,255,255,0] => print!("r "),
				_ => print!(". "),
			}
		}
		println!();
	}
	println!();
}