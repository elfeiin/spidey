use meval;
use regex;
use regex::Regex;
use super::cmd;
// Parses all numbers, with defaults if errors.
// The bool tells whether or not a value was actually set during this process.
pub fn parse_num(s: &str) -> (usize,isize,bool) {
	let mut u: usize = 1;
	let mut i: isize = 0;
	let mut b = true;
	match meval::eval_str(s) {
		Ok(n) => {u = n as usize; i = n as isize; b = false;},
		_ => ()
	}
	(u,i,b)
}
// Converts a hex string into [rgba] data (I love read that, [rgba] data)
// Works like CSS, kinda
fn parse_hex(s: &str) -> ([u8;4],bool) {
	match s.len() {
		3 => {
			let r = match u8::from_str_radix(&format!("{}{}",&s[ ..1],&s[ ..1]),16) {Ok(n) => n, _ => 0_u8};
			let g = match u8::from_str_radix(&format!("{}{}",&s[1..2],&s[1..2]),16) {Ok(n) => n, _ => 0_u8};
			let b = match u8::from_str_radix(&format!("{}{}",&s[2.. ],&s[2.. ]),16) {Ok(n) => n, _ => 0_u8};
			let a = 0u8;
			([r,g,b,a],false)
		},
		6 => {
			let r = match u8::from_str_radix(&s[ ..2],16) {Ok(n) => n, _ => 0_u8};
			let g = match u8::from_str_radix(&s[2..4],16) {Ok(n) => n, _ => 0_u8};
			let b = match u8::from_str_radix(&s[4.. ],16) {Ok(n) => n, _ => 0_u8};
			let a = 0u8;
			([r,g,b,a],false)
		},
		4 => {
			let r = match u8::from_str_radix(&format!("{}{}",&s[ ..1],&s[ ..1]),16) {Ok(n) => n, _ => 0_u8};
			let g = match u8::from_str_radix(&format!("{}{}",&s[1..2],&s[1..2]),16) {Ok(n) => n, _ => 0_u8};
			let b = match u8::from_str_radix(&format!("{}{}",&s[2..3],&s[2..3]),16) {Ok(n) => n, _ => 0_u8};
			let a = match u8::from_str_radix(&format!("{}{}",&s[3.. ],&s[3.. ]),16) {Ok(n) => n, _ => 0_u8};
			([r,g,b,a],false)
		},
		8 => {
			let r = match u8::from_str_radix(&s[ ..2],16) {Ok(n) => n, _ => 0_u8};
			let g = match u8::from_str_radix(&s[2..4],16) {Ok(n) => n, _ => 0_u8};
			let b = match u8::from_str_radix(&s[4..6],16) {Ok(n) => n, _ => 0_u8};
			let a = match u8::from_str_radix(&s[6.. ],16) {Ok(n) => n, _ => 0_u8};
			([r,g,b,a],false)
		},
		_ => ([0;4],true),
	}
}
enum Tone {
	Light,
	Mid,
	Dark,
}
// Takes a str as input and returns the appropriate [rgba] data
// Also takes l which is an enum Tone field
fn get_color(s: &str, l: Tone) -> [u8;4] {
	let mut max = 255;
	let mut min = 000;
	match l {
		Tone::Light => {
			min = 192;
		},
		Tone::Dark => {
			max = 192;
		},
		_ => ()
	}
	match s {
		"r" => [max,min,min,0],
		"y" => [max,max,min,0],
		"g" => [min,max,min,0],
		"c" => [min,max,max,0],
		"b" => [min,min,max,0],
		"m" => [max,min,max,0],
		"w" => [max,max,max,0],
		 _  => [min,min,min,0],
	}
}
// Parses a string and returns a vec of Commands
// I'm not that good at making parsers, lol. I realize this is a mess. It's my mess.... c:
// *Consider making each match arm return its own command and giving each match arm its own arguments
// to modify and whatnot
pub fn parse_text(text: &String) -> Vec<cmd::Command> {
	let mut comms = vec!();
	let color = Regex::new(r"[rygcbmw.]").unwrap();
	let position = Regex::new(r"[vseSE]").unwrap();
	let words = text.split_whitespace().collect::<Vec<&str>>();
	for c in words.iter() {
		let mut com = "";
		let mut hex: [u8;4] = [0;4];
		let mut int: isize = 0;
		let mut rep: usize = 1;
		let mut unset = true;
		let mut can_be_num = true;
		match c.rfind("[") {
			Some(_) => {
				can_be_num = false;
				let the_split = c.split("[").collect::<Vec<&str>>();
				rep = match parse_num(&the_split[0]) {
					(n,_,b) => { unset = unset || b; n }
				};
				com = "[";
			},
			_ => (),
		}
		match c.rfind("]") {
			Some(_) => {
				can_be_num = false;
				com = "]"
			},
			_ => (),
		}
		match c.rfind("#") {
			Some(_) => {
				can_be_num = false;
				let the_split = c.split("#").collect::<Vec<&str>>();
				rep = match parse_num(&the_split[0]) {
					(n,_,b) => { unset = unset || b; n }
				};
				hex = match parse_hex(&the_split[1]) {
					(n,b) => { unset = unset || b; n }
				};
				com = "#";
			},
			_ => (),
		}
		match color.find(c) {
			Some(l) => {
				can_be_num = false;
				let l = l.as_str();
				let mut the_split = c.split(l).collect::<Vec<&str>>();
				let mut lum: Tone = Tone::Mid;
				match the_split[0].rfind("l") {
					Some(_) => {
						the_split = the_split[..the_split.len()-1].to_vec();
						lum = Tone::Light;
					},
					_ => (),
				}
				match the_split[0].rfind("d") {
					Some(_) => {
						the_split = the_split[..the_split.len()-1].to_vec();
						lum = Tone::Dark;
					},
					_ => (),
				}
				rep = match parse_num(&the_split[0]) {
					(n,_,b) => { unset = unset || b; n }
				};
				hex = get_color(l,lum);
				com = l;
			},
			_ => ()
		}
		match position.find(c) {
			Some(l) => {
				can_be_num = false;
				let l = l.as_str();
				let the_split = c.split(l).collect::<Vec<&str>>();
				int = match parse_num(&the_split[1]) {
					(_,n,b) => { unset = unset || b; n }
				};
				com = l;
			},
			_ => ()
		}
		match c.as_ref() {
			"x" => {comms.push(cmd::Command::new(c.to_string(),[0;4],0,0,true)); continue},
			"y" => {comms.push(cmd::Command::new(c.to_string(),[0;4],0,0,true)); continue},
			_ => ()
		}
		if can_be_num {
			com = " ";
			int = parse_num(c).1;
		}
		let com = cmd::Command::new(com.to_string(),hex,int,rep,unset);
		comms.push(com);
	}
	comms
}
