use super::cmd::*;
use meval;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tone {
	Light,
	Normal,
	Dark,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
	Nope,
	Line,
	Mult,
}

#[derive(Debug, Clone)]
pub struct Parser {
	cmd: char,
	hex: String,
	num: String,
	sharps: bool,
	tone: Tone,
	comment: Comment,
	cmds: Vec<Command>,
}

impl Parser {
	
	pub fn new() -> Parser {
		Parser {
			cmd: char::from(0u8),
			hex: String::new(),
			num: String::new(),
			sharps: false,
			tone: Tone::Normal,
			comment: Comment::Nope,
			cmds: vec!(),
		}
	}
	
	pub fn reset(&mut self) {
		self.cmd = char::from(0u8);
		self.hex = String::new();
		self.num = String::new();
		self.sharps = false;
		self.tone = Tone::Normal;
		self.comment = Comment::Nope;
	}
	
	pub fn set_cmd(&mut self, a: char) {
		self.cmd = a;
	}
	
	pub fn set_hex(&mut self, a: String) {
		self.hex = a;
	}
	
	pub fn set_num(&mut self, a: String) {
		self.num = a;
	}
	
	pub fn push_hex(&mut self, a: char) {
		self.hex.push(a);
	}
	
	pub fn push_num(&mut self, a: char) {
		self.num.push(a);
	}
	
	pub fn set_sharps(&mut self, a: bool) {
		self.sharps = a;
	}
	
	pub fn set_tone(&mut self, a: Tone) {
		self.tone = a;
	}
	
	pub fn set_comment(&mut self, a: Comment) {
		self.comment = a;
	}
	
	pub fn cmd(&self) -> char {
		self.cmd
	}
	
	pub fn hex(&self) -> String {
		self.hex.to_owned()
	}
	
	pub fn num(&self) -> String {
		self.num.to_owned()
	}
	
	pub fn sharps(&self) -> bool {
		self.sharps
	}
	
	pub fn tone(&self) -> Tone {
		self.tone.to_owned()
	}
	
	pub fn comment(&self) -> Comment {
		self.comment.to_owned()
	}
	
	pub fn cmds(&self) -> Vec<Command> {
		self.cmds.to_owned()
	}
	
	pub fn put(&mut self) {
		if self.cmd() != char::from(0u8) {
			let int = parse_num(self.num()).0;
			let rep = parse_num(self.num()).1;
			if self.sharps {
				self.cmds.push(Command::new(
					self.cmd(),
					parse_hex(self.hex()),
					int,
					rep,
					self.hex() == String::new() && self.num() == String::new()
				));
			} else {
				self.cmds.push(Command::new(
					self.cmd(),
					tone(self.hex(), self.tone()),
					int,
					rep,
					self.hex() == String::new() && self.num() == String::new()
				));
			}
		}
	}
	
	pub fn parse(&mut self, s: &String) -> &Self {
		self.cmds = vec!();
		self.reset();
		let num_list: String = String::from("0123456789-+/*");
		let hex_list: String = String::from("0123456789abcdef");
		let color_list: String = String::from("rgbcymw.");
		let control_list = String::from("v>[]esESXY");
		let mut chars = s.chars();
		while let Some(c) = chars.next() {
			match c {
				'|' => {
					if self.comment() == Comment::Nope {
						self.set_comment(Comment::Line);
					}
					self.reset();
				},
				'{' => {
					if self.comment() == Comment::Nope {
						self.set_comment(Comment::Mult);
					}
					self.reset();
				},
				'}' => {
					self.set_comment(Comment::Nope);
					self.reset();
				},
				' ' => {
					self.put();
					self.reset();
				},
				'\n' => {
					if self.comment() == Comment::Nope {
						self.set_cmd('n');
						self.put();
					} else {
						if self.comment() == Comment::Line {
							self.set_comment(Comment::Nope);
						}
					}
					self.reset();
				},
				_ => (),
			}
			if self.comment == Comment::Nope {
				if self.sharps() {
					if let Some(_) = hex_list.find(c) {
						self.push_hex(c);
						continue;
					}
				} else {
					if let Some(_) = num_list.find(c) {
						self.push_num(c);
						continue;
					}
				}
				match c {
					'l' => {
						self.set_tone(Tone::Light);
					},
					'd' => {
						self.set_tone(Tone::Dark);
					},
					'#' => {
						self.set_cmd('#');
						self.set_sharps(true);
					},
					_ => (),
				}
				if let Some(_) = color_list.find(c) {
					self.set_cmd('#');
					self.set_hex(c.to_string());
					self.put();
					self.reset();
				}
				if let Some(_) = control_list.find(c) {
					self.set_cmd(c);
					self.put();
					self.reset();
				}
			}
		}
		self.put();
		self.reset();
		self
	}
	
}

// Convert a string containing a valid hex value to an array of u8
fn parse_hex(s: String) -> [u8;4] {

	let mut r: u8 = 255;
	let mut g: u8 = 255;
	let mut b: u8 = 255;
	let mut a: u8 = 255;
	
	// Uhhh.....
	match s.len() {
		3=>{match u8::from_str_radix(&format!("{}{}",&s[ ..1],&s[ ..1]),16) {
				Ok(n) => {r = n;},_ => ()};
			match u8::from_str_radix(&format!("{}{}",&s[1..2],&s[1..2]),16) {
				Ok(n) => {g = n;},_ => ()};
			match u8::from_str_radix(&format!("{}{}",&s[2.. ],&s[2.. ]),16) {
				Ok(n) => {b = n;},_ => ()};},
		6=>{match u8::from_str_radix(&s[ ..2],16) {
				Ok(n) => {r = n;},_ => ()};
			match u8::from_str_radix(&s[2..4],16) {
				Ok(n) => {g = n;},_ => ()};
			match u8::from_str_radix(&s[4.. ],16) {
				Ok(n) => {b = n;},_ => ()};},
		4=>{match u8::from_str_radix(&format!("{}{}",&s[ ..1],&s[ ..1]),16) {
				Ok(n) => {r = n;},_ => ()};
			match u8::from_str_radix(&format!("{}{}",&s[1..2],&s[1..2]),16) {
				Ok(n) => {g = n;},_ => ()};
			match u8::from_str_radix(&format!("{}{}",&s[2..3],&s[2..3]),16) {
				Ok(n) => {b = n;},_ => ()};
			match u8::from_str_radix(&format!("{}{}",&s[3.. ],&s[3.. ]),16) {
				Ok(n) => {a = n;},_ => ()};},
		8=>{match u8::from_str_radix(&s[ ..2],16) {
				Ok(n) => {r = n;},_ => ()};
			match u8::from_str_radix(&s[2..4],16) {
				Ok(n) => {g = n;},_ => ()};
			match u8::from_str_radix(&s[4..6],16) {
				Ok(n) => {b = n;},_ => ()};
			match u8::from_str_radix(&s[6.. ],16) {
				Ok(n) => {a = n;},_ => ()};},
		_ => (),
	}
	
	[r,g,b,a] // Return all our variables, formatted in an array
	
}

// Returns a tuple containing the conversion of a string to an isize and the same string to a usize, respectively
pub fn parse_num(s: String) -> (isize,usize) {

	let mut i: isize = 0;
	let mut u: usize = 1;
	match meval::eval_str(s) {
		Ok(n) => {i = n as isize; u = n.abs() as usize;},
		_ => ()
	}
	
	(i,u)
	
}

// Makes a color darker or lighter or does nothing to it
fn tone(s: String, l: Tone) -> [u8;4] {

	let mut max: u8 = 255;
	let mut min: u8 = 0;
	
	match l {
		Tone::Light => {
			min = 192;
		},
		Tone::Dark => {
			max = 192;
		},
		_ => ()
	}
	
	match s.as_ref() {
		"r" => [max,min,min,255],
		"y" => [max,max,min,255],
		"g" => [min,max,min,255],
		"c" => [min,max,max,255],
		"b" => [min,min,max,255],
		"m" => [max,min,max,255],
		"w" => [max,max,max,255],
		 _  => [min,min,min,255],
	}
	
}


// Deperecated. TODO: Delete this

// pub fn parse(text: &String) -> Vec<cmd::Command> {
// 	let mut comms = vec!();
// 	let color = Regex::new(r'[rygcbmw.]').unwrap();
// 	let position = Regex::new(r'[vseSE]').unwrap();
// 	let words = text.split_whitespace().collect::<Vec<&str>>();
// 	for c in words.iter() {
// 		let mut com = '';
// 		let mut hex: [f32;4] = [0.0;4];
// 		let mut int: isize = 0;
// 		let mut rep: usize = 1;
// 		let mut unset = true;
// 		let mut can_be_num = true;
// 		match c.rfind('[') {
// 			Some(_) => {
// 				can_be_num = false;
// 				let the_split = c.split('[').collect::<Vec<&str>>();
// 				rep = match parse_num(&the_split[0]) {
// 					(n,_,b) => { unset = unset || b; n }
// 				};
// 				com = '[';
// 			},
// 			_ => (),
// 		}
// 		match c.rfind(']') {
// 			Some(_) => {
// 				can_be_num = false;
// 				com = ']'
// 			},
// 			_ => (),
// 		}
// 		match c.rfind('#') {
// 			Some(_) => {
// 				can_be_num = false;
// 				let the_split = c.split('#').collect::<Vec<&str>>();
// 				rep = match parse_num(&the_split[0]) {
// 					(n,_,b) => { unset = unset || b; n }
// 				};
// 				hex = match parse_hex(&the_split[1]) {
// 					(n,b) => { unset = unset || b; n }
// 				};
// 				com = '#';
// 			},
// 			_ => (),
// 		}
// 		match color.find(c) {
// 			Some(l) => {
// 				can_be_num = false;
// 				let l = l.as_str();
// 				let mut the_split = c.split(l).collect::<Vec<&str>>();
// 				let mut lum: Tone = Tone::Normal;
// 				match the_split[0].rfind('l') {
// 					Some(_) => {
// 						the_split = the_split[..the_split.len()-1].to_vec();
// 						lum = Tone::Light;
// 					},
// 					_ => (),
// 				}
// 				match the_split[0].rfind('d') {
// 					Some(_) => {
// 						the_split = the_split[..the_split.len()-1].to_vec();
// 						lum = Tone::Dark;
// 					},
// 					_ => (),
// 				}
// 				rep = match parse_num(&the_split[0]) {
// 					(n,_,b) => { unset = unset || b; n }
// 				};
// 				hex = tone(l,lum);
// 				com = l;
// 			},
// 			_ => ()
// 		}
// 		match position.find(c) {
// 			Some(l) => {
// 				can_be_num = false;
// 				let l = l.as_str();
// 				let the_split = c.split(l).collect::<Vec<&str>>();
// 				int = match parse_num(&the_split[1]) {
// 					(_,n,b) => { unset = unset || b; n }
// 				};
// 				com = l;
// 			},
// 			_ => ()
// 		}
// 		match c.as_ref() {
// 			'X' => {comms.push(cmd::Command::new(c,[0.0;4],0,0,true)); continue},
// 			'Y' => {comms.push(cmd::Command::new(c,[0.0;4],0,0,true)); continue},
// 			_ => ()
// 		}
// 		if can_be_num {
// 			com = ' ';
// 			int = parse_num(c).1;
// 		}
// 		let com = cmd::Command::new(com,hex,int,rep,unset);
// 		comms.push(com);
// 	}
// 	comms
// }