use super::canvas::*;
use super::pointer::*;
// The command struct
pub struct Command {
	verb: String,
	hex: [u8;4],
	int: isize,
	rep: usize,
	unset: bool,
}
impl Command {
    pub fn new(verb: String, hex: [u8;4], int: isize, rep: usize, unset: bool) -> Self {
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
// The fact that this struct has all Copy-able primitives and so can have that attribute applied
// Makes me VERY happy C:
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
// Parse each command and run a specific match based on the Command's verb
// Can repeat a block of commands. Yes.
pub fn run(comms: &Vec<Command>, c: &mut Canvas, p: &mut Pointer) {
	let mut i = 0;
	let mut repeat_table: Vec<Looper> = vec!();
	while i < comms.len() {
		let cmd = &comms[i];
		match cmd.verb().as_ref() {
			"r" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"y" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"g" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"c" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"b" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"m" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"w" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"." => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
			"#" => {
				draw_color(cmd.hex(), cmd.rep, c, p);
			},
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
					p.set_virtual_right(c.width());
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
					p.set_virtual_bottom(c.height());
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
			 _  => (),
		}
		i += 1;
	}
}
// Puts color on a Canvas c:
fn draw_color(color: [u8;4], r: usize, c: &mut Canvas, p: &mut Pointer) {
	for _i in 0..r {
		c.put(p, color);
		p.slide(1,0);
	}
}
