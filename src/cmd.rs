use super::pointer::*;
pub struct Command {
	verb: String,
	hex: [f32;4],
	int: isize,
	rep: usize,
	unset: bool,
}
impl Command {
    pub fn new(verb: String, hex: [f32;4], int: isize, rep: usize, unset: bool) -> Self {
        Command {
            verb,
            hex,
            int,
            rep,
            unset
        }
    }
	pub fn verb(&self) -> &str {
		&self.verb
	}
	pub fn hex(&self) -> [f32;4] {
		self.hex
	}
	pub fn int(&self) -> isize {
		self.int
	}
	pub fn rep(&self) -> usize {
		self.rep
	}
	pub fn unset(&self) -> bool {
		self.unset
	}
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Looper {
	index: usize,
	remaining: usize,
}
impl Looper {
	pub fn index(&self) -> usize {
		self.index
	}
	pub fn remaining(&self) -> usize {
		self.remaining
	}
	pub fn decrease(&mut self) {
		self.remaining -= 1;
	}
}
pub fn run(comms: &Vec<Command>, p: &mut Pointer) -> Vec<([f32;4],[f64;4])> {
	let mut to_draw: Vec<([f32;4],[f64;4])> = vec![];
	let mut i = 0;
	let mut repeat_table: Vec<Looper> = vec!();
	while i < comms.len() {
		let cmd = &comms[i];
		match cmd.verb().as_ref() {
			" " => {
				p.slide(cmd.int(), 0);
			},
			"v" => {
				p.slide(0, cmd.int());
			},
			"s" => {
				if cmd.unset() {
					p.set_virtual_left(0);
				} else {
					p.set_virtual_left(cmd.int());
				}
			},
			"e" => {
				if cmd.unset() {
					p.set_virtual_right(p.width() as isize);
				} else {
					p.set_virtual_right(cmd.int());
				}
			},
			"S" => {
				if cmd.unset() {
					p.set_virtual_top(0);
				} else {
					p.set_virtual_top(cmd.int());
				}
			},
			"E" => {
				if cmd.unset() {
					p.set_virtual_bottom(p.height() as isize);
				} else {
					p.set_virtual_bottom(cmd.int());
				}
			},
			"X" => {
				p.flip_reverse_move_x();

			},
			"Y" => {
				p.flop_reverse_move_y();
			},
			"[" => {
				repeat_table.push(Looper {
					index: i,
					remaining: cmd.rep() - 1,
				});
			},
			"]" => {
				match repeat_table.last() {
					Some(_) => {
						let ilen = repeat_table.len()-1;
						let last = &mut repeat_table[ilen];
						if last.remaining() > 0 {
							i = last.index();
							last.decrease();
						} else {
							repeat_table.remove(ilen);
						}
					}
					_ => (),
				}
			},
			 _  => {
			 	to_draw.push((cmd.hex(), [p.x()*20.0,p.y()*20.0,20.0,20.0]));
			 	p.slide(1,0);
		 	},
		}
		i += 1;
	}
	to_draw
}