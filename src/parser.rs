use super::cmd;
use regex::Regex;
use meval;
enum Tone {
	Light,
	Normal,
	Dark,
}
pub fn parse(text: &String) -> Vec<cmd::Command> {
	let mut comms = vec!();
	let color = Regex::new(r"[rygcbmw.]").unwrap();
	let position = Regex::new(r"[vseSE]").unwrap();
	let words = text.split_whitespace().collect::<Vec<&str>>();
	for c in words.iter() {
		let mut com = "";
		let mut hex: [f32;4] = [0.0;4];
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
				let mut lum: Tone = Tone::Normal;
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
			"X" => {comms.push(cmd::Command::new(c.to_string(),[0.0;4],0,0,true)); continue},
			"Y" => {comms.push(cmd::Command::new(c.to_string(),[0.0;4],0,0,true)); continue},
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
fn get_color(s: &str, l: Tone) -> [f32;4] {
	let mut max: f32 = 1.0;
	let mut min: f32 = 0.0;
	match l {
		Tone::Light => {
			min = 192.0/255.0;
		},
		Tone::Dark => {
			max = 192.0/255.0;
		},
		_ => ()
	}
	match s {
		"r" => [max,min,min,1.0_f32],
		"y" => [max,max,min,1.0_f32],
		"g" => [min,max,min,1.0_f32],
		"c" => [min,max,max,1.0_f32],
		"b" => [min,min,max,1.0_f32],
		"m" => [max,min,max,1.0_f32],
		"w" => [max,max,max,1.0_f32],
		 _  => [min,min,min,1.0_f32],
	}
}
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
fn parse_hex(s: &str) -> ([f32;4],bool) {
	let mut r: f32 = 1.0;
	let mut g: f32 = 1.0;
	let mut b: f32 = 1.0;
	let mut a: f32 = 1.0;
	let mut b00 = true;
	match s.len() {
		3 => {
			match u8::from_str_radix(&format!("{}{}",&s[ ..1],&s[ ..1]),16) {
				Ok(n) => {
					r = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&format!("{}{}",&s[1..2],&s[1..2]),16) {
				Ok(n) => {
					g = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&format!("{}{}",&s[2.. ],&s[2.. ]),16) {
				Ok(n) => {
					b = n as f32/255.0;
				},
				_ => ()
			};
			b00 = false;
		},
		6 => {
			match u8::from_str_radix(&s[ ..2],16) {
				Ok(n) => {
					r = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&s[2..4],16) {
				Ok(n) => {
					g = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&s[4.. ],16) {
				Ok(n) => {
					b = n as f32/255.0;
				},
				_ => ()
			};
			b00 = false;
		},
		4 => {
			match u8::from_str_radix(&format!("{}{}",&s[ ..1],&s[ ..1]),16) {
				Ok(n) => {
					r = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&format!("{}{}",&s[1..2],&s[1..2]),16) {
				Ok(n) => {
					g = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&format!("{}{}",&s[2..3],&s[2..3]),16) {
				Ok(n) => {
					b = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&format!("{}{}",&s[3.. ],&s[3.. ]),16) {
				Ok(n) => {
					a = n as f32/255.0;
				},
				_ => ()
			};
			b00 = false;
		},
		8 => {
			match u8::from_str_radix(&s[ ..2],16) {
				Ok(n) => {
					r = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&s[2..4],16) {
				Ok(n) => {
					g = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&s[4..6],16) {
				Ok(n) => {
					b = n as f32/255.0;
				},
				_ => ()
			};
			match u8::from_str_radix(&s[6.. ],16) {
				Ok(n) => {
					a = n as f32/255.0;
				},
				_ => ()
			};
			b00 = false;
		},
		_ => (),
	}
	([r,g,b,a],b00)
}