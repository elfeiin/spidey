use super::pointer::*;
use super::pyxel::*;
use super::Screen;
#[derive(Debug, Clone)]
pub struct Command {
	verb: char,
	hex: [u8;4],
	int: isize,
	rep: usize,
	unset: bool,
}
impl Command {
    pub fn new(verb: char, hex: [u8;4], int: isize, rep: usize, unset: bool) -> Self {
        Command {
            verb,
            hex,
            int,
            rep,
            unset
        }
    }
	pub fn verb(&self) -> &char {
		&self.verb
	}
	pub fn hex(&self) -> [u8;4] {
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
pub fn run(comms: &Vec<Command>, pointer: &mut Pointer, screen: &mut Screen) {
	let mut i = 0;
	let mut repeat_table: Vec<Looper> = vec!();
	while i < comms.len() {
		let cmd = &comms[i];
		match cmd.verb() {
			' ' => {
				pointer.slide(cmd.int(), 0);
			},
			'v' => {
				pointer.slide(0, cmd.int());
			},
			's' => {
				if cmd.unset() {
					pointer.set_virtual_left(0);
				} else {
					pointer.set_virtual_left(cmd.int());
				}
			},
			'e' => {
				if cmd.unset() {
					pointer.set_virtual_right(pointer.width() as isize);
				} else {
					pointer.set_virtual_right(cmd.int());
				}
			},
			'S' => {
				if cmd.unset() {
					pointer.set_virtual_top(0);
				} else {
					pointer.set_virtual_top(cmd.int());
				}
			},
			'E' => {
				if cmd.unset() {
					pointer.set_virtual_bottom(pointer.height() as isize);
				} else {
					pointer.set_virtual_bottom(cmd.int());
				}
			},
			'X' => {
				pointer.flip_reverse_move_x();

			},
			'Y' => {
				pointer.flop_reverse_move_y();
			},
			'[' => {
				repeat_table.push(Looper {
					index: i,
					remaining: cmd.rep() - 1,
				});
			},
			']' => {
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
			 '#'  => {
				let mut k = 0;
				while k < cmd.rep() {
				 	screen.add_pyxel(Pyxel::new(cmd.hex(),pointer.x()*20.0,pointer.y()*20.0));
				 	pointer.slide(1,0);
				 	k+=1;
			 	}
		 	},
		 	_ => (),
		}
		i += 1;
	}
}